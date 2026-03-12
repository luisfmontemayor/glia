use std::time::Duration;
use crossbeam_channel::{bounded, Sender, Receiver};
use once_cell::sync::Lazy;
use std::thread;
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::Mutex;

struct TelemetryMessage {
    payload: String,
    url: String,
    timeout_sec: f64,
}

pub struct FlushSummary {
    pub failed_jobs: usize,
    pub common_errors: Vec<(String, usize)>,
}

struct Stats {
    failed_count: AtomicUsize,
    error_frequency: Mutex<HashMap<String, usize>>,
}

static STATS: Lazy<Stats> = Lazy::new(|| Stats {
    failed_count: AtomicUsize::new(0),
    error_frequency: Mutex::new(HashMap::new()),
});

static CHANNEL: Lazy<(Sender<TelemetryMessage>, Receiver<TelemetryMessage>)> = Lazy::new(|| {
    let limit_str = env::var("GLIA_LOCAL_QUEUE_LIMIT")
        .expect("FATAL: GLIA_LOCAL_QUEUE_LIMIT environment variable must be set");
    let limit = limit_str.parse::<usize>()
        .expect("FATAL: GLIA_LOCAL_QUEUE_LIMIT must be a valid positive integer");
    
    let (s, r) = bounded(limit);
    spawn_worker(r.clone());
    (s, r)
});

fn spawn_worker(receiver: Receiver<TelemetryMessage>) {
    thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime");

        rt.block_on(async move {
            let client = reqwest::Client::new();
            let debug_mode = env::var("GLIA_DEBUG").is_ok();
            
            while let Ok(msg) = receiver.recv() {
                let res = client.post(&msg.url)
                    .header("Content-Type", "application/json")
                    .timeout(Duration::from_secs_f64(msg.timeout_sec))
                    .body(msg.payload)
                    .send()
                    .await;

                let error_msg = match res {
                    Ok(resp) if resp.status().is_success() => None,
                    Ok(resp) => Some(format!("HTTP {}", resp.status())),
                    Err(e) => Some(e.to_string()),
                };

                if let Some(err) = error_msg {
                    STATS.failed_count.fetch_add(1, Ordering::SeqCst);
                    let mut freq = STATS.error_frequency.lock().unwrap();
                    *freq.entry(err.clone()).or_insert(0) += 1;
                    
                    if debug_mode {
                        eprintln!("[GLIA DEBUG] Telemetry push failed: {}", err);
                    }
                }
            }
        });
    });
}

/// TODO: rename to something that reflects that this Enqueues the payload to a background worker.
pub fn push_telemetry(json_payload: &str, url: &str, timeout_sec: f64) -> Result<(), String> {
    let (sender, _) = &*CHANNEL;
    sender.send(TelemetryMessage {
        payload: json_payload.to_string(),
        url: url.to_string(),
        timeout_sec,
    }).map_err(|e| e.to_string())
}

/// Blocks until all enqueued telemetry messages have been processed.
pub fn perform_flush() -> FlushSummary {
    let (sender, _) = &*CHANNEL;
    while !sender.is_empty() {
        thread::sleep(Duration::from_millis(10));
    }

    let failed_jobs = STATS.failed_count.swap(0, Ordering::SeqCst);
    let mut freq_map = STATS.error_frequency.lock().unwrap();
    let common_errors: Vec<_> = freq_map.drain().collect();

    FlushSummary {
        failed_jobs,
        common_errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 1. Success path: ensure payload reaches endpoint and returns 200 OK
    fn test_push_telemetry_success() {
        env::set_var("GLIA_LOCAL_QUEUE_LIMIT", "1000");
        
        let mut server = mockito::Server::new();
        let url = format!("{}/ingest", server.url());

        let mock = server.mock("POST", "/ingest")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"status\": \"success\"}")
            .create();

        let payload = r#"{"run_id": "123", "cpu_percent": 50.0}"#;
        let result = push_telemetry(payload, &url, 1.0);

        assert!(result.is_ok());
        let summary = perform_flush();
        assert_eq!(summary.failed_jobs, 0);
        mock.assert();
    }

    #[test]
    // 2. Server failure path: ensure function returns Ok even if server responds with 500
    fn test_push_telemetry_server_error() {
        env::set_var("GLIA_LOCAL_QUEUE_LIMIT", "1000");
        
        let mut server = mockito::Server::new();
        let url = format!("{}/ingest", server.url());

        let mock = server.mock("POST", "/ingest")
            .with_status(500)
            .with_body("Internal Error")
            .create();

        let payload = "{}";
        let result = push_telemetry(payload, &url, 1.0);

        assert!(result.is_ok()); // The function itself succeeds (enqueueing-wise)
        let summary = perform_flush();
        assert_eq!(summary.failed_jobs, 1);
        assert!(summary.common_errors.iter().any(|(e, _)| e.contains("HTTP 500")));
        mock.assert();
    }

    #[test]
    // 3. Network failure path: ensure function returns Ok because it's fire-and-forget
    fn test_push_telemetry_unreachable_host() {
        env::set_var("GLIA_LOCAL_QUEUE_LIMIT", "1000");
        let result = push_telemetry("{}", "http://invalid.local/injest", 0.1);
        assert!(result.is_ok());
        let summary = perform_flush();
        assert_eq!(summary.failed_jobs, 1);
    }
}
