#[cfg(test)]
mod tests {
    use crate::app::{App, Pane};
    use crate::table_state::TableFocusMode;
    use crate::network::JobMetrics;
    use crate::action::Action;
    use ratatui::{
        backend::TestBackend,
        Terminal,
    };
    use crate::ui;

    #[test]
    fn should_expand_selected_cell() {
        let mut app = App::new();
        let long_name = "very_long_job_name_that_should_normally_be_truncated_but_expanded_when_selected".to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let jobs = vec![
            JobMetrics {
                started_at: now,
                program_name: long_name.clone(),
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
        assert_eq!(app.summaries.len(), 1, "Summaries should be populated");
        app.focused_pane = Pane::Jobs;
        app.jobs_table_state.focus_mode = TableFocusMode::Cell;
        app.jobs_table_state.row_state.select(Some(0));
        app.jobs_table_state.selected_col = Some(0);

        let backend = TestBackend::new(200, 40);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            ui::draw(f, &mut app);
        }).unwrap();

        let buffer = terminal.backend().buffer();
        
        // Check if the long name is present in the buffer
        let mut found = false;
        let mut out = String::new();
        for y in 0..buffer.area.height {
            let mut line = String::new();
            for x in 0..buffer.area.width {
                line.push_str(buffer.get(x, y).symbol());
            }
            out.push_str(&line);
            out.push('\n');
            if line.contains(&long_name) {
                found = true;
            }
        }
        std::fs::write("buffer_dump.txt", out).unwrap();
        
        assert!(found, "Long job name not found in buffer: {}", long_name);
    }

    #[test]
    fn should_expand_selected_column() {
        let mut app = App::new();
        let long_name = "very_long_job_name_that_should_normally_be_truncated_but_expanded_when_column_focused".to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let jobs = vec![
            JobMetrics {
                started_at: now,
                program_name: long_name.clone(),
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
        assert_eq!(app.summaries.len(), 1, "Summaries should be populated");
        app.focused_pane = Pane::Jobs;
        app.jobs_table_state.focus_mode = TableFocusMode::Column;
        app.jobs_table_state.selected_col = Some(0);

        let backend = TestBackend::new(200, 40);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal.draw(|f| {
            ui::draw(f, &mut app);
        }).unwrap();

        let buffer = terminal.backend().buffer();
        
        // Check if the long name is present in the buffer
        let mut found = false;
        for y in 0..buffer.area.height {
            let mut line = String::new();
            for x in 0..buffer.area.width {
                line.push_str(buffer.get(x, y).symbol());
            }
            println!("Line {}: {}", y, line);
            if line.contains(&long_name) {
                found = true;
            }
        }
        
        assert!(found, "Long job name not found in buffer when column focused: {}", long_name);
    }

    #[test]
    fn should_only_highlight_selected_cell_not_row() {
        use ratatui::style::Modifier;
        use crate::theme::SAPPHIRE;
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
        
        let mut job1_found = false;
        let mut other_cell_highlighted = false;

        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                let cell = buffer.get(x, y);
                if cell.symbol() == "j" && buffer.get(x+1, y).symbol() == "o" && buffer.get(x+2, y).symbol() == "b" {
                    job1_found = true;
                    assert!(cell.style().add_modifier.contains(Modifier::REVERSED), "Focused cell should be REVERSED");
                    assert_eq!(cell.style().fg, Some(SAPPHIRE));
                    
                    for ix in 0..buffer.area.width {
                        let c = buffer.get(ix, y);
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
