pub fn encrypt(input: &str) -> String {
    let mut result = String::new();

    let pure: Vec<char> = input
        .chars()
        .filter_map(|x| {
            if x.is_ascii_alphanumeric() {
                Some(x.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect();

    let len = (pure.len() as f64).sqrt().ceil() as usize;

    if len == 0 {
        return result;
    }

    for i in 0..len {
        for chunk in pure.chunks(len) {
            result.push(*chunk.get(i).unwrap_or(&' '));
        }
        if i < len - 1 {
            result.push(' ');
        }
    }

    result
}
