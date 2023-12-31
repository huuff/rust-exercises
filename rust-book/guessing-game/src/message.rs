use ratatui::{style::{Color, Style}, text::Text};

use crate::game::GuessResult;

pub struct Message {
    text: &'static str,
    color: Color,
}

impl Message {
    pub fn new(text: &'static str, color: Color) -> Self {
	Self { text, color }
    }

    
    pub fn from_guess_result(r: GuessResult) -> Self {
	Self {
	    text: match r {
		GuessResult::TooHigh => "Too high!",
		GuessResult::TooLow => "Too low!",
		GuessResult::Correct => "Correct!",
	    },
	    color: match r {
		GuessResult::TooHigh | GuessResult::TooLow => Color::Red,
		GuessResult::Correct => Color::Green,
	    }
	}
    }

    pub fn to_text(&self) -> Text {
	Text::styled(self.text, Style::default().fg(self.color))
    }
}
