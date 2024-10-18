pub fn rotate(input: &str, key: u8) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let i = (c as u8 - base + key).rem_euclid(26);
            (base + i) as char
        } else {
            c
        }
    }).collect()
}
