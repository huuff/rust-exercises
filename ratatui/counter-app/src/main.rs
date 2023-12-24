
pub mod app;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update;

use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;
use std::io;

fn main() -> anyhow::Result<()> {
    // Create an application
    let mut app = App::new();

    // Initialize the terminal user interface
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
	// Render the user interface
	tui.draw(&mut app)?;

	match tui.events.next()? {
	    Event::Tick => {},
	    Event::Key(key_event) => update(&mut app, key_event),
	    Event::Mouse(_) => {}
	    Event::Resize(_, _) => {}
	}
    }

    tui.exit()?;
    Ok(())
}
