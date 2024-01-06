use crate::constants;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct GameLevel(pub u16);

impl GameLevel {
    pub fn max() -> GameLevel {
        // Since each level adds a zero to the maximum possible solution,
        // we want the max level to be just the maximum number of numbers
        // we can fit into the input
        GameLevel(constants::MAX_INPUT_SIZE)
    }

    /// Change into the next level. Returns true if it's possible and false otherwise
    pub fn advance(&mut self) -> bool {
	if *self >= GameLevel::max() {
	    return false;
	}

	*self = GameLevel(self.0 + 1);
	true
    }

    pub fn max_solution(&self) -> u64 {
        10_u64.pow(self.0.into())
    }

    /// Calculates the optimal number of guesses for this level to be won.
    /// This is actually the maximum number of guesses it'd take for a binary
    /// search to find the solution, but rounded up.
    pub fn optimal_guesses(&self) -> u64 {
        let max_solution: f64 = 10_f64.powf((self.0).into());
        max_solution.log2().ceil() as u64
    }
}
