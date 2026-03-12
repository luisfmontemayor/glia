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
}

pub fn perform_push(json_payload: &str, url: &str, timeout_sec: f64) -> Result<PushResult, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs_f64(timeout_sec))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.post(url)
        .header("Content-Type", "application/json")
        .body(json_payload.to_string())
        .send()
        .map_err(|e| e.to_string())?;

    Ok(PushResult {
        status: resp.status().as_u16(),
        body: resp.text().unwrap_or_default(),
    })
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