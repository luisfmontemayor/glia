use crossterm::event::KeyEvent;
use crate::network::JobMetrics;

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
}
