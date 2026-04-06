use extendr_api::prelude::*;
use crate::core::{self, GliaClient};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::env;
use std::panic;

static CLIENT: Lazy<Mutex<Option<GliaClient>>> = Lazy::new(|| Mutex::new(None));

fn get_client() -> std::sync::MutexGuard<'static, Option<GliaClient>> {
    let mut client_lock = CLIENT.lock().unwrap();
    if client_lock.is_none() {
        let limit = env::var("CORE_QUEUE_LIMIT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000);
        *client_lock = Some(GliaClient::new(limit));
    }
    client_lock
}

/// @export
#[extendr]
pub fn enqueue_to_background(json_payload: String, url: String, timeout: f64) -> Robj {
    let result = panic::catch_unwind(|| {
        let client_lock = get_client();
        let client = client_lock.as_ref().unwrap();
        client.enqueue_to_background(&json_payload, &url, timeout)
    });

    match result {
        Ok(Ok(_)) => list!(success = true).into(),
        Ok(Err(e)) => list!(success = false, error = format!("[CORE] {}", e)).into(),
        Err(_) => list!(success = false, error = "[CORE] Rust panicked during enqueue_to_background").into(),
    }
}

/// @export
#[extendr]
pub fn flush_queue() -> Robj {
    let result = panic::catch_unwind(|| {
        let client_lock = CLIENT.lock().unwrap();
        if let Some(client) = client_lock.as_ref() {
            let summary = client.flush();
            let errors: Vec<Robj> = summary.common_errors
                .into_iter()
                .map(|(err, count)| list!(error = err, count = count).into())
                .collect();
            
            Some(list!(
                failed_jobs = summary.failed_jobs,
                common_errors = errors
            ))
        } else {
            None
        }
    });

    match result {
        Ok(Some(summary)) => summary.into(),
        Ok(None) => list!(failed_jobs = 0, common_errors = list!()).into(),
        Err(_) => list!(success = false, error = "[CORE] Rust panicked during flush_queue").into(),
    }
}

/// @export
#[extendr]
pub fn trigger_panic() {
    let _ = panic::catch_unwind(|| {
        core::trigger_panic();
    });
}

extendr_module! {
    mod core; 
    fn enqueue_to_background;
    fn flush_queue;
    fn trigger_panic;
}
