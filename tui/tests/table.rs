use ratatui::{Terminal, backend::TestBackend, style::Modifier};
use tui::action::Action;
use tui::app::JobSummary;
use tui::app::{App, Pane};
use tui::network::JobMetrics;
use tui::table_state::TableFocusMode;
use tui::theme::{DARK_BLUE, SAPPHIRE};
use tui::ui;
use tui::ui::render_top_scripts_table;

#[test]
fn should_select_column_state_transition() {
    let mut app = App::new();
    // Ensure we start in Row mode
    app.jobs_table_state.focus_mode = TableFocusMode::Row;

    // Dispatch Action::TableFocusCol
    app.update(Action::TableFocusCol);

    // Assert that app.focus_mode is now TableFocusMode::Column
    assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Column);
}

#[test]
fn should_highlight_column_header_in_column_mode() {
    let mut app = App::new();
    app.summaries = vec![JobSummary {
        program_name: "test1".to_string(),
        count: 10,
        avg_wall_time_ms: 100,
        total_cpu_time_sec: 1.0,
        max_rss_kb: 1000,
    }];
    app.jobs_table_state.focus_mode = TableFocusMode::Column;
    app.jobs_table_state.selected_col = Some(0); // "Name" column

    let backend = TestBackend::new(100, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = f.size();
            render_top_scripts_table(f, &mut app, area);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();

    let mut header_highlighted = false;
    for y in 0..5 {
        for x in 0..100 {
            let cell = buffer.get(x, y);
            // The header for the first column is "Name"
            if cell.symbol() == "N" && buffer.get(x + 1, y).symbol() == "a" {
                if cell.style().fg == Some(DARK_BLUE)
                    && cell.style().add_modifier.contains(Modifier::REVERSED)
                    && cell.style().add_modifier.contains(Modifier::BOLD)
                {
                    header_highlighted = true;
                }
            }
        }
    }

    assert!(
        header_highlighted,
        "Column header was not highlighted correctly in column mode"
    );
}

#[test]
fn should_select_cell_on_enter() {
    let mut app = App::new();
    // Ensure we start in Row mode
    app.jobs_table_state.focus_mode = TableFocusMode::Row;

    // Dispatch Action::TableFocusCell
    app.update(Action::TableFocusCell);

    // Assert that app.focus_mode is now TableFocusMode::Cell
    assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Cell);
}

#[test]
fn should_highlight_column_in_column_mode() {
    let mut app = App::new();
    app.summaries = vec![
        JobSummary {
            program_name: "test1".to_string(),
            count: 10,
            avg_wall_time_ms: 100,
            total_cpu_time_sec: 1.0,
            max_rss_kb: 1000,
        },
        JobSummary {
            program_name: "test2".to_string(),
            count: 20,
            avg_wall_time_ms: 200,
            total_cpu_time_sec: 2.0,
            max_rss_kb: 2000,
        },
    ];
    app.jobs_table_state.focus_mode = TableFocusMode::Column;
    app.jobs_table_state.selected_col = Some(1); // "Uses" column

    let backend = TestBackend::new(100, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let area = f.size();
            render_top_scripts_table(f, &mut app, area);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();

    let mut highlighted_cells = Vec::new();
    for y in 0..20 {
        for x in 0..100 {
            let cell = buffer.get(x, y);
            if cell.style().bg == Some(SAPPHIRE) {
                highlighted_cells.push((x, y, cell.symbol().to_string()));
            }
        }
    }

    assert!(
        highlighted_cells.len() > 0,
        "No cells were highlighted in column mode"
    );
}

#[test]
fn should_expand_selected_cell() {
    let mut app = App::new();
    let long_name =
        "very_long_job_name_that_should_normally_be_truncated_but_expanded_when_selected"
            .to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let jobs = vec![JobMetrics {
        started_at: now,
        program_name: long_name.clone(),
        user_name: "user".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    }];
    app.window = tui::app::TimeWindow::WMax;
    app.update(Action::SetJobs(jobs));
    assert_eq!(app.summaries.len(), 1, "Summaries should be populated");
    app.focused_pane = Pane::Jobs;
    app.jobs_table_state.focus_mode = TableFocusMode::Cell;
    app.jobs_table_state.row_state.select(Some(0));
    app.jobs_table_state.selected_col = Some(0);

    let backend = TestBackend::new(200, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();

    let mut found = false;
    for y in 0..buffer.area.height {
        let mut line = String::new();
        for x in 0..buffer.area.width {
            line.push_str(buffer.get(x, y).symbol());
        }
        if line.contains(&long_name) {
            found = true;
        }
    }

    assert!(found, "Long job name not found in buffer: {}", long_name);
}

#[test]
fn should_expand_selected_column() {
    let mut app = App::new();
    let long_name =
        "very_long_job_name_that_should_normally_be_truncated_but_expanded_when_column_focused"
            .to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let jobs = vec![JobMetrics {
        started_at: now,
        program_name: long_name.clone(),
        user_name: "user".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    }];
    app.window = tui::app::TimeWindow::WMax;
    app.update(Action::SetJobs(jobs));
    assert_eq!(app.summaries.len(), 1, "Summaries should be populated");
    app.focused_pane = Pane::Jobs;
    app.jobs_table_state.focus_mode = TableFocusMode::Column;
    app.jobs_table_state.selected_col = Some(0);

    let backend = TestBackend::new(200, 40);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();

    let mut found = false;
    for y in 0..buffer.area.height {
        let mut line = String::new();
        for x in 0..buffer.area.width {
            line.push_str(buffer.get(x, y).symbol());
        }
        if line.contains(&long_name) {
            found = true;
        }
    }

    assert!(
        found,
        "Long job name not found in buffer when column focused: {}",
        long_name
    );
}

#[test]
fn should_only_highlight_selected_cell_not_row() {
    let mut app = App::new();
    let now = chrono::Utc::now().to_rfc3339();
    let jobs = vec![JobMetrics {
        started_at: now,
        program_name: "job1".to_string(),
        user_name: "user".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    }];
    app.window = tui::app::TimeWindow::WMax;
    app.update(Action::SetJobs(jobs));
    app.focused_pane = Pane::Jobs;
    app.jobs_table_state.focus_mode = TableFocusMode::Cell;
    app.jobs_table_state.row_state.select(Some(0));
    app.jobs_table_state.selected_col = Some(0); // Focus first cell

    let backend = TestBackend::new(100, 20);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();

    let mut job1_found = false;
    let mut other_cell_highlighted = false;

    for y in 0..buffer.area.height {
        for x in 0..buffer.area.width {
            let cell = buffer.get(x, y);
            if cell.symbol() == "j"
                && buffer.get(x + 1, y).symbol() == "o"
                && buffer.get(x + 2, y).symbol() == "b"
            {
                job1_found = true;
                assert!(
                    !cell.style().add_modifier.contains(Modifier::REVERSED),
                    "Focused cell should NOT be REVERSED"
                );

                for ix in 0..buffer.area.width {
                    let c = buffer.get(ix, y);
                    if ix >= x && ix < x + 4 {
                        continue;
                    }

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
    assert!(
        !other_cell_highlighted,
        "Other cells in the row should not be highlighted when in Cell focus mode"
    );
}
