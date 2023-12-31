use std::num::IntErrorKind;

use ratatui::style::Color;

use crate::{constants, game::Game, message::Message};

pub struct App {
    game: Game,
    pub input: String,
    pub message: Option<Message>,
    
}

impl App {
    pub fn new() -> Self {
	Self {
	    input: String::new(),
	    game: Game::new(),
	    message: None,
	}
    }

    pub fn add_to_input(&mut self, c: char) {
	if self.input.len() < constants::MAX_INPUT_SIZE {
	    self.input.push(c);
	}
    }

    pub fn delete_from_input(&mut self) {
	self.input.pop();
    }

    pub fn submit_guess(&mut self) {
	match self.input.parse::<u64>() {
	    Ok(guess) => {
		self.message = Some(Message::from_guess_result(self.game.check_guess(guess)));
		self.input.clear();
	    }
	    Err(err) => {
		if let IntErrorKind::Empty = err.kind() {
		    self.message = Some(Message::new("You must enter something!", Color::Red))
		} else {
		    panic!("{}", err)
		}
	    }
	}

    }
}
