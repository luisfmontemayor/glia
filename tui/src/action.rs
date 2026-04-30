use crossterm::event::KeyEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Tick,
    Quit,
    Render,
    Key(KeyEvent),
    NextWindow,
    NextMetric,
    PreviousMetric,
    FetchMetrics,
}
