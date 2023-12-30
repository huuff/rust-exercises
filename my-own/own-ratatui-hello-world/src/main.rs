use std::{io, thread, time::Duration};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, event::{self, KeyEventKind, KeyCode},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal, TerminalOptions, Viewport,
};

fn main() -> anyhow::Result<()> {
    // io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(5),
        },
    )?;

    loop {
        terminal.draw(ui)?;

        // thread::sleep(Duration::from_millis(66));
	if event::poll(Duration::from_millis(66))? {
	    if let event::Event::Key(key) = event::read()? {
		if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
		    break;
		}
	    }
	}
    }

    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn ui(f: &mut Frame) {
    let p = Paragraph::new("Hello World").block(Block::default().borders(Borders::ALL));

    f.render_widget(p, f.size());
}
