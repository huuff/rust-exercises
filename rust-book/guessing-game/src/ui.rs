use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect, SegmentSize},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{app::App, constants};

pub fn render(f: &mut Frame, app: &App) {
    let outer_block = Block::default()
        .title(format!("Guess the number! Level {}", app.level))
        .borders(Borders::ALL)
	;
    f.render_widget(outer_block, f.size());
    
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

}

fn render_input(f: &mut Frame, app: &App, target_rect: Rect) {
    let horizontal_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Min(0),
            // max size of the input plus 2 for the block borders
            Constraint::Length(constants::MAX_INPUT_SIZE as u16 + 2),
            Constraint::Min(0),
        ],
    )
    .segment_size(SegmentSize::EvenDistribution)
    .split(target_rect);

    let input = Paragraph::new(Text::raw(&app.input)).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Enter your guess"),
    );
    f.render_widget(input, horizontal_layout[1]);
}

fn render_message(f: &mut Frame, app: &App, target_rect: Rect) {
    if let Some(message) = &app.message {
	let message_paragraph = Paragraph::new(message.to_text()).alignment(Alignment::Center);
	f.render_widget(message_paragraph, target_rect);
    }
}
