use ratatui::{Terminal, backend::TestBackend};
use tui::app::{App, TimeWindow};
use tui::network::JobMetrics;
use tui::ui;

#[test]
fn test_no_data_modal_visibility() {
    let mut app = App::new();
    app.jobs.clear();
    app.is_loading = false;
    app.window = TimeWindow::W1h;

    let backend = TestBackend::new(80, 100);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    
    let mut buffer_text = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            buffer_text.push_str(buffer.get(x, y).symbol());
        }
        buffer_text.push('\n');
    }

    let buffer_text = buffer_text; // just to keep it

    assert!(
        buffer_text.contains("No data available for time window."),
        "Buffer did not contain expected message: 'No data available for time window.'"
    );
    assert!(
        buffer_text.contains("Waiting for updates..."),
        "Buffer did not contain expected message: 'Waiting for updates...'"
    );
}

#[test]
fn test_x_axis_ticks_unification() {
    let mut app = App::new();
    app.jobs = vec![JobMetrics {
        started_at: "2023-10-27T12:00:00Z".to_string(),
        program_name: "test_job".to_string(),
        user_name: "user".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    }];
    app.is_loading = false;
    app.blame_mode = false; // Bar chart

    let backend = TestBackend::new(80, 100);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    
    let mut found = false;
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            if buffer.get(x, y).symbol() == "┬" {
                found = true;
                break;
            }
        }
    }

    assert!(found, "Exactly character '┬' (U+252C) not found in the output");
}
