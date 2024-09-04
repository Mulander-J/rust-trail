pub fn get_diamond(c: char) -> Vec<String> {
    if !c.is_ascii_alphabetic() {
        return vec![];
    }

    let end_char = c.to_ascii_uppercase();
    if end_char == 'A' {
        return vec!["A".to_string()];
    }

    let width = ((end_char as usize) - ('A' as usize)) * 2 + 1;

    let mut ret: Vec<String> = ('A'..=end_char)
        .enumerate()
        .map(|(i, x)| {
            format!(
                "{:^width$}",
                if i == 0 {
                    "A".to_string()
                } else {
                    format!("{x}{:space$}{x}", "", space = 2 * i - 1)
                }
            )
        })
        .collect();

    ret.extend(
        ret.iter()
            .rev()
            .skip(1).cloned().collect::<Vec<_>>()
    );

    ret
}
