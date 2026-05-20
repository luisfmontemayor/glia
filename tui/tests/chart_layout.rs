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
    std::fs::write("modal_layout.txt", &buffer_text).unwrap();

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
    
    let mut content = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            content.push_str(buffer.get(x, y).symbol());
        }
        content.push('\n');
    }

    assert!(content.contains("12:00"), "Native BarGroup label '12:00' not found in the output");
}

#[test]
fn test_barchart_label_drift() {
    let mut app = App::new();
    app.is_loading = false;
    app.blame_mode = false;
    app.metric = tui::app::Metric::WallTime;
    
    // Create 10 jobs
    app.jobs = (0..10).map(|i| JobMetrics {
        started_at: format!("2023-10-27T12:0{i}:00Z"),
        program_name: "test".to_string(),
        user_name: "user".to_string(),
        wall_time_ms: (i + 1) * 10,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    }).collect();

    // The available width for the chart will be slightly less than the backend width due to borders.
    // If b_width = 5, b_gap = 1, then each bar takes 6 columns. 10 bars require 60 columns.
    // Let's set backend width to 50, which should definitely cut off the last few bars.
    let backend = TestBackend::new(50, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| ui::draw(f, &mut app)).unwrap();

    let buffer = terminal.backend().buffer();
    let mut content = String::new();
    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            content.push_str(buffer.get(x, y).symbol());
        }
        content.push('\n');
    }

    println!("RENDERED BUFFER:\n{}", content);
    std::fs::write("buffer_output.txt", &content).unwrap();
    
    // With the fix, labels correctly track the bars. 
    // 12:04 should be printed. 12:06 and 12:09 fall off the chart area and should NOT be printed.
    assert!(content.contains("12:04"), "12:04 should be visible");
    assert!(!content.contains("12:06"), "12:06 should be pushed off screen just like its corresponding bar");
    assert!(!content.contains("12:09"), "12:09 should be pushed off screen just like its corresponding bar");
}

#[test]
fn test_axis_line_symmetric_overhang() {
    let mut app = App::new();
    app.is_loading = false;
    app.blame_mode = false;
    app.metric = tui::app::Metric::WallTime;

    app.jobs = (0..4).map(|i| JobMetrics {
        started_at: format!("2023-10-27T12:0{i}:00Z"),
        program_name: "test".to_string(),
        user_name: "user".to_string(),
        wall_time_ms: (i + 1) * 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    }).collect();

    let backend = TestBackend::new(100, 50);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| ui::draw(f, &mut app)).unwrap();

    let buffer = terminal.backend().buffer();

    // Find the axis line row: contains ─ with ┬ ticks, between rows 5 and 45
    let mut axis_row = None;
    for y in 5..45u16 {
        let mut has_horizontal = false;
        let mut has_tick = false;
        for x in 0..buffer.area.width {
            let sym = buffer.get(x, y).symbol();
            if sym == "─" { has_horizontal = true; }
            if sym == "┬" { has_tick = true; }
        }
        if has_horizontal && has_tick {
            axis_row = Some(y);
            break;
        }
    }
    let axis_y = axis_row.expect("Could not find axis line row with ─ and ┬");

    // Find leftmost/rightmost axis chars on the axis row (left half = graph pane)
    let half_w = buffer.area.width / 2;
    let mut axis_left: Option<u16> = None;
    let mut axis_right: Option<u16> = None;
    for x in 0..half_w {
        let sym = buffer.get(x, axis_y).symbol();
        if sym == "─" || sym == "┬" {
            if axis_left.is_none() { axis_left = Some(x); }
            axis_right = Some(x);
        }
    }
    let axis_left_x = axis_left.expect("No axis chars found") as i32;
    let axis_right_x = axis_right.unwrap() as i32;

    // Find leftmost/rightmost bar characters above the axis
    let bar_chars = ['▂', '▃', '▄', '▅', '▆', '▇', '█', '▁'];
    let mut bar_left: Option<u16> = None;
    let mut bar_right: Option<u16> = None;
    for y in 0..axis_y {
        for x in 0..half_w {
            let sym = buffer.get(x, y).symbol();
            if sym.len() >= 3 {
                if let Some(c) = sym.chars().next() {
                    if bar_chars.contains(&c) {
                        if bar_left.is_none() || x < bar_left.unwrap() {
                            bar_left = Some(x);
                        }
                        if bar_right.is_none() || x > bar_right.unwrap() {
                            bar_right = Some(x);
                        }
                    }
                }
            }
        }
    }
    let bar_left_x = bar_left.expect("No bar characters found") as i32;
    let bar_right_x = bar_right.unwrap() as i32;

    let left_overhang = bar_left_x - axis_left_x;
    let right_overhang = axis_right_x - bar_right_x;

    assert!(
        (left_overhang - right_overhang).abs() <= 1,
        "Axis line overhang is asymmetric: left_overhang={left_overhang}, right_overhang={right_overhang}, \
         axis=[{axis_left_x}..{axis_right_x}], bars=[{bar_left_x}..{bar_right_x}]"
    );
}
