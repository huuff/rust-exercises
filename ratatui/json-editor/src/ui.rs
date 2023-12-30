use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(f: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    render_header(f, main_layout[0]);

    render_body(f, app, main_layout[1]);

    render_footer(f, &app, main_layout[2]);

    if let Some(editing) = &app.currently_editing {
        render_edit_popup(f, app, editing);
    }

    if app.current_screen == CurrentScreen::Exiting {
	render_exit_popup(f);
    }


}

fn render_header(f: &mut Frame, target_area: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create New Json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, target_area);
}

fn render_body(f: &mut Frame, app: &App, target_area: Rect) {
    let list = List::new(
	app.pairs.keys().map(|k| ListItem::new(Line::from(Span::styled(
	    format!("{: <25}: {}", k, app.pairs[k]),
	    Style::default().fg(Color::Yellow),
	))))
    );
    f.render_widget(list, target_area);
}

fn render_footer(f: &mut Frame, app: &App, target_area: Rect) {
    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        },
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, which hints on what the user is editing
        match &app.currently_editing {
            Some(CurrentlyEditing::Key) => {
                Span::styled("Editing Json Key", Style::default().fg(Color::Green))
            }
            Some(CurrentlyEditing::Value) => {
                Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
            }
            None => Span::styled("Not editing Anything", Style::default().fg(Color::DarkGray)),
        },
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
        ),
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(target_area);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}

fn render_edit_popup(f: &mut Frame, app: &App, editing: &CurrentlyEditing) {
    let popup_block = Block::default()
        .title("Enter a new key-value pair")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(60, 25, f.size());
    f.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let mut key_block = Block::default().title("Key").borders(Borders::ALL);
    let mut value_block = Block::default().title("Value").borders(Borders::ALL);

    let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

    match editing {
        CurrentlyEditing::Key => key_block = key_block.style(active_style),
        CurrentlyEditing::Value => value_block = value_block.style(active_style),
    }

    let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
    f.render_widget(key_text, popup_chunks[0]);

    let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
    f.render_widget(value_text, popup_chunks[1]);
}

fn render_exit_popup(f: &mut Frame) {
    f.render_widget(Clear, f.size());
    let popup_block = Block::default()
	.title("Y/N")
	.borders(Borders::NONE)
	.style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(
        "Would you like to output the buffer as json? (y/n)",
        Style::default().fg(Color::Red),
    );

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, f.size());
    f.render_widget(exit_paragraph, area);
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
