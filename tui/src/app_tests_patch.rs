    #[test]
    fn test_job_summary_aggregation_comprehensive() {
        let mut app = App::new();
        app.jobs = vec![
            JobMetrics {
                started_at: "2023-10-27T10:00:00Z".to_string(),
                program_name: "data_proc".to_string(),
                user_name: "alice".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
            JobMetrics {
                started_at: "2023-10-27T10:10:00Z".to_string(),
                program_name: "data_proc".to_string(),
                user_name: "alice".to_string(),
                wall_time_ms: 200,
                cpu_time_sec: 0.2,
                cpu_percent: 15.0,
                max_rss_kb: 2000,
                exit_code_int: 1,
            },
        ];
        
        let summaries = app.summarize_jobs();
        assert_eq!(summaries.len(), 1);
        let s = &summaries[0];
        assert_eq!(s.program_name, "data_proc");
        assert_eq!(s.count, 2);
        assert_eq!(s.avg_wall_time_ms, 150); // (100+200)/2
        assert_eq!(s.total_cpu_time_sec, 0.3); // (0.1+0.2)
        assert_eq!(s.max_rss_kb, 2000); // max(1000, 2000)
    }
