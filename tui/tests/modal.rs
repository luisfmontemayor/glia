use ratatui::{
    backend::TestBackend,
    style::{Color, Style},
    Terminal,
};
use tui::action::Action;
use tui::app::{App, Pane};
use tui::components::modal::render_modal;
use tui::network::JobMetrics;
use tui::ui;

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
            tui::ui::draw(f, &mut app);
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

#[test]
fn test_modal_no_data_no_title() {
    let mut app = App::new();
    app.jobs = vec![];
    app.show_detail = true;
    app.is_loading = false;

    let backend = TestBackend::new(100, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            tui::ui::draw(f, &mut app);
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

    // Check for the message
    assert!(content.contains("No data available for this time window"), "Should contain the no data message");

    // Ensure NO title border text
    assert!(!content.contains("Detail"), "Should NOT contain 'Detail' title");
    assert!(!content.contains("No Selection"), "Should NOT contain 'No Selection' title");
}

#[test]
fn test_modal_is_opaque_and_centered() {
    let mut app = App::new();
    
    // 1. Setup app so it WOULD have rendered background content if jobs were present.
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
