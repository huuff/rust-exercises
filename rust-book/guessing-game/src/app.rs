use crate::{constants, game::{self, Game}, message::Message};

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
	let numeric_guess = self.input.parse::<u64>().expect("input is not a number");

	self.message = Some(Message::from_guess_result(self.game.guess(numeric_guess)));

	self.input.clear();
    }
}
