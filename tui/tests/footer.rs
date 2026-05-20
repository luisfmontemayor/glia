use ratatui::{backend::TestBackend, Terminal};
use tui::app::App;
use tui::components::footer::render_footer;

#[test]
fn test_footer_static_keybinds() {
    let app = App::new();
    let backend = TestBackend::new(100, 3);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = f.size();
            render_footer(f, &app, area);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let mut content = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            content.push_str(buffer.get(x, y).symbol());
        }
    }
    
    assert!(content.contains("[p] Command Palette"), "Should contain command palette keybind");
    assert!(content.contains("[q] Quit"), "Should contain quit keybind");
}
