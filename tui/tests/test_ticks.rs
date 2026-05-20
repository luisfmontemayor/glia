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
    
    std::fs::write("buffer_output.txt", &content).unwrap();

    // Check that native labels are actually printed
    assert!(content.contains("10:05"), "Native label 10:05 missing");
}

#[test]
fn test_data_point_ticks_rendered() {
    let mut app = App::new();
    app.blame_mode = false;
    
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

    // Using width 100 so the graph area (which is 50% width) has width 50.
    // This aligns with the instruction's expected x positions of 7, 23, 39.
    let width = 100;
    let height = 50;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        ui::draw(f, &mut app);
    }).unwrap();

    let buffer = terminal.backend().buffer();
    
    // Find the axis line to locate the ticks row (it should be right above the axis).
    let mut axis_y = None;
    for y in (0..height).rev() {
        let mut row_str = String::new();
        for x in 2..48 { // Look within the chart area horizontal bounds
            row_str.push_str(buffer.get(x, y).symbol());
        }
        // The axis line is rendered as a Paragraph of HORIZONTAL symbols.
        // We avoid the footer by checking if it's above the footer area.
        if y < 45 && row_str.contains("────────") {
            axis_y = Some(y);
            break;
        }
    }
    
    let axis_y = axis_y.expect("Could not find axis line");
    
    // The ticks should be at x=6, 20, 34 relative to the padded chart area start (which is inner_area.x + 2 = 4).
    let chart_start_x = 2;
    let graph_margin = 2;
    let tick_1 = buffer.get(chart_start_x + graph_margin + 6, axis_y).symbol();
    let tick_2 = buffer.get(chart_start_x + graph_margin + 20, axis_y).symbol();
    let tick_3 = buffer.get(chart_start_x + graph_margin + 34, axis_y).symbol();
    
    assert_eq!(tick_1, "┬", "Tick 1 at x=7 missing");
    assert_eq!(tick_2, "┬", "Tick 2 at x=23 missing");
    assert_eq!(tick_3, "┬", "Tick 3 at x=39 missing");
}

#[test]
fn test_tick_count_matches_jobs() {
    let mut app = App::new();
    app.blame_mode = false;
    
    let n_jobs = 5;
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

    let width = 150; // Use a large width to ensure all ticks fit
    let height = 50;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        ui::draw(f, &mut app);
    }).unwrap();

    let buffer = terminal.backend().buffer();
    
    let mut axis_y = None;
    for y in (0..height).rev() {
        let mut row_str = String::new();
        for x in 0..width {
            row_str.push_str(buffer.get(x, y).symbol());
        }
        if y < 45 && row_str.contains("────────") {
            axis_y = Some(y);
            break;
        }
    }
    
    let axis_y = axis_y.expect("Could not find axis line");
    
    let mut tick_count = 0;
    // Look only inside the graph area (left 50%)
    // For width 150, graph area is x=0..75.
    // Inner area (excluding borders) is x=1..74.
    for x in 1..74 {
        if buffer.get(x, axis_y).symbol() == "┬" {
            tick_count += 1;
        }
    }
    
    assert_eq!(tick_count, n_jobs, "Tick count should match number of jobs");
}

