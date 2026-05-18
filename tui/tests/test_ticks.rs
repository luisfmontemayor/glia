use ratatui::{Terminal, backend::TestBackend};
use tui::app::App;
use tui::network::JobMetrics;
use tui::ui;

#[test]
fn test_native_labels_rendered() {
    let mut app = App::new();
    app.blame_mode = false;
    app.data_point_threshold = 50;
    
    let n_jobs = 3;
    for i in 1..=n_jobs {
        app.jobs.push(JobMetrics {
            started_at: format!("2023-10-27T10:{:02}:00Z", i * 5),
            program_name: "job".to_string(),
            user_name: "user".to_string(),
            wall_time_ms: (i * 10) as i32,
            cpu_time_sec: 0.1,
            cpu_percent: 10.0,
            max_rss_kb: 1000,
            exit_code_int: 0,
        });
    }

    let width = 50;
    let backend = TestBackend::new(width, 50);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        ui::draw(f, &mut app);
    }).unwrap();

    let buffer = terminal.backend().buffer();
    
    let mut content = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            content.push_str(buffer.get(x, y).symbol());
        }
        content.push('\n');
    }
    
    // Check that native labels are actually printed
    assert!(content.contains("10:05"), "Native label 10:05 missing");
}
