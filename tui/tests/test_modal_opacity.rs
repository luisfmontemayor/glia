use ratatui::{Terminal, backend::TestBackend, style::{Color, Style}};
use tui::app::App;
use tui::network::JobMetrics;
use tui::ui;

#[test]
fn test_modal_is_opaque_and_centered() {
    let mut app = App::new();
    
    // 1. First, setup app so it WOULD have rendered background content if jobs were present.
    // We want to ensure that even if the background logic runs, the modal clears it.
    app.jobs = vec![]; 
    app.is_loading = false;
    
    let width = 80;
    let height = 24;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        // Force render something in the background manually to simulate background lines
        // Ratatui draw calls are sequential.
        for y in 0..height {
            for x in 0..width {
                f.render_widget(ratatui::widgets::Paragraph::new("X").style(Style::default().fg(Color::Yellow)), ratatui::layout::Rect::new(x, y, 1, 1));
            }
        }
        
        ui::draw(f, &mut app);
    }).unwrap();

    let buffer = terminal.backend().buffer();
    
    // The "no data" modal should be centered.
    // Currently it's 60x30% of f.size() (80x24) -> 48x7 approx.
    // We will verify that in the middle of the screen, there are NO "X" characters.
    
    let mid_x = width / 2;
    let mid_y = height / 2;
    
    // Check a small area around the center (where the modal should definitely be)
    for y in (mid_y - 2)..(mid_y + 2) {
        for x in (mid_x - 10)..(mid_x + 10) {
            let sym = buffer.get(x, y).symbol();
            assert!(sym != "X", "Background character 'X' leaked into modal at ({}, {})", x, y);
        }
    }
}

#[test]
fn test_detail_modal_is_opaque() {
    let mut app = App::new();
    app.jobs = vec![JobMetrics {
        started_at: "2023-10-27T10:00:00Z".to_string(),
        program_name: "test_job".to_string(),
        user_name: "alice".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    }];
    app.refresh_summaries();
    app.show_detail = true;
    app.jobs_table_state.row_state.select(Some(0));

    let width = 80;
    let height = 24;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        // Force background characters
        for y in 0..height {
            for x in 0..width {
                f.render_widget(ratatui::widgets::Paragraph::new("B").style(Style::default().fg(Color::Blue)), ratatui::layout::Rect::new(x, y, 1, 1));
            }
        }
        ui::draw(f, &mut app);
    }).unwrap();

    let buffer = terminal.backend().buffer();
    let mid_x = width / 2;
    let mid_y = height / 2;
    
    for y in (mid_y - 2)..(mid_y + 2) {
        for x in (mid_x - 10)..(mid_x + 10) {
            let sym = buffer.get(x, y).symbol();
            assert!(sym != "B", "Background character 'B' leaked into detail modal at ({}, {})", x, y);
        }
    }
}
