use ratatui::{Terminal, backend::TestBackend};
use tui::app::App;
use tui::network::JobMetrics;
use tui::ui;

#[test]
fn should_render_line_chart_in_blame_mode() {
    let mut app = App::new();
    app.blame_mode = true;
    app.jobs = vec![
        JobMetrics {
            started_at: "2023-10-27T10:00:00Z".to_string(),
            program_name: "job1".to_string(),
            user_name: "alice".to_string(),
            wall_time_ms: 100,
            cpu_time_sec: 0.1,
            cpu_percent: 10.0,
            max_rss_kb: 1000,
            exit_code_int: 0,
        },
        JobMetrics {
            started_at: "2023-10-27T10:05:00Z".to_string(),
            program_name: "job2".to_string(),
            user_name: "bob".to_string(),
            wall_time_ms: 200,
            cpu_time_sec: 0.2,
            cpu_percent: 15.0,
            max_rss_kb: 2000,
            exit_code_int: 0,
        },
    ];

    let backend = TestBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();

    let mut has_braille = false;
    let mut has_bar = false;

    for cell in buffer.content() {
        let symbol = cell.symbol();
        if symbol
            .chars()
            .any(|c| (0x2800..=0x28FF).contains(&(c as u32)))
        {
            has_braille = true;
        }
        if symbol.contains('█') {
            has_bar = true;
        }
    }

    assert!(
        has_braille,
        "Buffer should contain braille characters (line chart)"
    );
    assert!(!has_bar, "Buffer should not contain bar characters (█)");
}

#[test]
fn test_low_density_blame_mode_alignment() {
    let mut app = App::new();
    app.blame_mode = true;
    app.data_point_threshold = 15;

    // 2 points -> should have labels evenly distributed
    app.jobs = vec![
        JobMetrics {
            started_at: "2023-10-27T10:00:00Z".to_string(),
            program_name: "job1".to_string(),
            user_name: "alice".to_string(),
            wall_time_ms: 100,
            cpu_time_sec: 0.1,
            cpu_percent: 10.0,
            max_rss_kb: 1000,
            exit_code_int: 0,
        },
        JobMetrics {
            started_at: "2023-10-27T10:05:00Z".to_string(),
            program_name: "job2".to_string(),
            user_name: "bob".to_string(),
            wall_time_ms: 200,
            cpu_time_sec: 0.2,
            cpu_percent: 15.0,
            max_rss_kb: 2000,
            exit_code_int: 0,
        },
    ];

    let backend = TestBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();

    let mut found_10_00 = false;
    let mut found_10_05 = false;

    for x in 0..100 {
        for y in 0..50 {
            let cell = buffer.get(x, y);
            let s = cell.symbol();
            if s == "1" {
                let mut label = String::new();
                for dx in 0..5 {
                    if x + dx < 100 {
                        label.push_str(buffer.get(x + dx, y).symbol());
                    }
                }
                if label == "10:00" {
                    found_10_00 = true;
                }
                if label == "10:05" {
                    found_10_05 = true;
                }
            }
        }
    }

    assert!(found_10_00, "Should find 10:00 label");
    assert!(found_10_05, "Should find 10:05 label");
}
