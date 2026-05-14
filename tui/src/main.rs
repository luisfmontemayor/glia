use tui::action::Action;
use tui::app::App;
use tui::app::Pane;
use tui::app::TimeWindow;
use tui::components::table::table_state::TableFocusMode;
use tui::ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{error::Error, io};
use tokio::sync::mpsc;

type Backend = CrosstermBackend<io::Stdout>;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup panic hook to restore terminal on crash
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        panic_hook(panic_info);
    }));

    let mut terminal = setup_terminal()?;
    let app = App::new();
    let res = run_app(&mut terminal, app).await;
    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<Backend>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(terminal: &mut Terminal<Backend>) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run_app(terminal: &mut Terminal<Backend>, mut app: App) -> io::Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let tick_rate = std::time::Duration::from_millis(250);

    let tx_tick = tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tick_rate).await;
            if tx_tick.send(Action::Tick).is_err() {
                break;
            }
        }
    });

    let tx_event = tx.clone();
    tokio::task::spawn_blocking(move || {
        loop {
            if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false)
                && let Ok(Event::Key(key)) = event::read()
                && tx_event.send(Action::Key(key)).is_err()
            {
                break;
            }
        }
    });

    let tx_fetch = tx.clone();
    tokio::spawn(async move {
        loop {
            let _ = tx_fetch.send(Action::FetchMetrics);
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    while app.running {
        if let Some(mut action) = rx.recv().await {
            loop {
                match action {
                    Action::Tick => {}
                    Action::FetchMetrics => {
                        let tx_res = tx.clone();
                        let window = match app.window {
                            TimeWindow::W1h => "1h",
                            TimeWindow::W3h => "3h",
                            TimeWindow::W6h => "6h",
                            TimeWindow::W12h => "12h",
                            TimeWindow::W24h => "24h",
                            TimeWindow::WMax => "max",
                        }
                        .to_string();
                        tokio::spawn(async move {
                            match tui::network::fetch_metrics(&window).await {
                                Ok(jobs) => {
                                    let _ = tx_res.send(Action::SetJobs(jobs));
                                }
                                Err(e) => {
                                    let _ = tx_res.send(Action::FetchError(e.to_string()));
                                }
                            }
                        });
                    }
                    Action::Key(key) => {
                        if let Some(key_action) = handle_key_event(key, &app) {
                            match key_action {
                                Action::NextWindow => {
                                    app.update(Action::NextWindow);
                                    let _ = tx.send(Action::FetchMetrics);
                                }
                                _ => app.update(key_action),
                            }
                        }
                    }
                    _ => app.update(action),
                }

                if !app.running {
                    break;
                }

                match rx.try_recv() {
                    Ok(next_action) => action = next_action,
                    Err(_) => break,
                }
            }
        }
        terminal.draw(|f| ui::draw(f, &mut app))?;
    }
    Ok(())
}

fn handle_key_event(key: event::KeyEvent, app: &App) -> Option<Action> {
    if key.modifiers.contains(event::KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        return Some(Action::Quit);
    }
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Tab => Some(Action::NextMetric),
        KeyCode::BackTab => Some(Action::PreviousMetric),
        KeyCode::Char('t') => Some(Action::NextWindow),
        KeyCode::Char('p') => Some(Action::ToggleCommandPalette),
        _ => match app.focused_pane {
            Pane::Graph => match key.code {
                KeyCode::Char('j') => Some(Action::FocusPane(Pane::Jobs)),
                KeyCode::Char('b') => Some(Action::ToggleBlameMode),
                _ => None,
            },
            Pane::Jobs => {
                if app.jobs_table_state.is_searching {
                    match key.code {
                        KeyCode::Backspace => Some(Action::TableBackspace),
                        KeyCode::Esc | KeyCode::Enter => Some(Action::TableEndSearch),
                        KeyCode::Char(c) => Some(Action::TableChar(c)),
                        _ => None,
                    }
                } else {
                    match app.jobs_table_state.focus_mode {
                        TableFocusMode::Row => match key.code {
                            KeyCode::Char('g') => Some(Action::FocusPane(Pane::Graph)),
                            KeyCode::Char('/') => Some(Action::TableSearch("".to_string())),
                            KeyCode::Char('j') | KeyCode::Down => Some(Action::NextRow),
                            KeyCode::Char('k') | KeyCode::Up => Some(Action::PreviousRow),
                            KeyCode::Char('s') => Some(Action::TableSort),
                            KeyCode::Enter => Some(Action::TableFocusCell),
                            _ => None,
                        },
                        TableFocusMode::Cell => match key.code {
                            KeyCode::Char('g') => Some(Action::FocusPane(Pane::Graph)),
                            KeyCode::Char('h') | KeyCode::Left => Some(Action::TablePrevCol),
                            KeyCode::Char('l') | KeyCode::Right => Some(Action::TableNextCol),
                            KeyCode::Char('j') | KeyCode::Down => Some(Action::NextRow),
                            KeyCode::Char('k') | KeyCode::Up => Some(Action::PreviousRow),
                            KeyCode::Char('s') => Some(Action::TableSort),
                            KeyCode::Char('r') => Some(Action::TableFocusRow),
                            KeyCode::Char('c') => Some(Action::TableFocusCol),
                            KeyCode::Esc => Some(Action::TableFocusRow),
                            _ => None,
                        },
                        TableFocusMode::Column => match key.code {
                            KeyCode::Char('g') => Some(Action::FocusPane(Pane::Graph)),
                            KeyCode::Char('h') | KeyCode::Left => Some(Action::TablePrevCol),
                            KeyCode::Char('l') | KeyCode::Right => Some(Action::TableNextCol),
                            KeyCode::Char('s') => Some(Action::TableSort),
                            KeyCode::Esc => Some(Action::TableFocusRow),
                            _ => None,
                        },
                    }
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyModifiers};

    #[test]
    fn test_handle_key_event_quit() {
        let app = App::new();
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        assert_eq!(handle_key_event(key, &app), Some(Action::Quit));
    }

    #[test]
    fn test_handle_key_event_ctrl_c() {
        let app = App::new();
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(handle_key_event(key, &app), Some(Action::Quit));
    }

    #[test]
    fn test_handle_key_event_tab() {
        let app = App::new();
        let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::empty());
        assert_eq!(handle_key_event(key, &app), Some(Action::NextMetric));
    }

    #[test]
    fn test_handle_key_event_esc_unselect() {
        let mut app = App::new();
        app.jobs_table_state.focus_mode = TableFocusMode::Cell;
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        assert_eq!(handle_key_event(key, &app), Some(Action::TableFocusRow));
    }

    #[test]
    fn test_handle_key_event_search() {
        let mut app = App::new();
        app.jobs_table_state.is_searching = true;
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        assert_eq!(handle_key_event(key, &app), Some(Action::TableChar('a')));
        
        let key_esc = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        assert_eq!(handle_key_event(key_esc, &app), Some(Action::TableEndSearch));
    }
}
