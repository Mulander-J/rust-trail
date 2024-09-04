/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some((b'z' - (c as u8 - b'a')) as char)
            } else if c.is_numeric() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some((b'a' + (b'z' - c as u8)) as char)
            } else if c.is_numeric() {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}
