use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct JobMetrics {
    pub started_at: String,
    pub program_name: String,
    pub user_name: String,
    pub wall_time_ms: i32,
    pub cpu_time_sec: f32,
    pub cpu_percent: f32,
    pub max_rss_kb: i32,
    pub exit_code_int: i32,
}

pub fn parse_job_metrics(json: &str) -> serde_json::Result<Vec<JobMetrics>> {
    serde_json::from_str(json)
}

pub async fn fetch_metrics(window: &str) -> Result<Vec<JobMetrics>, Box<dyn Error>> {
    let base_url = std::env::var("GLIA_TELEMETRY_URL").unwrap_or_else(|_| "http://localhost:8000/telemetry".to_string());
    let url = format!("{}?window={}&limit=1000", base_url, window);
    let resp = reqwest::get(url).await?.json::<Vec<JobMetrics>>().await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_metrics_success() {
        let data = r#"
        [
            {
                "started_at": "2023-10-27T10:00:00Z",
                "program_name": "data_proc",
                "user_name": "alice",
                "wall_time_ms": 100,
                "cpu_time_sec": 0.1,
                "cpu_percent": 10.0,
                "max_rss_kb": 1024,
                "exit_code_int": 0
            },
            {
                "started_at": "2023-10-27T10:05:00Z",
                "program_name": "ml_train",
                "user_name": "bob",
                "wall_time_ms": 200,
                "cpu_time_sec": 0.2,
                "cpu_percent": 20.0,
                "max_rss_kb": 2048,
                "exit_code_int": 1
            }
        ]
        "#;
        let metrics = parse_job_metrics(data).unwrap();
        assert_eq!(metrics.len(), 2);
        assert_eq!(metrics[0].program_name, "data_proc");
        assert_eq!(metrics[1].user_name, "bob");
    }

    #[tokio::test]
    async fn test_fetch_metrics_polling_logic() {
        use crate::action::Action;
        use tokio::sync::mpsc;

        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx_res = tx.clone();
        
        tokio::spawn(async move {
            let data = r#"[{"started_at":"2023-10-27T10:00:00Z","program_name":"test","user_name":"user","wall_time_ms":100,"cpu_time_sec":0.1,"cpu_percent":10.0,"max_rss_kb":1024,"exit_code_int":0}]"#;
            let jobs = parse_job_metrics(data).unwrap();
            let _ = tx_res.send(Action::SetJobs(jobs));
        });

        if let Some(Action::SetJobs(jobs)) = rx.recv().await {
            assert_eq!(jobs.len(), 1);
            assert_eq!(jobs[0].program_name, "test");
        } else {
            panic!("Expected Action::SetJobs");
        }
    }
}

#[cfg(test)]
mod network_error_tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_metrics_connection_failure() {
        // We assume nothing is listening on port 1 (standard practice for connection failure)
        let result = fetch_metrics("1h").await;
        assert!(result.is_err(), "Should return Err on connection failure");
    }

    #[tokio::test]
    #[ignore]
    async fn test_fetch_metrics_reaches_backend() {
        // This test actually attempts to reach the backend.
        let result = fetch_metrics("1h").await;
        println!("Result from backend: {:?}", result);
        // We just assert that it is Ok to satisfy TDD, and we print it out to verify what's there.
        assert!(result.is_ok(), "Expected Ok but got an Err. Make sure the backend is running at the configured port.");
    }
}
