mod app;
mod ui;

use crossterm::{
    event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyCode, KeyEventKind},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode}
};
use ratatui::{backend::{CrosstermBackend, Backend}, Terminal};
use std::io;
use app::{App, CurrentScreen, CurrentlyEditing};
use ui::ui;

fn main() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    match res {
	Ok(true) => app.print_json()?,
	Err(err) => println!("{err:?}"),
	_ => {}
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
	terminal.draw(|f| ui(f, app))?;

	if let Event::Key(key) = event::read()? {
	    if key.kind == KeyEventKind::Release {
		// skip events that are not KeyEventKind::Press
		continue;
	    }

	    match app.current_screen {
		CurrentScreen::Main => match key.code {
		    KeyCode::Char('e') => {
			app.current_screen = CurrentScreen::Editing;
			app.currently_editing = Some(CurrentlyEditing::Key);
		    }
		    KeyCode::Char('q') => {
			app.current_screen = CurrentScreen::Exiting;
		    }
		    _ => {}
		}
		CurrentScreen::Exiting => match key.code {
		    KeyCode::Char('y') => {
			return Ok(true);
		    }
		    KeyCode::Char('n') | KeyCode::Char('q') => {
			return Ok(false);
		    }
		    _ => {}
		},
		CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
		    match key.code {
			// TODO: Clean these... Can't we just put the if let inside the match?
			KeyCode::Enter => {
			    if let Some(editing) = &app.currently_editing {
				match editing {
				    CurrentlyEditing::Key => {
					app.currently_editing = Some(CurrentlyEditing::Value);
				    }
				    CurrentlyEditing::Value => {
					app.save_key_value();
					app.current_screen = CurrentScreen::Main;
				    }
				}
			    }
			}
			KeyCode::Backspace => {
			    if let Some(editing) = &app.currently_editing {
				match editing {
				    CurrentlyEditing::Key => {
					app.key_input.pop();
				    }
				    CurrentlyEditing::Value => {
					app.value_input.pop();
				    }
				}
			    }
			}
			KeyCode::Esc => {
			    app.current_screen = CurrentScreen::Main;
			    app.currently_editing = None;
			}
			KeyCode::Tab => {
			    app.toggle_editing();
			}
			KeyCode::Char(value) => {
			    if let Some(editing) = &app.currently_editing {
				match editing {
				    CurrentlyEditing::Key => {
					app.key_input.push(value);
				    }
				    CurrentlyEditing::Value => {
					app.value_input.push(value);
				    }
				}
			    }
			}
			_ => {}
		    }
		}
		_ => {}
	    }
	}
    }
}
