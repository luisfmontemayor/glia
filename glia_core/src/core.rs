use std::time::Duration;
use crossbeam_channel::{bounded, Sender, Receiver};
use once_cell::sync::Lazy;
use std::thread;
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct PushResult {
    pub status: u16,
    pub body: String,
struct TelemetryMessage {
    payload: String,
    url: String,
    timeout_sec: f64,
}

pub fn perform_push(json_payload: &str, url: &str, timeout_sec: f64) -> Result<PushResult, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs_f64(timeout_sec))
        .build()
        .map_err(|e| e.to_string())?;
pub struct FlushSummary {
    pub failed_jobs: usize,
    pub common_errors: Vec<(String, usize)>,
}

    let resp = client.post(url)
        .header("Content-Type", "application/json")
        .body(json_payload.to_string())
        .send()
        .map_err(|e| e.to_string())?;
struct Stats {
    failed_count: AtomicUsize,
    error_frequency: Mutex<HashMap<String, usize>>,
}

    Ok(PushResult {
        status: resp.status().as_u16(),
        body: resp.text().unwrap_or_default(),
    })
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

/// Fire-and-forget telemetry push. Enqueues the payload to a background worker.
pub fn push_telemetry(json_payload: &str, url: &str, timeout_sec: f64) -> Result<(), String> {
    let (sender, _) = &*CHANNEL;
    sender.send(TelemetryMessage {
        payload: json_payload.to_string(),
        url: url.to_string(),
        timeout_sec,
    }).map_err(|e| e.to_string())
}

/// Blocks until all enqueued telemetry messages have been processed.
/// Returns a summary of failures encountered since the last flush.
pub fn perform_flush() -> FlushSummary {
    let (sender, _) = &*CHANNEL;
    while !sender.is_empty() {
        thread::sleep(Duration::from_millis(10));
    }

    let failed_jobs = STATS.failed_count.swap(0, Ordering::SeqCst);
    let mut freq_map = STATS.error_frequency.lock().unwrap();
    let mut common_errors: Vec<_> = freq_map.drain().collect();
    
    // Sort by frequency descending
    common_errors.sort_by(|a, b| b.1.cmp(&a.1));

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
    fn test_perform_push_success() {
        let mut server = mockito::Server::new();
        let url = format!("{}/ingest", server.url());

        let _mock = server.mock("POST", "/ingest")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{\"status\": \"success\"}")
            .create();

        let payload = r#"{"run_id": "123", "cpu_percent": 50.0}"#;
        let result = perform_push(payload, &url, 1.0);

        
        assert!(result.is_ok());
        let push_result = result.unwrap();
        assert_eq!(push_result.status, 200);
        assert_eq!(push_result.body, "{\"status\": \"success\"}");
    }

    #[test]
    // 2. Server failure path: ensure function returns Ok even if server responds with 500
    fn test_perform_push_server_error() {
        let mut server = mockito::Server::new();
        let url = format!("{}/ingest", server.url());

        // Mock a 500 Internal Server Error
        let _mock = server.mock("POST", "/ingest")
            .with_status(500)
            .with_body("Internal Error")
            .create();

        let payload = "{}";
        let result = perform_push(payload, &url, 1.0);

        assert!(result.is_ok()); // The function itself succeeds (network-wise)
        let push_result = result.unwrap();
        assert_eq!(push_result.status, 500);
        assert_eq!(push_result.body, "Internal Error");
    }

    #[test]
    // 3. Network failure path: ensure function returns Err when host is unreachable
    fn test_perform_push_unreachable_host() {
        let result = perform_push("{}", "http://invalid.local/injest", 0.1);
        assert!(result.is_err());
    }
}