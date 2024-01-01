use std::time::Instant;

use ratatui::{
    style::{Color, Style},
    text::Text,
};

use crate::{game::GuessResult, constants};

pub struct Message {
    text: &'static str,
    color: Color,
    created_at: Instant,
}

impl Message {
    pub fn new(text: &'static str, color: Color) -> Self {
        Self {
            text,
            color,
            created_at: Instant::now(),
        }
    }

    pub fn from_guess_result(r: GuessResult) -> Self {
        Self::new(
            match r {
                GuessResult::TooHigh => "Too high!",
                GuessResult::TooLow => "Too low!",
                GuessResult::Correct => "Correct!",
            },
            match r {
                GuessResult::TooHigh | GuessResult::TooLow => Color::Red,
                GuessResult::Correct => Color::Green,
            },
        )
    }

    pub fn to_text(&self) -> Text {
        Text::styled(self.text, Style::default().fg(self.color))
    }

    pub fn is_expired(&self) -> bool {
	self.created_at.elapsed() > constants::MAX_MESSAGE_DURATION
    }
}
