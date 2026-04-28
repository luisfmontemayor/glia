use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_job_metrics() {
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
}
