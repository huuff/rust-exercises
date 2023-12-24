use std::io::{Read, BufWriter, Write, BufReader, self};

// TODO: Test it!
pub fn piglatinize<W: Write, R: Read>(reader: &mut BufReader<R>, writer: &mut BufWriter<W>) -> Result<(), io::Error> {
    let mut current_word_buffer = Vec::new();
    for byte in reader.bytes() {
	let byte = byte?;

	if (byte as char).is_ascii_whitespace()  {
	    let pig_latin_word = piglatinize_word(&mut current_word_buffer);
	    write!(writer, "{pig_latin_word}")?;
	    write!(writer, "{}", byte as char)?;
	    current_word_buffer.clear();
	} else {
	    current_word_buffer.push(byte);
	}
    }

    Ok(())
}

fn piglatinize_word(word: &mut Vec<u8>) -> String {
    if word.is_empty() { return "".to_string() } 

    let first_letter = word[0];
    if is_vowel(&first_letter) {
	format!("{}-hay", String::from_utf8_lossy(word))
    } else {
	let first_letter = word.remove(0);
	format!("{}-{}ay", String::from_utf8_lossy(word), first_letter as char)
    }
}

// TODO: Rename this to vowel lol
const VOWELS: [u8; 10] = [
    b'a', b'e', b'i', b'o', b'u',
    b'A', b'E', b'I', b'O', b'U',
];
fn is_vowel(byte: &u8) -> bool {
    VOWELS.contains(&byte)
}

#[cfg(test)]
mod tests {
    use std::io::Stderr;

    use super::*;
    
    #[test]
    fn detects_vowel() {
       assert_eq!(is_vowel(&b'a'), true); 
    }

    #[test]
    fn detects_nonvowel() {
	assert_eq!(is_vowel(&b'n'), false);
    }

    #[test]
    fn latinizes_with_consonant() {
	let mut input = vec![b'p', b'a', b'c', b'o'];

	assert_eq!(piglatinize_word(&mut input), "aco-pay");
    }

    #[test]
    fn latinizes_with_vowel() {
	let mut input = vec![b'a', b'd', b'i', b'o', b's'];

	assert_eq!(piglatinize_word(&mut input), "adios-hay");
    }

    // TODO: This doesn't work because there's no whitespace at the end of the string, fix the function
    // so it also works with the end of buffer
    #[test]
    fn correctly_piglatinizes() {
	let input_string = String::from("hi how are you");
	let mut input = BufReader::new(input_string.as_bytes());
	let mut output_buffer = vec![];
	let mut output = BufWriter::new(&mut output_buffer);

	let result = piglatinize(&mut input, &mut output);
	// TODO: Lol, any better way?
	drop(output);

	assert!(result.is_ok());
	assert_eq!(String::from_utf8(output_buffer).unwrap(), String::from("i-hay ow-hay re-hay ou-yay"));
    }
}