use ratatui::{Terminal, backend::TestBackend};
use tui::action::Action;
use tui::app::{App, TimeWindow};
use tui::network::JobMetrics;
use tui::ui;
use tui::components::table::table_state::TableFocusMode;

#[test]
fn test_app_set_jobs_action() {
    let mut app = App::new();
    app.window = TimeWindow::WMax;
    let new_jobs = vec![JobMetrics {
        started_at: "2023-10-27T12:00:00Z".to_string(),
        program_name: "new_job".to_string(),
        user_name: "charles".to_string(),
        wall_time_ms: 500,
        cpu_time_sec: 0.5,
        cpu_percent: 50.0,
        max_rss_kb: 5120,
        exit_code_int: 0,
    }];

    app.update(Action::SetJobs(new_jobs.clone()));

    assert_eq!(app.jobs.len(), 1);
    assert_eq!(app.jobs[0].program_name, "new_job");
}

#[test]
fn test_set_jobs_auto_increases_window() {
    let mut app = App::new();
    app.window = TimeWindow::W1h;

    let now = chrono::Utc::now();
    let old_time = now - chrono::Duration::hours(2);
    let job = JobMetrics {
        started_at: old_time.to_rfc3339(),
        program_name: "old_job".to_string(),
        user_name: "user".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    };

    app.update(Action::SetJobs(vec![job]));

    assert_eq!(app.window, TimeWindow::W3h);
    assert_eq!(app.jobs.len(), 1);
}

#[test]
fn should_display_no_data_message() {
    let mut app = App::new();
    app.jobs = vec![];
    app.window = TimeWindow::W1h;
    app.has_user_changed_window = true;
    app.is_loading = false;

    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            ui::draw(f, &mut app);
        })
        .unwrap();

    let buffer = terminal.backend().buffer();
    let expected_message = "No data available for time window.";

    let mut found = false;
    for y in 0..buffer.area.height {
        let mut line = String::new();
        for x in 0..buffer.area.width {
            line.push_str(buffer.get(x, y).symbol());
        }
        if line.contains(expected_message) {
            found = true;
            break;
        }
    }

    assert!(
        found,
        "Message '{}' not found in the output",
        expected_message
    );
}

#[test]
fn should_autocycle_on_initial_load() {
    let mut app = App::new();
    let old_job = JobMetrics {
        started_at: (chrono::Utc::now() - chrono::Duration::hours(10)).to_rfc3339(),
        program_name: "old_job".to_string(),
        user_name: "user".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    };

    assert_eq!(app.window, TimeWindow::W1h);
    assert!(!app.has_user_changed_window);

    app.update(Action::SetJobs(vec![old_job]));

    assert_eq!(app.window, TimeWindow::W12h);
}

#[test]
fn should_not_autocycle_after_manual_change() {
    let mut app = App::new();
    let old_job = JobMetrics {
        started_at: (chrono::Utc::now() - chrono::Duration::hours(12)).to_rfc3339(),
        program_name: "old_job".to_string(),
        user_name: "user".to_string(),
        wall_time_ms: 100,
        cpu_time_sec: 0.1,
        cpu_percent: 10.0,
        max_rss_kb: 1000,
        exit_code_int: 0,
    };

    app.has_user_changed_window = true;
    app.window = TimeWindow::W1h;

    app.update(Action::SetJobs(vec![old_job]));

    assert_eq!(app.window, TimeWindow::W1h);
    assert!(app.jobs.is_empty());
}

#[test]
fn test_r_in_column_mode_selects_row() {
    let mut app = App::new();
    app.jobs_table_state.focus_mode = TableFocusMode::Column;
    app.update(Action::TableFocusRow);
    assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Row);
}

#[test]
fn test_c_in_row_mode_selects_column() {
    let mut app = App::new();
    app.jobs_table_state.focus_mode = TableFocusMode::Row;
    app.update(Action::TableFocusCol);
    assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Column);
    assert_eq!(app.jobs_table_state.selected_col, Some(0));
}

#[test]
fn test_enter_in_column_mode_selects_cell() {
    let mut app = App::new();
    app.jobs_table_state.focus_mode = TableFocusMode::Column;
    app.update(Action::TableFocusCell);
    assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Cell);
}

#[test]
fn test_c_in_row_mode_resets_selected_column_to_first() {
    let mut app = App::new();
    app.jobs_table_state.focus_mode = TableFocusMode::Row;
    app.jobs_table_state.selected_col = Some(2); // simulated previous column selection
    app.update(Action::TableFocusCol);
    assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Column);
    assert_eq!(app.jobs_table_state.selected_col, Some(0)); // should reset to first column
}

#[test]
fn test_c_in_cell_mode_preserves_selected_column() {
    let mut app = App::new();
    app.jobs_table_state.focus_mode = TableFocusMode::Cell;
    app.jobs_table_state.selected_col = Some(2); 
    app.update(Action::TableFocusCol);
    assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Column);
    assert_eq!(app.jobs_table_state.selected_col, Some(2)); // should preserve
}

#[test]
fn test_reverse_time_window_cycling() {
    let mut app = App::new();
    // Initial window should be W1h
    assert_eq!(app.window, TimeWindow::W1h);
    
    // Cycle backwards: W1h -> WMax -> W24h -> W12h -> W6h -> W3h -> W1h
    app.update(Action::PrevTimeWindow);
    assert_eq!(app.window, TimeWindow::WMax);
    assert!(app.fetch_requested);
    app.fetch_requested = false;

    app.update(Action::PrevTimeWindow);
    assert_eq!(app.window, TimeWindow::W24h);
    
    app.update(Action::PrevTimeWindow);
    assert_eq!(app.window, TimeWindow::W12h);

    app.update(Action::PrevTimeWindow);
    assert_eq!(app.window, TimeWindow::W6h);

    app.update(Action::PrevTimeWindow);
    assert_eq!(app.window, TimeWindow::W3h);

    app.update(Action::PrevTimeWindow);
    assert_eq!(app.window, TimeWindow::W1h);
}
