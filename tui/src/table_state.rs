use ratatui::widgets::TableState;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum TableFocusMode {
    #[default]
    Row,
    Cell,
}

#[derive(Debug, Clone, Default)]
pub struct JobsTableState {
    pub row_state: TableState,
    pub selected_col: Option<usize>,
    pub focus_mode: TableFocusMode,
    pub search_query: String,
    pub is_searching: bool,
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
