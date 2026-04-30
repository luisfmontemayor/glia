pub mod action;
pub mod app;
pub mod network;
pub mod theme;
pub mod ui;

use crate::action::Action;
use crate::app::App;
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
            if event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(Event::Key(key)) = event::read() {
                    if tx_event.send(Action::Key(key)).is_err() {
                        break;
                    }
                }
            }
        }
    });

    while app.running {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Some(action) = rx.recv().await {
            match action {
                Action::Tick => {
                    // Tick logic goes here
                }
                Action::Key(key) => match key.code {
                    KeyCode::Char('q') => app.update(Action::Quit),
                    KeyCode::Tab => app.update(Action::NextMetric),
                    KeyCode::BackTab => app.update(Action::PreviousMetric),
                    KeyCode::Char('t') => app.update(Action::NextWindow),
                    _ => {}
                },
                _ => app.update(action),
            }
        }
    }
    Ok(())
}
