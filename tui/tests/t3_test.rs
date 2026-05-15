use tui::app::{App, TimeWindow};
use tui::action::Action;

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
