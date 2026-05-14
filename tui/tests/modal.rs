use ratatui::{backend::TestBackend, Terminal};
use tui::action::Action;
use tui::app::{App, Pane};
use tui::components::modal::render_modal;
use tui::network::JobMetrics;

#[test]
fn test_modal_data_population() {
    let mut app = App::new();
    let now = chrono::Utc::now().to_rfc3339();
    let jobs = vec![
        JobMetrics {
            started_at: now.clone(),
            program_name: "test_job".to_string(),
            user_name: "user1".to_string(),
            wall_time_ms: 1234,
            cpu_time_sec: 1.5,
            cpu_percent: 50.0,
            max_rss_kb: 5000,
            exit_code_int: 0,
        },
        JobMetrics {
            started_at: now,
            program_name: "test_job".to_string(),
            user_name: "user2".to_string(),
            wall_time_ms: 1000,
            cpu_time_sec: 1.0,
            cpu_percent: 40.0,
            max_rss_kb: 4000,
            exit_code_int: 1, // Failure
        },
    ];
    
    app.update(Action::SetJobs(jobs));
    app.focused_pane = Pane::Jobs;
    // Mock row selection 
    app.jobs_table_state.row_state.select(Some(0));
    app.show_detail = true;

    let backend = TestBackend::new(100, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            render_modal(f, &app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let mut content = String::new();
    for y in 0..buffer.area.height {
        let mut line = String::new();
        for x in 0..buffer.area.width {
            line.push_str(buffer.get(x, y).symbol());
        }
        content.push_str(&line.trim());
        content.push('\n');
    }

    // Verify program name
    assert!(content.contains("Detail: test_job"), "Should contain title with program name");
    
    // Verify success/failure count
    assert!(content.contains("1 Success"), "Should contain correct success count");
    assert!(content.contains("1 Failure"), "Should contain correct failure count");

    // Verify users (sorted and deduped)
    assert!(content.contains("user1, user2"), "Should contain users string");

    // Verify average math formatting 
    // Avg wall time: (1234 + 1000) / 2 = 1117
    assert!(content.contains("1117 ms"), "Should contain average wall time");
}
