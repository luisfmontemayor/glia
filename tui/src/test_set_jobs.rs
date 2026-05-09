#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::action::Action;
    use crate::network::JobMetrics;

    #[test]
    fn test_app_set_jobs_action() {
        let mut app = App::new();
        app.window = crate::app::TimeWindow::WMax;
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

    #[test]
    fn test_set_jobs_auto_increases_window() {
        use crate::app::TimeWindow;
        let mut app = App::new();
        app.window = TimeWindow::W1h;
        
        // Create a job that is older than 1h but newer than 3h
        let now = chrono::Utc::now();
        let old_time = now - chrono::Duration::hours(2);
        let job = JobMetrics {
            started_at: old_time.to_rfc3339(),
            program_name: "old_job".to_string(),
            user_name: "user".to_string(),
            wall_time_ms: 100,
            cpu_time_sec: 0.1,
            cpu_percent: 10.0,
            max_rss_kb: 1000,
            exit_code_int: 0,
        };
        
        app.update(Action::SetJobs(vec![job]));
        
        // It should have increased the window to W3h to include the job
        assert_eq!(app.window, TimeWindow::W3h);
        assert_eq!(app.jobs.len(), 1);
    }
}
