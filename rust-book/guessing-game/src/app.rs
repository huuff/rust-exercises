use crate::constants;



pub struct App {
    pub input: String,
}

impl App {
    pub fn new() -> Self {
	Self {
	    input: String::new(),
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
}
