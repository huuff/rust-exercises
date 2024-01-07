use std::io::{self, Stdout};

use crossterm::{terminal::{EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode}, ExecutableCommand as _};
use ratatui::{prelude::*, widgets::{Block, Borders, Table, Row}, layout::SegmentSize};
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
    f.render_widget(outer_block, f.size());

    let vertical_layout = Layout::new(
	Direction::Vertical,
	Constraint::from_mins([0, 20, 0])
    )
        .segment_size(SegmentSize::EvenDistribution)
	;

    let horizontal_layout = Layout::new(
	Direction::Horizontal,
	Constraint::from_mins([0, 20, 0])
    )
        .segment_size(SegmentSize::EvenDistribution)
	;

    let center_rect = horizontal_layout.split(vertical_layout.split(f.size())[1])[1];
    render_derpartment_table(f, app, center_rect);
}

pub fn render_derpartment_table(f: &mut Frame, app: &App, target_area: Rect) {
    let widths = [Constraint::default(), Constraint::Length(10)];
    // TODO: It'd be cool if I could use some &strs here
    let rows = vec![
	["Department".to_string(), "Employees".to_string()],
    ].into_iter().chain(
	Department::iter().map(|department|
			       [
				   department.to_string(),
				   app.department_to_employees
					.get(&department)
					.map(|employees| employees.len().to_string())
				       .unwrap_or("0".to_string())
			       ]
	)
    ).map(|t| Row::new(t));

    let table = Table::new(rows, widths)
        .block(Block::default()
               .title("Departments")
		.borders(Borders::ALL)
	).segment_size(SegmentSize::EvenDistribution);

    f.render_widget(table, target_area);
}
