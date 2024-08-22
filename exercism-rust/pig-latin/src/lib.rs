fn translate_word(word: &str) -> String {
    /* Solution2 */
    let mut i = if word.starts_with('y') {
        word.find(|c| "aeiou".contains(c)).unwrap()
    } else {
        word.find(|c| "aeiouy".contains(c)).unwrap()
    };
    if i >= 1 && &word[i - 1..=i] == "qu" {
        i += 1
    }
    if &word[..2] == "yt" || "ay" == &word[i..] {
        i = 0
    }
    format!("{}{}ay", &word[i..], &word[..i])

    /* Solution1 */
    // // 定义一个字符集合来检查元音
    // let vowels = ['a', 'e', 'i', 'o', 'u'];
    //
    // // 检查单词是否为空
    // if word.is_empty() {
    //     return String::new();
    // }
    //
    // // 获取单词的首字符
    // let first_char = word.chars().next().unwrap();
    //
    // // 检查首字符是否是元音
    // return if vowels.contains(&first_char)
    //     || word.starts_with("xr")
    //     || word.starts_with("yt")
    // {
    //     // rule-1
    //     // 以元音开头的单词
    //     format!("{}ay", word)
    // } else {
    //     // 以辅音开头的单词
    //     let mut chars = word.chars();
    //     // 找到首个元音的位置
    //     let mut index = 0;
    //     while let Some(c) = chars.next() {
    //         if vowels.contains(&c) {
    //             break;
    //         }
    //         index += 1;
    //     }
    //
    //     // 拼接处理后的字符串
    //     let (prefix, rest) = word.split_at(index);
    //
    //     if prefix.ends_with('q') && rest.starts_with('u') {
    //         // rule-3
    //         return format!(
    //             "{}{}quay",
    //             rest.strip_prefix('u').unwrap_or(""),
    //             prefix.strip_suffix('q').unwrap_or("")
    //         );
    //     }
    //
    //     if let Some(y) = prefix.find('y') {
    //         if y > 0 {
    //             // rule-4
    //             return format!("{}{rest}{}ay", prefix.split_at(y).1, prefix.split_at(y).0);
    //         }
    //     }
    //
    //     // rule-2
    //     format!("{rest}{prefix}ay")
    // };


    /* Solution3 */
    // const VOWEL_SOUNDS: [&str; 7] = ["a", "e", "i", "o", "u", "xr", "yt"];
    // if word.is_empty() || VOWEL_SOUNDS.iter().any(|&s| word.starts_with(s)) {
    //     0
    // } else if word.starts_with("qu") {
    //     2
    // } else if word.get(1..2) == Some("y") {
    //     1
    // } else {
    //     1 + translate_word(&word[1..])
    // }
}
pub fn translate(str: &str) -> String {
    str.split_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}
