use std::{io, time::Duration};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    event::{self, KeyEventKind, KeyCode, KeyEvent},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, block::Position},
    Frame, Terminal, TerminalOptions, Viewport, layout::Alignment,
};

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut terminal = Terminal::with_options(
        CrosstermBackend::new(io::stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(5),
        },
    )?;

    loop {
        terminal.draw(ui)?;

	if event::poll(Duration::from_millis(66))? {
	    if let event::Event::Key(KeyEvent { kind, code, ..}) = event::read()? {
		if kind != KeyEventKind::Press { continue; }

		match code {
		    KeyCode::Char('q') => { break; },
		    _ => {}
		}
	    }
	}
    }

    disable_raw_mode()?;

    Ok(())
}

fn ui(f: &mut Frame) {
    let p = Paragraph::new("Hello World")
	.block(Block::default()
	       .borders(Borders::ALL)
	       .title("Press q to exit")
	       .title_position(Position::Bottom)
	       .title_alignment(Alignment::Right)
	)
	;

    f.render_widget(p, f.size());
}