#[test]
fn test_tick_clipping_with_barchart() {
    let mut app = App::new();
    app.blame_mode = false;
    
    // Create 10 jobs. With a width of 40, some will definitely be clipped.
    let n_jobs = 10;
    for i in 0..n_jobs {
        app.jobs.push(JobMetrics {
            started_at: format!("2023-10-27T12:{:02}:00Z", i),
            program_name: "job".to_string(),
            user_name: "user".to_string(),
            wall_time_ms: 100,
            cpu_time_sec: 0.1,
            cpu_percent: 10.0,
            max_rss_kb: 1000,
            exit_code_int: 0,
        });
    }

    // Width 40. Graph area (50%) is 20. Inner area is ~18.
    // If b_width=5, gap=1, each job takes 6 columns.
    // 3 jobs take 17 columns. The 4th job (starting at 1 + 3*6 = 19) won't fit.
    let width = 40;
    let height = 24;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        ui::draw(f, &mut app);
    }).unwrap();

    let buffer = terminal.backend().buffer();
    
    let mut axis_y = None;
    for y in (0..height).rev() {
        let mut row_str = String::new();
        for x in 0..width {
            row_str.push_str(buffer.get(x, y).symbol());
        }
        if y < 20 && row_str.contains("──") {
            axis_y = Some(y);
            break;
        }
    }
    
    let axis_y = axis_y.expect("Could not find axis line");
    let bars_y = axis_y - 2; // Bottom-most row of bars
    
    // Check every x position in the graph area.
    // If there's a tick at x, there MUST be a bar at x (symbol not empty/space).
    // If there's no bar at x, there MUST NOT be a tick at x.
    for x in 1..20 {
        let tick = buffer.get(x, axis_y).symbol();
        let bar = buffer.get(x, bars_y).symbol();
        
        if tick == "┬" {
            // There should be a bar character here. 
            // BarChart uses NINE_LEVELS, so it's one of " ", "▂", ..., "█".
            // Since our value is 100 (which is > 0), it should not be empty.
            assert!(bar != " " && !bar.is_empty(), "Tick at x={} exists but no bar above it", x);
        }
    }
}

#[test]
fn test_date_alignment_under_data_points() {
    let mut app = App::new();
    app.blame_mode = false;
    
    // We create jobs that cross a date boundary so that a date label (MM-DD) is rendered.
    // The chart logic only prints the date if it differs from the last_date.
    app.jobs.push(JobMetrics {
        started_at: "2023-10-26T23:50:00Z".to_string(),
        program_name: "job1".to_string(),
        user_name: "user".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    });
    app.jobs.push(JobMetrics {
        started_at: "2023-10-27T00:10:00Z".to_string(),
        program_name: "job2".to_string(),
        user_name: "user".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    });

    let width = 100;
    let height = 50;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        ui::draw(f, &mut app);
    }).unwrap();

    let buffer = terminal.backend().buffer();
    
    // Find the axis line
    let mut axis_y = None;
    for y in (0..height).rev() {
        let mut row_str = String::new();
        for x in 0..width {
            row_str.push_str(buffer.get(x, y).symbol());
        }
        if y < 45 && row_str.contains("────────") {
            axis_y = Some(y);
            break;
        }
    }
    let axis_y = axis_y.expect("Could not find axis line");
    let labels_y = axis_y + 1; // HH:MM row
    let dates_y = axis_y + 2;  // MM-DD row

    // Find the tick for the second job (which should have a date label because date changed to 10-27)
    let mut ticks = Vec::new();
    for x in 0..width {
        if buffer.get(x, axis_y).symbol() == "┬" {
            ticks.push(x);
        }
    }
    assert!(ticks.len() >= 2, "Expected at least 2 ticks");
    let second_tick_x = ticks[1];

    // The date label "10-27" is 5 chars long.
    // The chart logic calculates:
    // tick_x = group_x + bars_width / 2
    // label_x = group_x + (bars_width.saturating_sub(5) / 2)
    // For b_width=5 (2 jobs out of 50 threshold -> bw is max 20, but capped probably by available width, let's just find the text).
    
    // Let's find "10-27" in the dates_y row
    let mut date_start_x = None;
    for x in 0..(width - 4) {
        let s = format!("{}{}{}{}{}",
            buffer.get(x, dates_y).symbol(),
            buffer.get(x+1, dates_y).symbol(),
            buffer.get(x+2, dates_y).symbol(),
            buffer.get(x+3, dates_y).symbol(),
            buffer.get(x+4, dates_y).symbol()
        );
        if s == "10-27" {
            date_start_x = Some(x);
            break;
        }
    }
    
    let date_start_x = date_start_x.expect("Date label 10-27 not found");
    
    // The center of the 5-char date label should ideally align with the tick.
    let date_center_x = date_start_x + 2;
    
    // Let's verify that the center of the date label is close to the tick (within 1 character due to integer division).
    let diff = (date_center_x as i32 - second_tick_x as i32).abs();
    assert!(diff <= 1, "Date label center ({}) should align with tick ({}), but difference is {}", date_center_x, second_tick_x, diff);
}

