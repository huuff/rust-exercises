mod piglatin;

use std::io::{BufReader, self, Read};

// TODO: Make it write to a generic output buffer rather than stdout!
// TODO: And this way, I can test it
// TODO: Also let it take a generic buffer!
fn main() {
    let reader = BufReader::new(io::stdin());

    let mut current_word_buffer = Vec::new();
    for byte in reader.bytes() {
	let byte = byte.expect("Error while reading");

	if (byte as char).is_ascii_whitespace()  {
	    let pig_latin_word = piglatin::piglatinize_word(&mut current_word_buffer);
	    print!("{pig_latin_word}");
	    print!("{}", byte as char);
	    current_word_buffer.clear();
	} else {
	    current_word_buffer.push(byte);
	}
    }
}

