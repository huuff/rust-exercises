use std::time::Instant;

use ratatui::{
    style::{Color, Style},
    text::Text,
};

use crate::{game::GuessResult, constants};

pub struct Message {
    text: String,
    color: Color,
    created_at: Instant,
}

impl Message {
    // TODO: Can I make this take an &str?
    pub fn new(text: String, color: Color) -> Self {
        Self {
            text,
            color,
            created_at: Instant::now(),
        }
    }

    pub fn from_guess_result(r: GuessResult) -> Self {
        Self::new(
            r.to_string(),
            match r {
                GuessResult::TooHigh | GuessResult::TooLow => Color::Red,
                GuessResult::Correct => Color::Green,
            },
        )
    }

    pub fn to_text(&self) -> Text {
        Text::styled(&self.text, Style::default().fg(self.color))
    }

    pub fn is_expired(&self) -> bool {
	self.created_at.elapsed() > constants::MAX_MESSAGE_DURATION
    }
}
