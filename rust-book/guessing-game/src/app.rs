use std::cmp::Ordering;

use rand::Rng;

use crate::constants;

pub enum LastGuessDirection {
    TooHigh,
    TooLow,
    Correct,
}

pub struct App {
    pub input: String,
    pub solution: u64,
    pub last_guess_direction: Option<LastGuessDirection>,
}

impl App {
    pub fn new() -> Self {
	let mut rng = rand::thread_rng();
	Self {
	    input: String::new(),
	    solution: rng.gen_range(1..=100),
	    last_guess_direction: None,
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

	self.last_guess_direction = Some(match numeric_guess.cmp(&self.solution) {
	    Ordering::Less => LastGuessDirection::TooLow,
	    Ordering::Equal => LastGuessDirection::Correct,
	    Ordering::Greater => LastGuessDirection::TooHigh,
	});

	self.input.clear();
    }
}
