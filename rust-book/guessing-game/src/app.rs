use crate::{constants, game::{self, Game}};

pub struct App {
    game: Game,
    pub input: String,
    pub last_guess_result: Option<game::GuessResult>,
    
}

impl App {
    pub fn new() -> Self {
	Self {
	    input: String::new(),
	    game: Game::new(),
	    last_guess_result: None,
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

	self.last_guess_result = Some(self.game.guess(numeric_guess));

	self.input.clear();
    }
}
