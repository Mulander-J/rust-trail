pub fn abbreviate(phrase: &str) -> String {
    /* Solution2 */
    format!(" {phrase} ")
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .filter_map(|window| {
            let left = window[0];
            let right = window[1];
            let include_right = "-_ ".contains(left) && right.is_alphabetic()
                || left.is_lowercase() && right.is_uppercase();
            match include_right {
                true => Some(right.to_ascii_uppercase()),
                false => None,
            }
        })
        .collect()

    /* Solution1 */
    // let mut is_continue = false;
    // let mut is_upper = false;
    //
    // phrase.chars().fold(String::new(), |mut str, mut c| {
    //     if c.is_alphabetic() {
    //         if !is_continue {
    //             // 单词首字母
    //             c = c.to_ascii_uppercase();
    //             str.push(c);
    //         } else {
    //             // 单词中错开的大写字母
    //             if c.is_uppercase() && !is_upper {
    //                 str.push(c);
    //             }
    //         }
    //         is_continue = true;
    //         is_upper = c.is_uppercase();
    //     } else if c != '\'' {
    //         is_continue = false;
    //         is_upper = false
    //     }
    //     str
    // })

    /* Solution3 */
    // phrase
    //     .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
    //     .flat_map(|word| {
    //         word.chars().take(1).chain(
    //             word.chars()
    //                 .skip_while(|c| c.is_uppercase())
    //                 .filter(|c| c.is_uppercase()),
    //         )
    //     })
    //     .collect::<String>()
    //     .to_uppercase()
}
