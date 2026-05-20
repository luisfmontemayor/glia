use ratatui::{Terminal, backend::TestBackend};
use tui::app::{App, TimeWindow};
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

    let mut found_09_32 = false;
    let mut found_10_02 = false;
    let mut found_10_32 = false;

    for x in 0..100 {
        for y in 0..50 {
            let cell = buffer.get(x, y);
            let s = cell.symbol();
            if s == "0" || s == "1" {
                let mut label = String::new();
                for dx in 0..5 {
                    if x + dx < 100 {
                        label.push_str(buffer.get(x + dx, y).symbol());
                    }
                }
                if label == "09:32" {
                    found_09_32 = true;
                }
                if label == "10:02" {
                    found_10_02 = true;
                }
                if label == "10:32" {
                    found_10_32 = true;
                }
            }
        }
    }

    assert!(!found_09_32, "Should NOT find 09:32 label (min) anymore");
    assert!(found_10_02, "Should find 10:02 label (mid)");
    assert!(!found_10_32, "Should NOT find 10:32 label (max) anymore");

    // Check for the manual "0" label
    let mut found_zero = false;
    for x in 0..100 {
        for y in 0..50 {
            if buffer.get(x, y).symbol() == "0" {
                // The manual "0" should be isolated
                let left_is_empty = if x > 0 { buffer.get(x-1, y).symbol() == " " } else { true };
                let right_is_empty = if x < 99 { buffer.get(x+1, y).symbol() == " " } else { true };
                if left_is_empty && right_is_empty {
                    found_zero = true;
                }
            }
        }
    }
    assert!(found_zero, "Should find the manual '0' label");

    // Check for the max Y label (200)
    let mut found_max_y = false;
    for x in 0..100 {
        for y in 0..50 {
            let mut label = String::new();
            for dx in 0..3 {
                if x + dx < 100 {
                    label.push_str(buffer.get(x + dx, y).symbol());
                }
            }
            if label == "200" {
                found_max_y = true;
            }
        }
    }
    assert!(found_max_y, "Should find the max Y label '200'");
}

#[test]
fn test_blame_chart_no_value_title() {
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

    // Scan the y-axis area (left side, x=0..15) for "Value"
    let mut found_value = false;
    for y in 0..50 {
        let mut row = String::new();
        for x in 0..15 {
            row.push_str(buffer.get(x, y).symbol());
        }
        if row.contains("Value") {
            found_value = true;
            break;
        }
    }

    assert!(
        !found_value,
        "Y-axis should NOT contain 'Value' title in blame mode"
    );
}

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

#[test]
fn test_zero_value_bar_height() {
    let mut app = App::new();
    app.blame_mode = false;
    app.jobs = (0..9).map(|i| JobMetrics {
            started_at: format!("2023-10-27T10:0{}:00Z", i),
            program_name: format!("job{}", i),
            user_name: "alice".to_string(),
            wall_time_ms: 0, // Zero value
            cpu_time_sec: 0.0,
            cpu_percent: 0.0,
            max_rss_kb: 0,
            exit_code_int: 0,
    }).collect();

    // Width 100, Height 50. 
    // Graph area is roughly half width (50).
    // b_width is 5.
    // expected height = b_width / 2 = 2.5 columns.
    // In terminal terms, 1 row is roughly 2 columns wide.
    // So 2.5 columns height is roughly 1.25 rows.
    let width = 100;
    let height = 50;
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    
    // Print buffer for debugging
    for y in 0..height {
        let mut line = String::new();
        for x in 0..width {
            line.push_str(buffer.get(x, y).symbol());
        }
        if line.chars().any(|c| !c.is_whitespace()) {
            println!("{:2}: {}", y, line);
        }
    }
    
    // Find the axis line
    let mut axis_y = None;
    for y in (0..height).rev() {
        let mut row_str = String::new();
        for x in 0..width {
            row_str.push_str(buffer.get(x, y).symbol());
        }
        if row_str.contains("┬") {
            axis_y = Some(y);
            break;
        }
    }
    
    let axis_y = axis_y.expect("Could not find axis line");
    let chart_row_bottom = axis_y - 1;
    
    // Check if there's any non-empty character in the chart area (above the axis)
    let mut has_bar = false;
    for y in 0..axis_y {
        for x in 0..width {
            let sym = buffer.get(x, y).symbol();
            // Symbols for BarChart (NINE_LEVELS)
            if sym == "█" || sym == "▄" || sym == "▅" || sym == "▂" || sym == "▃" || sym == "▆" || sym == "▇" || sym == " " {
                if sym != " " && !sym.is_empty() && sym != "│" && sym != "┬" && sym != "─" && sym != "┌" && sym != "┐" && sym != "└" && sym != "┘" {
                    has_bar = true;
                    break;
                }
            }
        }
        if has_bar { break; }
    }
    
    assert!(has_bar, "Zero value bar should be visible somewhere above axis");
    
    // Check how many rows the bar occupies
    let mut bar_rows = 0;
    for y in (0..axis_y).rev() {
        let mut row_has_bar = false;
        for x in 0..width {
            let sym = buffer.get(x, y).symbol();
            if sym == "█" || sym == "▄" || sym == "▅" || sym == "▂" || sym == "▇" || sym == "▆" || sym == "▃" || sym == " " {
                // Wait, BarChart might use different symbols. 
                // But symbols::bar::NINE_LEVELS are: " ", "▂", "▃", "▄", "▅", "▆", "▇", "█"
                if sym != " " && !sym.is_empty() && sym != "│" && sym != "┬" && sym != "─" {
                     row_has_bar = true;
                     break;
                }
            }
        }
        if row_has_bar {
            bar_rows += 1;
        } else {
            if bar_rows > 0 { break; } // End of bar
        }
    }
    
    println!("Bar rows: {}", bar_rows);
    let expected_rows = (2 / 2).max(1);
    assert_eq!(bar_rows, expected_rows, "Zero value bar should occupy exactly {} rows based on 1:2 aspect ratio, but found {}", expected_rows, bar_rows);

    // Check if the value "0" is rendered somewhere in the chart area
    let mut found_zero = false;
    for y in 0..axis_y {
        let mut row_str = String::new();
        for x in 0..width {
            row_str.push_str(buffer.get(x, y).symbol());
        }
        if row_str.contains('0') {
            found_zero = true;
            break;
        }
    }
    assert!(found_zero, "Value '0' should be rendered on the 0-value bar");
}

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
fn test_command_palette_wrapping_no_leading_pipe() {
    let mut app = App::new();
    app.is_loading = false;
    app.show_command_palette = true;

    // Set a small terminal size to force wrapping
    let backend = TestBackend::new(60, 30);
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

    // Print content for visual verification in logs
    println!("PALETTE WRAPPING BUFFER:\n{}", content);

    // Check that no line in the command palette block starts with a pipe separator character.
    // The box drawing vertical line is U+2502 (│), whereas the pipe separator is U+007C (|).
    // We assert that the separator | is not rendered immediately after a left border (│) followed by optional spaces.
    assert!(!content.contains("│ |"), "Command palette line started with a leading pipe separator");
    assert!(!content.contains("│  |"), "Command palette line started with a leading pipe separator after spaces");
}
