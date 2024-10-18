extern crate rand;
use rand::Rng;

fn shift(key: &str, s: &str, direction: bool) -> Option<String> {
    let key_len = key.len();
    if key_len == 0 || !key.chars().all(|ck| ck.is_ascii_alphabetic() && ck.is_ascii_lowercase()) {
        return None;
    }

    let base = b'a';
    Some(s.chars().zip(key.chars().cycle()).map(|(cp, ck)| {
        let shift_value = if direction {
            (cp as u8 - base + ck as u8 - base).rem_euclid(26)
        } else {
            (cp as i8 - ck as i8).rem_euclid(26) as u8
        };
        (base + shift_value) as char
    }).collect())
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    shift(key, s, true)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    shift(key, s, false)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut r = rand::thread_rng();
    let mut key = String::new();

    for _ in 0..100 {
        key.push(char::from(b'a' + r.gen_range(0..26) as u8));
    }
    let encoded = encode(&key, s);
    (key.clone(), encoded.unwrap_or(key))
}