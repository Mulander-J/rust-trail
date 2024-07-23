pub fn brackets_are_balanced(string: &str) -> bool {
    let mut balances = vec![];

    for ch in string.chars() {
        match ch {
            '[' | '{' | '(' => balances.push(ch),
            ']' | '}' | ')' => match (balances.pop().unwrap_or('_'), ch) {
                ('[', ']') => {}
                ('{', '}') => {}
                ('(', ')') => {}
                _ => return false,
            },
            _ => {}
        }
    }

    balances.is_empty()
}
