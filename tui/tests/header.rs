use ratatui::{backend::TestBackend, Terminal};
use tui::app::{App, TimeWindow};
use tui::components::header::render_header;
use tui::theme::{GREEN, RED};

#[test]
fn test_header_status_colors() {
    let mut app = App::new();
    app.db_status = true;
    app.api_status = false;

    let backend = TestBackend::new(100, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = f.size();
            render_header(f, &app, area);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let mut found_active = false;
    let mut found_inactive = false;

    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            let cell = buffer.get(x, y);
            if cell.symbol() == "A" && buffer.get(x + 1, y).symbol() == "C" {
                if cell.style().fg == Some(GREEN) {
                    found_active = true;
                }
            }
            if cell.symbol() == "I" && buffer.get(x + 1, y).symbol() == "N" {
                if cell.style().fg == Some(RED) {
                    found_inactive = true;
                }
            }
        }
    }
    assert!(found_active, "Should find ACTIVE in GREEN");
    assert!(found_inactive, "Should find INACTIVE in RED");
}

#[test]
fn test_header_time_window_labels() {
    let mut app = App::new();
    app.window = TimeWindow::WMax;

    let backend = TestBackend::new(100, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = f.size();
            render_header(f, &app, area);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let mut content = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            content.push_str(buffer.get(x, y).symbol());
        }
    }
    assert!(content.contains("All Time"), "WMax should map to All Time");
}

#[test]
fn test_time_window_header_shows_shift_t() {
    let mut app = App::new();
    let backend = TestBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            tui::ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let mut content = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            content.push_str(buffer.get(x, y).symbol());
        }
    }
    assert!(content.contains("[t / T]") || content.contains("t / T"), "Should find [t / T] in header");
}
