use std::cmp::Ordering;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Offset, Rect, SegmentSize},
    style::{Color, Style, Modifier},
    text::Text,
    widgets::{Block, Borders, List, Paragraph, Tabs},
    Frame,
};

use crate::{
    app::{App, HistoryTab},
    constants,
    history::HistoryEntry,
};

pub fn render(f: &mut Frame, app: &App) {
    if app.finished {
	render_game_end(f, crate::util::centered_rect(f.size(), 25, 25));
	return;
    }
    
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
    .split(f.size());

    render_input(f, app, vertical_layout[1]);

    render_message(f, app, vertical_layout[2]);

    let bottom_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length(30),
            Constraint::Min(0),
            Constraint::Length(30),
        ],
    )
    .segment_size(SegmentSize::EvenDistribution)
    .split(vertical_layout[3]);

    render_history(f, app, bottom_layout[0].offset(Offset { x: 2, y: -1 }));
    render_instructions(f, bottom_layout[2].offset(Offset { x: -2, y: -1 }));
}

/// Render the outer block that will have the title
fn render_outer_block(f: &mut Frame, app: &App) {
    let outer_block = Block::default()
        .title(format!("Guess the number! Level {}", app.level.0))
        .borders(Borders::ALL);
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
    if (app.current_tick % (ticks_for_a_blink * 2)) > ticks_for_a_blink
        && app.input.len() < constants::MAX_INPUT_SIZE.try_into().unwrap()
    {
        let cursor = Block::default().style(Style::new().bg(Color::White));
        f.render_widget(
            cursor,
            Rect::new(
                input_rect.x + 1 + (app.input.len() as u16),
                input_rect.y + 1,
                1,
                1,
            ),
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
    let list = List::new(["'q': exit", "'enter': submit guess", "'t': change tab"])
        .block(Block::default().title("Instructions").borders(Borders::ALL));

    f.render_widget(list, target_rect)
}

fn render_history(f: &mut Frame, app: &App, target_rect: Rect) {
    let tabs = Tabs::new(vec!["Guesses", "Games"])
        .block(Block::default().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::UNDERLINED))
        .select(match app.current_tab {
            HistoryTab::Guesses => 0,
            HistoryTab::Games => 1,
        })
        .divider("|");

    let tabs_target_rect = Rect::new(target_rect.x, target_rect.y - 2, target_rect.width, 3);
    f.render_widget(tabs, tabs_target_rect);

    let list = match app.current_tab {
        HistoryTab::Guesses => List::new(
            app.game
                .guess_history
                .entries
                .iter()
                .map(|HistoryEntry { key, value }| Text::raw(format!("{}: {}", key, value.to_str()))),
        ),
        HistoryTab::Games => List::new(app.game_history.entries.iter().map(
            |HistoryEntry { key: level, value }| {
                Text::styled(
                    format!("Level {}: {} attempts", level.0, value),
                    match value.cmp(&(level.optimal_guesses().try_into().unwrap())) {
                        Ordering::Less => Style::new().bg(Color::Green).fg(Color::Black),
                        Ordering::Equal => Style::new().fg(Color::White),
                        Ordering::Greater => Style::new().bg(Color::Red).fg(Color::Black),
                    },
                )
            },
        )),
    }
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(list, target_rect);
}

pub fn render_game_end(f: &mut Frame, target_area: Rect) {
    let p = Paragraph::new("You won the game!");

    f.render_widget(p, target_area);
}
