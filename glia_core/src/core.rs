use std::time::Duration;

pub struct PushResult {
    pub status: u16,
    pub body: String,
}

pub fn perform_push(json_payload: &str, url: &str, timeout_sec: f64) -> Result<PushResult, String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs_f64(timeout_sec))
        .build()
        .map_err(|e| e.to_string())?;

    let endpoint = format!("{}/ingest", url.trim_end_matches('/'));

    let resp = client.post(&endpoint)
        .header("Content-Type", "application/json")
        .body(json_payload.to_string())
        .send()
        .map_err(|e| e.to_string())?;

    Ok(PushResult {
        status: resp.status().as_u16(),
        body: resp.text().unwrap_or_default(),
    })
}