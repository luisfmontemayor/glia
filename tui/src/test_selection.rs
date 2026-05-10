#[cfg(test)]
mod tests {
    use crate::app::App;
    use crate::action::Action;
    use crate::table_state::TableFocusMode;

    #[test]
    fn should_select_cell_on_enter() {
        let mut app = App::new();
        // Ensure we start in Row mode
        app.jobs_table_state.focus_mode = TableFocusMode::Row;
        
        // Dispatch Action::TableFocusCell
        app.update(Action::TableFocusCell);
        
        // Assert that app.focus_mode is now TableFocusMode::Cell
        // Note: focus_mode is in app.jobs_table_state.focus_mode
        assert_eq!(app.jobs_table_state.focus_mode, TableFocusMode::Cell);
    }
}
