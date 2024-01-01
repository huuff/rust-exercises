use std::num::IntErrorKind;

use ratatui::style::Color;

use crate::{constants, game::{Game, GuessResult}, message::Message, history::{History, HistoryEntry}};

pub struct App {
    game: Game,
    pub level: u16,
    pub input: String,
    pub message: Option<Message>,
    pub current_tick: u64,
    pub game_history: History<u16, u64>,
}

impl App {
    pub fn new() -> Self {
	let level = 1;
	Self {
	    input: String::new(),
	    level,
	    game: Game::new(level),
	    message: None,
	    current_tick: 0,
	    game_history: History::new(),
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
		    self.game_history.push(HistoryEntry { key: self.level, value: self.game.attempts });
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

    pub fn clear_message_if_expired(&mut self) {
	if let Some(message) = &self.message {
	    if message.is_expired() {
		self.message = None;
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
