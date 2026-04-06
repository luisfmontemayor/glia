use std::time::Duration;
use tokio::sync::mpsc::{channel, Sender};
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
    Flush(std::sync::mpsc::Sender<()>),
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
        let (s, mut r) = channel(limit);
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
                let debug_mode = env::var("CORE_DEBUG").is_ok();
                
                let batch_size = env::var("CORE_BATCH_SIZE")
                    .ok()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1000);
                
                let batch_timeout_sec = env::var("CORE_BATCH_TIMEOUT_SEC")
                    .ok()
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(2);

                let mut buffer: Vec<(String, String, f64)> = Vec::with_capacity(batch_size);
                let mut last_send = tokio::time::Instant::now();

                loop {
                    let timeout = tokio::time::Duration::from_secs(batch_timeout_sec);
                    let sleep = tokio::time::sleep_until(last_send + timeout);
                    
                    tokio::select! {
                        msg = r.recv() => {
                            match msg {
                                Some(TelemetryMessage::Data { payload, url, timeout_sec }) => {
                                    buffer.push((payload, url, timeout_sec));
                                    if buffer.len() >= batch_size {
                                        Self::send_batch(&client, &mut buffer, &stats_clone, debug_mode).await;
                                        last_send = tokio::time::Instant::now();
                                    }
                                }
                                Some(TelemetryMessage::Flush(ack_sender)) => {
                                    if !buffer.is_empty() {
                                        Self::send_batch(&client, &mut buffer, &stats_clone, debug_mode).await;
                                    }
                                    let _ = ack_sender.send(());
                                    last_send = tokio::time::Instant::now();
                                }
                                None => break, // Channel closed
                            }
                        }
                        _ = sleep => {
                            if !buffer.is_empty() {
                                Self::send_batch(&client, &mut buffer, &stats_clone, debug_mode).await;
                            }
                            last_send = tokio::time::Instant::now();
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

    async fn send_batch(
        client: &reqwest::Client,
        buffer: &mut Vec<(String, String, f64)>,
        stats: &Arc<Stats>,
        debug_mode: bool
    ) {
        if buffer.is_empty() { return; }

        // For now, we assume all items in a batch go to the same URL and have same timeout
        // (This matches current usage where GLIA_PYTHON sends all to one GLIA_API_URL)
        let (_first_payload, url, timeout_sec) = &buffer[0];
        let url = url.clone();
        let timeout_sec = *timeout_sec;

        // Merge payloads. Each payload is a JSON list string like "[{...}]"
        // We want to merge them into a single list "[{...},{...},...]"
        let mut merged_payload = String::from("[");
        for (i, (payload, _, _)) in buffer.iter().enumerate() {
            let stripped = payload.trim().trim_start_matches('[').trim_end_matches(']');
            if !stripped.is_empty() {
                if i > 0 {
                    merged_payload.push(',');
                }
                merged_payload.push_str(stripped);
            }
        }
        merged_payload.push(']');

        let res = client.post(&url)
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs_f64(timeout_sec))
            .body(merged_payload)
            .send()
            .await;

        let error_msg = match res {
            Ok(resp) if resp.status().is_success() => None,
            Ok(resp) => Some(format!("HTTP {}", resp.status())),
            Err(e) => Some(e.to_string()),
        };

        if let Some(err) = error_msg {
            // If the whole batch fails, we count ALL jobs in it as failed
            stats.failed_count.fetch_add(buffer.len(), Ordering::SeqCst);
            let mut freq = stats.error_frequency.lock().unwrap();
            *freq.entry(err.clone()).or_insert(0) += 1;
            
            if debug_mode {
                eprintln!("[CORE DEBUG] Batch push failed ({} jobs): {}", buffer.len(), err);
            }
        }

        buffer.clear();
    }

    pub fn enqueue_to_background(&self, json_payload: &str, url: &str, timeout_sec: f64) -> Result<(), String> {
        self.sender.try_send(TelemetryMessage::Data {
            payload: json_payload.to_string(),
            url: url.to_string(),
            timeout_sec,
        }).map_err(|e| e.to_string())
    }

    pub fn flush(&self) -> FlushSummary {
        let timeout_sec = env::var("CORE_FLUSH_TIMEOUT_SEC")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(5);

        let (ack_sender, ack_receiver) = std::sync::mpsc::channel();
        if self.sender.try_send(TelemetryMessage::Flush(ack_sender)).is_ok() {
            let _ = ack_receiver.recv_timeout(Duration::from_secs(timeout_sec));
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

/// A canary function to test FFI panic handling.
pub fn trigger_panic() {
    panic!("[CORE] INTENTIONAL PANIC: Testing FFI boundary safety.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_client(limit: usize) -> GliaClient {
        GliaClient::new(limit)
    }

    #[test]
    fn test_queue_overflow() {
        // Create a client with a very small queue
        let client = setup_client(2);
        
        // Fill the queue
        assert!(client.enqueue_to_background("{}", "http://localhost", 1.0).is_ok());
        assert!(client.enqueue_to_background("{}", "http://localhost", 1.0).is_ok());
        
        // The third one should fail because the channel is bounded and full
        let result = client.enqueue_to_background("{}", "http://localhost", 1.0);
        assert!(result.is_err());
        // tokio mpsc try_send error message contains "no available capacity" or similar
        assert!(result.unwrap_err().to_string().to_lowercase().contains("capacity"));
    }

    #[tokio::test]
    async fn test_enqueue_to_background_success() {
        let client = setup_client(100);
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/ingest", server.url());

        let mock = server.mock("POST", "/ingest")
            .with_status(200)
            .with_body("ok")
            .create_async()
            .await;

        let result = client.enqueue_to_background("{}", &url, 1.0);
        assert!(result.is_ok());
        
        let summary = client.flush();
        assert_eq!(summary.failed_jobs, 0);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_enqueue_to_background_server_error() {
        let client = setup_client(100);
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/ingest", server.url());

        let mock = server.mock("POST", "/ingest")
            .with_status(500)
            .create_async()
            .await;

        let _ = client.enqueue_to_background("{}", &url, 1.0);
        let summary = client.flush();
        
        assert_eq!(summary.failed_jobs, 1);
        assert!(summary.common_errors.iter().any(|(e, _)| e.contains("HTTP 500")));
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_enqueue_to_background_unreachable_host() {
        let client = setup_client(100);
        let _ = client.enqueue_to_background("{}", "http://invalid.local", 0.1);
        let summary = client.flush();
        assert_eq!(summary.failed_jobs, 1);
    }

    #[tokio::test]
    async fn test_flush_clears_multiple_items() {
        let client = setup_client(100);
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/ingest", server.url());

        // We expect ONE batch request containing 3 items
        let mock = server.mock("POST", "/ingest")
            .expect(1)
            .with_status(200)
            .create_async()
            .await;

        client.enqueue_to_background("{}", &url, 1.0).unwrap();
        client.enqueue_to_background("{}", &url, 1.0).unwrap();
        client.enqueue_to_background("{}", &url, 1.0).unwrap();

        let summary = client.flush();
        
        assert_eq!(summary.failed_jobs, 0);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_batching_by_count() {
        std::env::set_var("CORE_BATCH_SIZE", "2000"); // Ensure they don't auto-send
        std::env::set_var("CORE_BATCH_TIMEOUT_SEC", "60"); // Don't time out
        
        let client = GliaClient::new(2000);
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/ingest", server.url());

        // We expect ONE batch request containing 1000 items
        let mock = server.mock("POST", "/ingest")
            .expect(1)
            .with_status(201)
            .create_async()
            .await;

        for _ in 0..1000 {
            client.enqueue_to_background("{\"id\": 1}", &url, 1.0).unwrap();
        }

        // Give the worker thread a tiny bit of time to move items from the channel to the buffer
        tokio::time::sleep(Duration::from_millis(100)).await;

        let summary = client.flush();
        assert_eq!(summary.failed_jobs, 0);
        mock.assert_async().await;
        
        std::env::remove_var("CORE_BATCH_SIZE");
        std::env::remove_var("CORE_BATCH_TIMEOUT_SEC");
    }

    #[tokio::test]
    async fn test_batching_by_time() {
        let client = GliaClient::new(100);
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/ingest", server.url());

        let mock = server.mock("POST", "/ingest")
            .expect(1)
            .with_status(201)
            .create_async()
            .await;

        client.enqueue_to_background("{\"id\": 1}", &url, 1.0).unwrap();

        // Should NOT be sent yet
        tokio::time::sleep(Duration::from_millis(500)).await;
        // mock.assert() might fail here if it's sent immediately

        // Should be sent after 2 seconds
        tokio::time::sleep(Duration::from_millis(2100)).await;
        
        let summary = client.flush();
        assert_eq!(summary.failed_jobs, 0);
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_batching_with_env_vars() {
        std::env::set_var("CORE_BATCH_SIZE", "5");
        std::env::set_var("CORE_BATCH_TIMEOUT_SEC", "1");
        
        let client = GliaClient::new(100);
        let mut server = mockito::Server::new_async().await;
        let url = format!("{}/ingest", server.url());

        let mock = server.mock("POST", "/ingest")
            .expect(1)
            .with_status(201)
            .create_async()
            .await;

        for _ in 0..5 {
            client.enqueue_to_background("{\"id\": 1}", &url, 1.0).unwrap();
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let summary = client.flush();
        assert_eq!(summary.failed_jobs, 0);
        mock.assert_async().await;
        
        // Clean up env vars for other tests
        std::env::remove_var("CORE_BATCH_SIZE");
        std::env::remove_var("CORE_BATCH_TIMEOUT_SEC");
    }

    #[tokio::test]
    async fn test_flush_timeout_config() {
        std::env::set_var("CORE_FLUSH_TIMEOUT_SEC", "1");
        let client = GliaClient::new(100);
        
        // This test doesn't easily prove the timeout happened without mocks that hang,
        // but we can at least verify it doesn't crash and respects the variable path.
        let start = std::time::Instant::now();
        let _ = client.flush();
        let duration = start.elapsed();
        
        // It shouldn't take MUCH longer than the timeout (plus some overhead)
        // Default is 5s, we set 1s.
        assert!(duration.as_secs() < 5);

        std::env::remove_var("CORE_FLUSH_TIMEOUT_SEC");
    }
}
