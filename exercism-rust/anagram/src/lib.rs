use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let len = lower_word.len();
    let target = sort(&lower_word);

    possible_anagrams
        .iter()
        .copied()
        .filter(|pa| {
            let lower_pa = pa.to_lowercase();
            lower_pa.len() == len && lower_pa != lower_word && sort(&lower_pa) == target
        })
        .collect()

    // let mut set: HashSet<&str> = HashSet::with_capacity(possible_anagrams.len());

    // for pa in possible_anagrams {
    //     let mut item: Vec<char> = pa.to_lowercase().chars().collect();
    //     if item.len() != target.len() || pa.eq_ignore_ascii_case(word) || item == target {
    //         continue
    //     }
    //     item.sort();
    //     if item == target {
    //         set.insert(pa);
    //     }
    // }
    // set
}

fn sort(s: &str) -> Vec<char> {
    let mut res: Vec<char> = s.chars().collect();
    res.sort();
    res
}
