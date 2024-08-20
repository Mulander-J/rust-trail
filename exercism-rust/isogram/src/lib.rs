use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    // let pattern = |&c: &char| c != '-' && !c.is_whitespace();
    // let set: HashSet<_> = candidate.chars()
    //     .filter(pattern)
    //     .map(|c| c.to_ascii_lowercase())
    //     .collect();
    //
    // set.len() == candidate.chars().filter(pattern).count()

    let mut seen = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        // .map(|c| c.to_ascii_lowercase())
        .all(|c| seen.insert(c))
}
