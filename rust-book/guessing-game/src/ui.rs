use std::cmp::Ordering;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect, SegmentSize, Offset},
    text::Text,
    widgets::{Block, Borders, Paragraph, List},
    Frame, style::{Style, Color},
};

use crate::{app::App, constants, history::HistoryEntry};

pub fn render(f: &mut Frame, app: &App) {
    render_outer_block(f, app);
    
    let vertical_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
        ],
    )
	.segment_size(SegmentSize::EvenDistribution)
	.split(f.size())
	;

    render_input(f, app, vertical_layout[1]);

    render_message(f, app, vertical_layout[2]);

    let bottom_layout = Layout::new(
	Direction::Horizontal,
	[
	    Constraint::Length(30),
	    Constraint::Min(0),
	    Constraint::Length(30),
	]
    )
	.segment_size(SegmentSize::EvenDistribution)
	.split(vertical_layout[3]);

    render_history(f, app, bottom_layout[0].offset(Offset { x: 2, y: -1 }));
    render_instructions(f, bottom_layout[2].offset(Offset { x: -2, y: -1 }));

}

/// Render the outer block that will have the title
fn render_outer_block(f: &mut Frame, app: &App) {
    let outer_block = Block::default()
        .title(format!("Guess the number! Level {}", app.level))
        .borders(Borders::ALL)
	;
    f.render_widget(outer_block, f.size());
}

fn render_input(f: &mut Frame, app: &App, target_rect: Rect) {
    let horizontal_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Min(0),
            // max size of the input plus 2 for the block borders
            Constraint::Length(constants::MAX_INPUT_SIZE + 2),
            Constraint::Min(0),
        ],
    )
    .segment_size(SegmentSize::EvenDistribution)
    .split(target_rect);


    let input_rect = horizontal_layout[1];
    let input = Paragraph::new(Text::raw(&app.input)).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Enter your guess"),
    );
    f.render_widget(input, input_rect);

    // Render a blinking cursor
    let ticks_for_a_blink = constants::CURSOR_BLINK_DURATION_MILLIS / constants::TICK_TIME_MILLIS;
    if (app.current_tick % (ticks_for_a_blink*2)) > ticks_for_a_blink
	&& app.input.len() < constants::MAX_INPUT_SIZE.try_into().unwrap()
    {
	let cursor = Block::default().style(Style::new().bg(Color::White));
	f.render_widget(
	    cursor,
	    Rect::new(input_rect.x+1+(app.input.len() as u16), input_rect.y+1, 1, 1)
	);
    }
}

fn render_message(f: &mut Frame, app: &App, target_rect: Rect) {
    if let Some(message) = &app.message {
	let message_paragraph = Paragraph::new(message.to_text()).alignment(Alignment::Center);
	f.render_widget(message_paragraph, target_rect);
    }
}

fn render_instructions(f: &mut Frame, target_rect: Rect) {
    let list = List::new([
	"'q': exit",
	"'enter': submit guess",
    ]).block(Block::default()
             .title("Instructions")
            .borders(Borders::ALL)
    );

    f.render_widget(list, target_rect)
}

// TODO: Render a separate tabbed history for guesses
fn render_history(f: &mut Frame, app: &App, target_rect: Rect) {
    let list = List::new(
	app.game_history.entries
	    .iter()
	    .map(|HistoryEntry { key, value }| {
		let max_solution: f64 = 10_f64.powf((*key).into());
		let optimal_attempts = max_solution.log2().ceil() as u64;
		
		Text::styled(
		    format!("Level {}: {} attempts", key, value),
		    match value.cmp(&optimal_attempts) {
			Ordering::Less => Style::new().bg(Color::Green),
			Ordering::Equal => Style::new().bg(Color::Blue),
			Ordering::Greater => Style::new().bg(Color::Red),
		    })
	    })
    ).block(Block::default()
            .title("Levels")
            .borders(Borders::ALL)
    );

    f.render_widget(list, target_rect)
    
}
