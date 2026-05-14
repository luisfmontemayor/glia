use tui::action::Action;
use tui::app::App;
use tui::ui;
use tui::app::Pane;
use tui::components::table::table_state::TableFocusMode;
use tui::app::TimeWindow;

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
                        }.to_string();
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
                        if key.modifiers.contains(event::KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                            app.update(Action::Quit);
                        }
                        match key.code {
                            KeyCode::Char('q') => app.update(Action::Quit),
                            KeyCode::Tab => app.update(Action::NextMetric),
                            KeyCode::BackTab => app.update(Action::PreviousMetric),
                            KeyCode::Char('t') => {
                                app.update(Action::NextWindow);
                                let _ = tx.send(Action::FetchMetrics);
                            }
                            KeyCode::Char('p') => app.update(Action::ToggleCommandPalette),
                            _ => {
                                match app.focused_pane {
                                    Pane::Graph => match key.code {
                                        KeyCode::Char('j') => app.update(Action::FocusPane(Pane::Jobs)),
                                        KeyCode::Char('b') => app.update(Action::ToggleBlameMode),
                                        _ => {}
                                    },
                                    Pane::Jobs => {
                                        if app.jobs_table_state.is_searching {
                                            match key.code {
                                                KeyCode::Backspace => app.update(Action::TableBackspace),
                                                KeyCode::Esc | KeyCode::Enter => app.update(Action::TableEndSearch),
                                                KeyCode::Char(c) => app.update(Action::TableChar(c)),
                                                _ => {}
                                            }
                                        } else {
                                            match app.jobs_table_state.focus_mode {
                                                TableFocusMode::Row => match key.code {
                                                    KeyCode::Char('g') => app.update(Action::FocusPane(Pane::Graph)),
                                                    KeyCode::Char('/') => app.update(Action::TableSearch("".to_string())),
                                                    KeyCode::Char('j') | KeyCode::Down => app.update(Action::NextRow),
                                                    KeyCode::Char('k') | KeyCode::Up => app.update(Action::PreviousRow),
                                                    KeyCode::Char('s') => app.update(Action::TableSort),
                                                    KeyCode::Enter => app.update(Action::TableFocusCell),
                                                    _ => {}
                                                },
                                                TableFocusMode::Cell => match key.code {
                                                    KeyCode::Char('g') => app.update(Action::FocusPane(Pane::Graph)),
                                                    KeyCode::Char('h') | KeyCode::Left => app.update(Action::TablePrevCol),
                                                    KeyCode::Char('l') | KeyCode::Right => app.update(Action::TableNextCol),
                                                    KeyCode::Char('j') | KeyCode::Down => app.update(Action::NextRow),
                                                    KeyCode::Char('k') | KeyCode::Up => app.update(Action::PreviousRow),
                                                    KeyCode::Char('s') => app.update(Action::TableSort),
                                                    KeyCode::Char('r') => app.update(Action::TableFocusRow),
                                                    KeyCode::Char('c') => app.update(Action::TableFocusCol),
                                                    KeyCode::Esc => app.update(Action::TableFocusRow),
                                                    _ => {}
                                                },
                                                TableFocusMode::Column => match key.code {
                                                    KeyCode::Char('g') => app.update(Action::FocusPane(Pane::Graph)),
                                                    KeyCode::Char('h') | KeyCode::Left => app.update(Action::TablePrevCol),
                                                    KeyCode::Char('l') | KeyCode::Right => app.update(Action::TableNextCol),
                                                    KeyCode::Char('s') => app.update(Action::TableSort),
                                                    KeyCode::Esc => app.update(Action::TableFocusRow),
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
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
