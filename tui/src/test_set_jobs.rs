#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::action::Action;
    use crate::network::JobMetrics;

    #[test]
    fn test_app_set_jobs_action() {
        let mut app = App::new();
        let new_jobs = vec![
            JobMetrics {
                started_at: "2023-10-27T12:00:00Z".to_string(),
                program_name: "new_job".to_string(),
                user_name: "charles".to_string(),
                wall_time_ms: 500,
                cpu_time_sec: 0.5,
                cpu_percent: 50.0,
                max_rss_kb: 5120,
                exit_code_int: 0,
            },
        ];
        
        // This will fail to compile if SetJobs is missing
        app.update(Action::SetJobs(new_jobs.clone()));
        
        assert_eq!(app.jobs.len(), 1);
        assert_eq!(app.jobs[0].program_name, "new_job");
    }
}
