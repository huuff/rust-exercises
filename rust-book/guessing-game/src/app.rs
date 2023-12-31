use std::num::IntErrorKind;

use ratatui::style::Color;

use crate::{constants, game::{Game, GuessResult}, message::Message};

pub struct App {
    game: Game,
    pub level: u16,
    pub input: String,
    pub message: Option<Message>,
}

impl App {
    pub fn new() -> Self {
	let level = 1;
	Self {
	    input: String::new(),
	    level,
	    game: Game::new(level),
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
		let guess_result = self.game.check_guess(guess);

		if let GuessResult::Correct = guess_result {
		    self.advance_level();
		}
		
		self.message = Some(Message::from_guess_result(guess_result));
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

    fn advance_level(&mut self) {
	// Advance only if the level is below the max size of the input since each level adds a digit
	// to the maximum possible solution
	if self.level < (constants::MAX_INPUT_SIZE as u16) {
	    self.level += 1;
	    self.game = Game::new(self.level);
	}
    }
}
