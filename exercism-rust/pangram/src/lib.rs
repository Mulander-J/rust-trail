use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    /* Solution4 */
    sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        // .filter_map(|c| {
        //     if c.is_ascii_alphabetic() {
        //         Some(c.to_ascii_lowercase())
        //     } else {
        //         None
        //     }
        // })
        .try_fold(HashSet::with_capacity(26), |mut set, c| {
            set.insert(c);
            if set.len() == 26 {
                Err(set)
            } else {
                Ok(set)
            }
        })
        .err()
        .map_or(false, |set| set.len() == 26)

    /* Solution1 */
    // HashSet::from(
    //     sentence
    //         .chars()
    //         .filter_map(|c| {
    //             if !c.is_ascii_alphabetic() {
    //                 None
    //             } else {
    //                 Some(c.to_ascii_lowercase())
    //             }
    //         })
    //         .collect::<HashSet<_>>(),
    // )
    //     .len() == 26

    /* Solution2 */
    // let mut letters = HashSet::with_capacity(26);
    //
    // for c in sentence.chars() {
    //     if c.is_ascii_alphabetic() {
    //         letters.insert(c.to_ascii_lowercase());
    //     }
    //     if letters.len() == 26 {
    //         return true;
    //     }
    // }
    //
    // false

    /* Solution3 */
    // let s = sentence.to_lowercase();
    // ('a'..='z').all(|c| s.contains(c))
}
