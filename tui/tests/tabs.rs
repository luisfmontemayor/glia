use ratatui::{backend::TestBackend, style::Modifier, Terminal};
use tui::app::{App, Metric};
use tui::components::tabs::render_tabs;
use tui::theme::PINK;

#[test]
fn test_active_tab_highlighting() {
    let mut app = App::new();
    app.metric = Metric::CpuPercent;

    let backend = TestBackend::new(100, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = f.size();
            render_tabs(f, &app, area);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let mut found_highlight = false;

    // Looking for the highlighted tab "CPU Percent"
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            let cell = buffer.get(x, y);
            if cell.symbol() == "C" && buffer.get(x + 1, y).symbol() == "P" && buffer.get(x + 2, y).symbol() == "U" {
                let third_cell = buffer.get(x + 4, y);
                if third_cell.symbol() == "P" { // "CPU Percent"
                    if cell.style().fg == Some(PINK) && cell.style().add_modifier.contains(Modifier::BOLD) {
                        found_highlight = true;
                    }
                }
            }
        }
    }
    assert!(found_highlight, "Should find the active tab highlighted with PINK and BOLD");
}
