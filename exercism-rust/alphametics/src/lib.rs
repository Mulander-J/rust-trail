use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut letter_set = HashSet::new();
    let mut adders: Vec<String> = vec![];
    let mut tmp = String::new();

    for c in input.chars().filter(|c| !c.is_whitespace()) {
        match c {
            '+' | '=' => {
                if !tmp.is_empty() {
                    adders.push(tmp);
                    tmp = String::new();
                }
            }
            c if c.is_ascii_alphabetic() => {
                letter_set.insert(c);
                tmp.push(c);
            }
            _ => return None,
        }
    }
    if !tmp.is_empty() {
        adders.push(tmp);
    }

    let letters: Vec<char> = letter_set.into_iter().collect();
    if letters.len() > 10 {
        return None;
    }

    for perm in (0..=9).permutations(letters.len()) {
        let mut char_to_digit = HashMap::new();
        for (i, &ch) in letters.iter().enumerate() {
            char_to_digit.insert(ch, perm[i]);
        }

        if adders.iter().any(|word| char_to_digit[&word.chars().next().unwrap()] == 0) {
            continue;
        }

        let word_values: Vec<u64> = adders.iter()
            .map(|word| {
                let num_str: String = word.chars()
                    .map(|ch| (char_to_digit[&ch] as u8 + b'0') as char)
                    .collect();
                num_str.parse::<u64>().unwrap()
            })
            .collect();

        let sum: u64 = word_values.iter().take(word_values.len() - 1).sum();
        let result = word_values[word_values.len() - 1];

        if sum == result {
            return Some(char_to_digit);
        }
    }

    None
}
