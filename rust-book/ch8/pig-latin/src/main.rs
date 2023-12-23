mod piglatin;

use std::io::{BufReader, self, BufWriter};

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    
    let result = piglatin::piglatinize(&mut reader, &mut writer);

    if let Err(error) = result {
	eprintln!("Error: {}", error);
    }
}

