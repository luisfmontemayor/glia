pub mod app;
pub mod network;
pub mod ui;

use crate::app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{error::Error, io};

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
    while app.running {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.running = false,
                    KeyCode::Tab => app.next_metric(),
                    KeyCode::Char('t') => app.next_window(),
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
