

pub fn piglatinize_word(word: &mut Vec<u8>) -> String {
    if word.is_empty() { return "".to_string() } 

    let first_letter = word[0];
    if is_vocal(&first_letter) {
	format!("{}-hay", String::from_utf8_lossy(word))
    } else {
	let first_letter = word.remove(0);
	format!("{}-{}ay", String::from_utf8_lossy(word), first_letter as char)
    }
}

const VOCALS: [u8; 10] = [
    b'a', b'e', b'i', b'o', b'u',
    b'A', b'E', b'I', b'O', b'U',
];
fn is_vocal(byte: &u8) -> bool {
    VOCALS.contains(&byte)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn detects_vocal() {
       assert_eq!(is_vocal(&b'a'), true); 
    }

    #[test]
    fn detects_nonvocal() {
	assert_eq!(is_vocal(&b'n'), false);
    }
}
