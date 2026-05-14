use ratatui::{Terminal, backend::TestBackend};
use tui::app::App;
use tui::network::JobMetrics;
use tui::ui;

#[test]
fn test_render_smoke_test_with_jobs() {
    let mut app = App::new();
    // Add 50 dummy jobs to test high-density rendering (catches out-of-bounds panics)
    for i in 0..50 {
        app.jobs.push(JobMetrics {
            started_at: "2023-10-27T12:00:00Z".to_string(),
            program_name: format!("test_job_{}", i),
            user_name: "alice".to_string(),
            wall_time_ms: 500,
            cpu_time_sec: 0.5,
            cpu_percent: 50.0,
            max_rss_kb: 5120,
            exit_code_int: 0,
        });
    }
    app.refresh_summaries();

    // Test with various areas to catch out-of-bounds rendering
    let sizes = [(80, 24), (144, 43), (40, 10)];

    for (w, h) in sizes {
        let backend = TestBackend::new(w, h);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                ui::draw(f, &mut app);
            })
            .unwrap();
    }
}

#[test]
fn test_render_smoke_test_blame_mode() {
    let mut app = App::new();
    app.blame_mode = true;
    app.jobs = vec![JobMetrics {
        started_at: "2023-10-27T12:00:00Z".to_string(),
        program_name: "test_job".to_string(),
        user_name: "alice".to_string(),
        wall_time_ms: 500,
        cpu_time_sec: 0.5,
        cpu_percent: 50.0,
        max_rss_kb: 5120,
        exit_code_int: 0,
    }];
    app.refresh_summaries();

    let sizes = [(80, 24), (144, 43)];

    for (w, h) in sizes {
        let backend = TestBackend::new(w, h);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                ui::draw(f, &mut app);
            })
            .unwrap();
    }
}
