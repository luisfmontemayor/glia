use crate::app::Pane;
use crate::network::JobMetrics;
use crossterm::event::KeyEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Tick,
    Quit,
    Render,
    Key(KeyEvent),
    NextTimeWindow,
    PrevTimeWindow,
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
    ToggleCommandPalette,
}
