mod app;
mod event;
mod constants;
mod game;
mod message;
mod ui;
mod history;
mod level;
mod util;

use std::io;

use app::App;
use crossterm::{
    event::KeyCode,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use event::{Event, EventHandler};
use ratatui::{
    backend::CrosstermBackend,
    Terminal, TerminalOptions, Viewport,
};

// TODO: Add a debug mode where I can see the current solution so I can test winning
// TODO: Maybe allow saving the game?
fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Fullscreen,
        },
    )?;
   
    let mut app = App::new();
    let event_handler = EventHandler::new(constants::TICK_TIME_MILLIS);

    loop {
        match event_handler.next()? {
            Event::Tick => {
		app.clear_message_if_expired();
		app.current_tick += 1;
                terminal.draw(|f| ui::render(f, &app))?;
            }
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char(c) => {
			if c.is_digit(10) {
			    app.add_to_input(c);
			} else if c == 'q' {
			    break;
			} else if c == 't' {
			    app.switch_tab();
			} else if c == 'd' {
			    app.debug = !app.debug;
			}
                    }
		    KeyCode::Enter => {
			app.submit_guess();
		    }
		    KeyCode::Backspace => {
			app.delete_from_input();
		    }
                    _ => {}
                };
            }
            _ => {}
        }
    }

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
