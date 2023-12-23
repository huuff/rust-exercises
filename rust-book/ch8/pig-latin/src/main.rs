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
	    let pig_latin_word = piglatinize(&mut current_word_buffer);
	    print!("{pig_latin_word}");
	    print!("{}", byte as char);
	    current_word_buffer.clear();
	} else {
	    current_word_buffer.push(byte);
	}
    }
}

const VOCALS: [u8; 10] = [
    b'a', b'e', b'i', b'o', b'u',
    b'A', b'E', b'I', b'O', b'U',
];
fn is_vocal(byte: &u8) -> bool {
    VOCALS.contains(&byte)
}

fn piglatinize(word: &mut Vec<u8>) -> String {
    if word.is_empty() { return "".to_string() } 

    let first_letter = word[0];
    if is_vocal(&first_letter) {
	format!("{}-hay", String::from_utf8_lossy(word))
    } else {
	let first_letter = word.remove(0);
	format!("{}-{}ay", String::from_utf8_lossy(word), first_letter as char)
    }
}
