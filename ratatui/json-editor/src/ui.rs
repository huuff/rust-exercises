use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Text, Line, Span},
    widgets::{Block, Borders, Paragraph, ListItem, List},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create New Json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    // TODO: Refactor to a .map
    // TODO: Most of the below's code is for rendring the footer, we could extract it
    // to a different function
    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
	list_items.push(ListItem::new(Line::from(Span::styled(
	    format!("{: <25}: {}", key, app.pairs[key]),
	    Style::default().fg(Color::Yellow),
	))))
    }

    let list = List::new(list_items);

    f.render_widget(list, chunks[1]);

    let current_navigation_text = vec![
	// The first half of the text
	match app.current_screen {
	    CurrentScreen::Main => {
		Span::styled("Normal Mode", Style::default().fg(Color::Green))
	    }
	    CurrentScreen::Editing => {
		Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
	    }
	    CurrentScreen::Exiting => {
		Span::styled("Exiting", Style::default().fg(Color::LightRed))
	    }
	},
	// A white divider bar to separate the two sections
	Span::styled(" | ", Style::default().fg(Color::White)),
	// The final section of the text, which hints on what the user is editing
	{
	    // TODO: Simplify to a single match
	    if let Some(editing) = &app.currently_editing {
		match editing {
		    CurrentlyEditing::Key => Span::styled(
			"Editing Json Key",
			Style::default().fg(Color::Green)
		    ),
		    CurrentlyEditing::Value => Span::styled(
			"Editing Json Value",
			Style::default().fg(Color::LightGreen),
		    ),
		}
	    } else {
		Span::styled(
		    "Not editing Anything",
		    Style::default().fg(Color::DarkGray)
		)
	    }
	}
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = match app.current_screen {
        CurrentScreen::Main => Span::styled(
	    "(q) to quit / (e) to make new pair",
	    Style::default().fg(Color::Red),
	),
        CurrentScreen::Editing => Span::styled(
	    "(ESC) to cancel/(Tab) to switch boxes/(enter) to complete",
	    Style::default().fg(Color::Red),
	),
        CurrentScreen::Exiting => Span::styled(
	    "(q) to quit / (e) to make new pair",
	    Style::default().fg(Color::Red),
	)
    };

    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
