use ratatui::{Terminal, backend::TestBackend};
use tui::app::App;
use tui::network::JobMetrics;
use tui::ui;

#[test]
fn test_ticks() {
    let mut app = App::new();
    app.blame_mode = false;
    
    // Create 10 jobs
    for i in 0..10 {
        app.jobs.push(JobMetrics {
            started_at: format!("2023-10-27T10:{:02}:00Z", i),
            program_name: format!("job{}", i),
            user_name: "alice".to_string(),
            wall_time_ms: 100 * (i + 1),
            cpu_time_sec: 0.1,
            cpu_percent: 10.0,
            max_rss_kb: 1000,
            exit_code_int: 0,
        });
    }

    let backend = TestBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();
}
