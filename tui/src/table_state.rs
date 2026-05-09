use ratatui::widgets::TableState;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum TableFocusMode {
    #[default]
    Row,
    Cell,
}

#[derive(Debug, Clone)]
pub struct JobsTableState {
    pub row_state: TableState,
    pub selected_col: Option<usize>,
    pub focus_mode: TableFocusMode,
    pub search_query: String,
    pub is_searching: bool,
    pub sort_col: Option<usize>,
    pub sort_desc: bool,
}

impl Default for JobsTableState {
    fn default() -> Self {
        Self {
            row_state: TableState::default(),
            selected_col: None,
            focus_mode: TableFocusMode::default(),
            search_query: String::new(),
            is_searching: false,
            sort_col: None,
            sort_desc: true,
        }
    }
}

impl JobsTableState {
    pub fn next_col(&mut self, max_cols: usize) {
        if max_cols == 0 {
            self.selected_col = None;
            return;
        }
        self.selected_col = match self.selected_col {
            Some(i) => Some((i + 1).min(max_cols - 1)),
            None => Some(0),
        };
    }

    pub fn prev_col(&mut self) {
        self.selected_col = match self.selected_col {
            Some(i) => Some(i.saturating_sub(1)),
            None => Some(0),
        };
    }

    pub fn jump_top(&mut self) {
        self.row_state.select(Some(0));
    }

    pub fn jump_bottom(&mut self, row_count: usize) {
        if row_count > 0 {
            self.row_state.select(Some(row_count - 1));
        } else {
            self.row_state.select(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_navigation() {
        let mut state = JobsTableState::default();
        assert_eq!(state.selected_col, None);

        state.next_col(3);
        assert_eq!(state.selected_col, Some(0));

        state.next_col(3);
        assert_eq!(state.selected_col, Some(1));

        state.next_col(3);
        assert_eq!(state.selected_col, Some(2));

        state.next_col(3);
        assert_eq!(state.selected_col, Some(2)); // Should stop at max_cols - 1

        state.prev_col();
        assert_eq!(state.selected_col, Some(1));

        state.prev_col();
        assert_eq!(state.selected_col, Some(0));

        state.prev_col();
        assert_eq!(state.selected_col, Some(0)); // Should stop at 0
    }

    #[test]
    fn test_empty_cols() {
        let mut state = JobsTableState::default();
        state.next_col(0);
        assert_eq!(state.selected_col, None);
    }
}
