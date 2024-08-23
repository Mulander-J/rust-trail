pub fn encode(source: &str) -> String {
    let chars: Vec<_> = source.chars().enumerate().collect();
    let last_index = chars.len().saturating_sub(1);
    let mut count = 1;

    chars
        .windows(2)
        .map(|window| match window {
            &[(_, prev), (index, next)] => {
                let mut part = String::new();
                if prev == next {
                    count += 1;
                } else {
                    if count > 1 {
                        part.push_str(&count.to_string());
                    }
                    part.push(prev);
                    count = 1;
                }
                if last_index == index {
                    if count > 1 {
                        part.push_str(&count.to_string());
                    }
                    part.push(next)
                }
                part
            }
            _ => unreachable!(),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    // let mut decoded = String::new();
    // let mut count_str = String::new(); // 用于累积数字字符
    //
    // for c in source.chars() {
    //     if c.is_digit(10) {
    //         // 如果当前字符是数字，将其加入 count_str
    //         count_str.push(c);
    //     } else {
    //         decoded.push_str(
    //             &c.to_string()
    //                 .repeat(count_str.parse::<usize>().unwrap_or(1)),
    //         ); // 根据 count 追加字符 c
    //         count_str.clear(); // 清空 count_str 以处理下一个字符
    //     }
    // }
    //
    // decoded

    source
        .chars()
        .fold((String::new(), 0), |(mut acc, last_n), c| {
            if let Some(d) = c.to_digit(10) {
                (acc, 10 * last_n + d)
            } else {
                acc += c.to_string()
                    .repeat(std::cmp::max(last_n, 1) as usize).as_str();
                (acc, 0)
            }
        }).0
}
