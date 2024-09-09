const VALID_LEN: usize = 10;
pub fn number(user_number: &str) -> Option<String> {
    user_number
        .chars()
        .filter(|x| x.is_numeric())
        .try_fold((String::new(), 0_usize), |(mut acc, len), c| {
            match len {
                0 if c == '1' => Some((acc, len)),
                0 | 3 if c.to_digit(10).unwrap() < 2 => None,
                n if n > VALID_LEN => None,
                _ => {
                    acc.push(c);
                    let len = acc.len();
                    Some((acc, len))
                }
            }
        })
        .and_then(|(x, len)| if len == VALID_LEN { Some(x) } else { None })
}
