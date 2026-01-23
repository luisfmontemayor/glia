use pyo3::prelude::*;
use crate::core;

#[pyclass(name = "PushResult")]
pub struct PyPushResult {
    #[pyo3(get)]
    pub status: u16,  // HTTP Code (200, 404, 500)
    #[pyo3(get)]
    pub body: String, // Server response or Error detail
}

#[pyfunction]
pub fn push_telemetry(json_payload: String, url: String, timeout: f64) -> PyResult<PyPushResult> {
    match core::perform_push(&json_payload, &url, timeout) {
        Ok(res) => Ok(PyPushResult { status: res.status, body: res.body }),
        Err(e) => Err(pyo3::exceptions::PyConnectionError::new_err(e)),
    }
}