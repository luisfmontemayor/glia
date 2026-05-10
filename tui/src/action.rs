use crossterm::event::KeyEvent;
use crate::network::JobMetrics;
use crate::app::Pane;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Tick,
    Quit,
    Render,
    Key(KeyEvent),
    NextWindow,
    NextMetric,
    PreviousMetric,
    NextRow,
    PreviousRow,
    FetchMetrics,
    SetJobs(Vec<JobMetrics>),
    FetchError(String),
    ToggleDetail,
    ToggleBlameMode,
    UpdateHealth(bool, bool),
    FocusPane(Pane),
    TableFocusRow,
    TableFocusCell,
    TableFocusCol,
    TableNextCol,
    TablePrevCol,
    TableSearch(String),
    TableEndSearch,
    TableChar(char),
    TableBackspace,
    TableSort,
}
