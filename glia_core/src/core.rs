use std::time::Duration;
use crossbeam_channel::{bounded, Sender, Receiver};
use std::thread;
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub enum TelemetryMessage {
    Data {
        payload: String,
        url: String,
        timeout_sec: f64,
    },
    Flush(Sender<()>),
}

pub struct FlushSummary {
    pub failed_jobs: usize,
    pub common_errors: Vec<(String, usize)>,
}

struct Stats {
    failed_count: AtomicUsize,
    error_frequency: Mutex<HashMap<String, usize>>,
}

pub struct GliaClient {
    sender: Sender<TelemetryMessage>,
    stats: Arc<Stats>,
    _worker_handle: thread::JoinHandle<()>,
}

impl GliaClient {
    pub fn new(limit: usize) -> Self {
        let (s, r) = bounded(limit);
        let stats = Arc::new(Stats {
            failed_count: AtomicUsize::new(0),
            error_frequency: Mutex::new(HashMap::new()),
        });

        let stats_clone = Arc::clone(&stats);
        let handle = thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");

            rt.block_on(async move {
                let client = reqwest::Client::new();
                let debug_mode = env::var("GLIA_DEBUG").is_ok();
                
                while let Ok(msg) = r.recv() {
                    match msg {
                        TelemetryMessage::Data { payload, url, timeout_sec } => {
                            let res = client.post(&url)
                                .header("Content-Type", "application/json")
                                .timeout(Duration::from_secs_f64(timeout_sec))
                                .body(payload)
                                .send()
                                .await;

                            let error_msg = match res {
                                Ok(resp) if resp.status().is_success() => None,
                                Ok(resp) => Some(format!("HTTP {}", resp.status())),
                                Err(e) => Some(e.to_string()),
                            };

                            if let Some(err) = error_msg {
                                stats_clone.failed_count.fetch_add(1, Ordering::SeqCst);
                                let mut freq = stats_clone.error_frequency.lock().unwrap();
                                *freq.entry(err.clone()).or_insert(0) += 1;
                                
                                if debug_mode {
                                    eprintln!("[GLIA DEBUG] Telemetry push failed: {}", err);
                                }
                            }
                        }
                        TelemetryMessage::Flush(ack_sender) => {
                            let _ = ack_sender.send(());
                        }
                    }
                }
            });
        });

        Self {
            sender: s,
            stats,
            _worker_handle: handle,
        }
    }

    pub fn queue_telemetry(&self, json_payload: &str, url: &str, timeout_sec: f64) -> Result<(), String> {
        self.sender.send(TelemetryMessage::Data {
            payload: json_payload.to_string(),
            url: url.to_string(),
            timeout_sec,
        }).map_err(|e| e.to_string())
    }

    pub fn flush(&self) -> FlushSummary {
        let (ack_sender, ack_receiver) = bounded(1);
        if self.sender.send(TelemetryMessage::Flush(ack_sender)).is_ok() {
            let _ = ack_receiver.recv();
        }

        let failed_jobs = self.stats.failed_count.swap(0, Ordering::SeqCst);
        let mut freq_map = self.stats.error_frequency.lock().unwrap();
        let common_errors: Vec<_> = freq_map.drain().collect();

        FlushSummary {
            failed_jobs,
            common_errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_client() -> GliaClient {
        GliaClient::new(100)
    }

    #[test]
    fn test_queue_telemetry_success() {
        let client = setup_client();
        let mut server = mockito::Server::new();
        let url = format!("{}/ingest", server.url());

        let mock = server.mock("POST", "/ingest")
            .with_status(200)
            .with_body("ok")
            .create();

        let result = client.queue_telemetry("{}", &url, 1.0);
        assert!(result.is_ok());
        
        let summary = client.flush();
        assert_eq!(summary.failed_jobs, 0);
        mock.assert();
    }

    #[test]
    fn test_queue_telemetry_server_error() {
        let client = setup_client();
        let mut server = mockito::Server::new();
        let url = format!("{}/ingest", server.url());

        let mock = server.mock("POST", "/ingest")
            .with_status(500)
            .create();

        let _ = client.queue_telemetry("{}", &url, 1.0);
        let summary = client.flush();
        
        assert_eq!(summary.failed_jobs, 1);
        assert!(summary.common_errors.iter().any(|(e, _)| e.contains("HTTP 500")));
        mock.assert();
    }

    #[test]
    fn test_queue_telemetry_unreachable_host() {
        let client = setup_client();
        let _ = client.queue_telemetry("{}", "http://invalid.local", 0.1);
        let summary = client.flush();
        assert_eq!(summary.failed_jobs, 1);
    }
}
