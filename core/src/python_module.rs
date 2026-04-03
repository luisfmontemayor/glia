use pyo3::prelude::*;
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

#[pyfunction]
#[pyo3(signature = (json_payload, url, timeout=1.0))]
pub fn queue_telemetry(json_payload: String, url: String, timeout: f64) -> PyResult<()> {
    let result = panic::catch_unwind(|| {
        let client_lock = get_client();
        let client = client_lock.as_ref().unwrap();
        client.queue_telemetry(&json_payload, &url, timeout)
    });

    match result {
        Ok(inner) => inner.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("[CORE] {}", e))),
        Err(_) => Err(pyo3::exceptions::PyRuntimeError::new_err("[CORE] Rust panicked during queue_telemetry")),
    }
}

#[pyfunction]
pub fn flush_queue() -> PyResult<PyFlushSummary> {
    let result = panic::catch_unwind(|| {
        let client_lock = CLIENT.lock().unwrap();
        if let Some(client) = client_lock.as_ref() {
            let summary = client.flush();
            Some(PyFlushSummary {
                failed_jobs: summary.failed_jobs,
                common_errors: summary.common_errors,
            })
        } else {
            None
        }
    });

    match result {
        Ok(Some(summary)) => Ok(summary),
        Ok(None) => Ok(PyFlushSummary {
            failed_jobs: 0,
            common_errors: Vec::new(),
        }),
        Err(_) => Err(pyo3::exceptions::PyRuntimeError::new_err("[CORE] Rust panicked during flush_queue")),
    }
}

#[pyfunction]
pub fn trigger_panic() -> PyResult<()> {
    let result = panic::catch_unwind(|| {
        core::trigger_panic();
    });

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(pyo3::exceptions::PyRuntimeError::new_err("[CORE] Intentional Rust panic caught at FFI boundary")),
    }
}

#[pyclass(name = "FlushSummary")]
pub struct PyFlushSummary {
    #[pyo3(get)]
    pub failed_jobs: usize,
    #[pyo3(get)]
    pub common_errors: Vec<(String, usize)>,
}
