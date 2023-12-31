use std::cmp;

use rand::Rng;

pub struct Game {
   solution: u64,
}

impl Game {
    pub fn new() -> Self {
	let mut rng = rand::thread_rng();
	
	Self {
	    solution: rng.gen_range(1..=100),
	}
    }

    pub fn check_guess(&self, guess: u64) -> GuessResult {
	GuessResult::from_ordering(guess.cmp(&self.solution))
    }
}

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
}
