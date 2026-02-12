pub mod app;
pub mod ui;

use std::{
    error::Error,
    io,
    sync::Arc,
    time::{Duration, Instant},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal};
use crate::telemetry::MinerMetrics;
use self::app::App;

pub async fn run_tui(metrics: Arc<MinerMetrics>) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(100); // Faster TUI for better responsiveness
    let mut app = App::new(metrics.clone());
    let res = run_app(&mut terminal, &mut app, tick_rate).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        let snapshot = app.metrics.snapshot().await;
        terminal.draw(|f| ui::render(f, app, &snapshot))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Esc => app.should_quit = true,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick().await;
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}
