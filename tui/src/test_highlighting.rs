#[cfg(test)]
mod tests {
    use crate::app::{App, Pane};
    use crate::table_state::TableFocusMode;
    use crate::network::JobMetrics;
    use crate::action::Action;
    use ratatui::{
        backend::TestBackend,
        Terminal,
        style::{Modifier, Style, Color},
    };
    use crate::ui;
    use crate::theme::SAPPHIRE;

    #[test]
    fn should_only_highlight_selected_cell_not_row() {
        let mut app = App::new();
        let now = chrono::Utc::now().to_rfc3339();
        let jobs = vec![
            JobMetrics {
                started_at: now,
                program_name: "job1".to_string(),
                user_name: "user".to_string(),
                wall_time_ms: 100,
                cpu_time_sec: 0.1,
                cpu_percent: 10.0,
                max_rss_kb: 1000,
                exit_code_int: 0,
            },
        ];
        app.window = crate::app::TimeWindow::WMax;
        app.update(Action::SetJobs(jobs));
        app.focused_pane = Pane::Jobs;
        app.jobs_table_state.focus_mode = TableFocusMode::Cell;
        app.jobs_table_state.row_state.select(Some(0));
        app.jobs_table_state.selected_col = Some(0); // Focus first cell

        let backend = TestBackend::new(100, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            ui::draw(f, &mut app);
        }).unwrap();

        let buffer = terminal.backend().buffer();
        
        // Find where "job1" is rendered. It should be highlighted.
        // And find another cell in the same row, e.g. "1" (the count). It should NOT be highlighted with REVERSED.
        
        let mut job1_found = false;
        let mut other_cell_highlighted = false;

        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                let cell = buffer.get(x, y);
                if cell.symbol() == "j" && buffer.get(x+1, y).symbol() == "o" && buffer.get(x+2, y).symbol() == "b" {
                    job1_found = true;
                    // Check if 'j' is highlighted
                    assert!(cell.style().add_modifier.contains(Modifier::REVERSED), "Focused cell should be REVERSED");
                    assert_eq!(cell.style().fg, Some(SAPPHIRE));
                    
                    // Now check another cell in the same row.
                    // The count "1" should be in the same row (y).
                    // Let's look for "1" in this row.
                    let row_str: String = (0..buffer.area.width).map(|ix| buffer.get(ix, y).symbol()).collect();
                    println!("Row {}: {}", y, row_str);
                    
                    for ix in 0..buffer.area.width {
                        let c = buffer.get(ix, y);
                        // Skip the "job1" part
                        if ix >= x && ix < x + 4 { continue; }
                        
                        if c.symbol() == "1" {
                             if c.style().add_modifier.contains(Modifier::REVERSED) {
                                 other_cell_highlighted = true;
                             }
                        }
                    }
                }
            }
        }
        
        assert!(job1_found, "job1 not found");
        assert!(!other_cell_highlighted, "Other cells in the row should not be highlighted when in Cell focus mode");
    }
}
