

pub fn piglatinize_word(word: &mut Vec<u8>) -> String {
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
}
