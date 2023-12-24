use std::io;

use ratatui::{Terminal, backend::CrosstermBackend, widgets::Paragraph};
use crossterm::event::{self, KeyCode};

fn main() -> io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stderr()))?;

    let mut counter = 0;
    loop {
	terminal.draw(|f| {
	    f.render_widget(Paragraph::new(format!("Counter: {counter}")), f.size());
	})?;

	if event::poll(std::time::Duration::from_millis(250))? {
	    if let event::Event::Key(key) = event::read()? {
		match key.code {
		    KeyCode::Char('j') => counter += 1,
		    KeyCode::Char('k') => counter -= 1,
		    KeyCode::Char('q') => break,
		    _ => {},
		};
	    };
	}
    }

    crossterm::execute!(io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
