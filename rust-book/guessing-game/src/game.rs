use std::cmp;

use rand::Rng;

use crate::{history::History, level::GameLevel};

pub struct Game {
   solution: u64,
   pub guess_history: History<u64, GuessResult>,
}

impl Game {
    pub fn new(level: GameLevel) -> Self {
	let mut rng = rand::thread_rng();
	
	Self {
	    solution: rng.gen_range(1..=(level.max_solution())),
	    guess_history: History::new(),
	}
    }

    pub fn check_guess(&mut self, guess: u64) -> GuessResult {
	let result = GuessResult::from_ordering(guess.cmp(&self.solution));
	self.guess_history.push(guess, result);
	result
    }

    pub fn attempts(&self) -> usize {
	self.guess_history.entries.len()
    }
}

#[derive(Copy, Clone)]
pub enum GuessResult {
    TooHigh,
    TooLow,
    Correct
}

impl GuessResult {
    fn from_ordering(ord: cmp::Ordering) -> Self {
	match ord {
	    cmp::Ordering::Less => GuessResult::TooLow,
	    cmp::Ordering::Equal => GuessResult::Correct,
	    cmp::Ordering::Greater => GuessResult::TooHigh,
	}
    }

    pub fn to_str(&self) -> &'static str {
	match self {
	    GuessResult::TooHigh => "Too high!",
	    GuessResult::TooLow => "Too low!",
	    GuessResult::Correct => "Correct!",
	}
    }
}

