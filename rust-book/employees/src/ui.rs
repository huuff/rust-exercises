use std::io::{self, Stdout};

use crossterm::{terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}, ExecutableCommand as _};
use ratatui::{prelude::*, widgets::{Block, Borders}};
use strum::IntoEnumIterator;

use crate::{App, Department};

pub fn init_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let terminal = Terminal::with_options(
	CrosstermBackend::new(io::stdout()),
	TerminalOptions {
	    viewport: Viewport::Fullscreen,
	}
    )?;
    Ok(terminal)
}

pub fn close_terminal() -> anyhow::Result<()> {
    io::stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn render(f: &mut Frame, app: &App) {
    let outer_block = Block::default()
        .title("Employees")
        .borders(Borders::ALL)
	;

    // TODO: Actually use this to show a table
    Department::iter();
    f.render_widget(outer_block, f.size());
}
