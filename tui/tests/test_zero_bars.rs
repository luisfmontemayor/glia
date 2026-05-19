use ratatui::{Terminal, backend::TestBackend};
use tui::app::App;
use tui::network::JobMetrics;
use tui::ui;

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
    assert!(bar_rows <= 2, "Zero value bar should occupy at most 2 rows, but found {}", bar_rows);

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
