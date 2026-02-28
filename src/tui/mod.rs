pub mod actions;
pub mod app;
pub mod components;
pub mod event;
pub mod keybindings;
pub mod theme;
pub mod ui;

use std::time::Duration;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::data::collector::collect_port_entries;
use crate::tui::actions::handle_action;
use crate::tui::app::App;
use crate::tui::event::{Event, EventHandler};
use crate::tui::keybindings::handle_key;

pub async fn run_tui(refresh_interval: Duration) -> anyhow::Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    // Initial data load
    let entries = tokio::task::spawn_blocking(collect_port_entries).await??;
    let mut app = App::new(entries, refresh_interval);

    // Event handler
    let mut events = EventHandler::new(refresh_interval);

    // Main loop
    let result = run_loop(&mut term, &mut app, &mut events).await;

    // Teardown terminal (always, even on error)
    terminal::disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    result
}

async fn run_loop(
    term: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    events: &mut EventHandler,
) -> anyhow::Result<()> {
    loop {
        term.draw(|frame| ui::render(frame, app))?;

        match events.next().await {
            Some(Event::Key(key)) => {
                let action = handle_key(app, key);
                handle_action(app, action);
            }
            Some(Event::Tick) => {
                let new_entries = tokio::task::spawn_blocking(collect_port_entries).await??;
                app.update_entries(new_entries);
            }
            None => break,
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
