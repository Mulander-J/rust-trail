const NUMBER_WORDS: [&str; 11] = [
    "no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
];

// 🚀 单复数处理函数
fn plural_s(n: u32) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}

// 🎯 动态大小写转换
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

// 📦 生成单个歌词段落
fn generate_verse(current: u32) -> String {
    let next = current.saturating_sub(1);

    // 安全获取数字单词（处理越界情况）
    let current_word = NUMBER_WORDS
        .get(current as usize)
        .map(|&s| capitalize(s))
        .unwrap_or_else(|| current.to_string());

    let next_word = NUMBER_WORDS
        .get(next as usize)
        .map(|&s| s.to_string())
        .unwrap_or_else(|| next.to_string());

    format!(
        "{current_word} green bottle{current_suffix} hanging on the wall,\n\
         {current_word} green bottle{current_suffix} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {next_word} green bottle{next_suffix} hanging on the wall.\n",
        current_suffix = plural_s(current),
        next_suffix = plural_s(next)
    )
}

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let start = start_bottles.saturating_sub(take_down - 1);
    (start..=start_bottles)
        .rev()
        .map(|n| n.min(10)) // 🔒 限制最大处理到10
        .filter(|&n| n > 0) // 🚧 过滤无效数值
        .map(generate_verse)
        .collect::<Vec<_>>()
        .join("\n") // 💡 段间用空行分隔
}
