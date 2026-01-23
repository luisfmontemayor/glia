use extendr_api::prelude::*;
use crate::core;

#[extendr]
pub fn push_telemetry(json_payload: String, url: String, timeout: f64) -> Robj {
    match core::perform_push(&json_payload, &url, timeout) {
        Ok(res) => list!(status = res.status, body = res.body).into(),
        Err(e) => list!(status = 0, body = e).into(),
    }
}