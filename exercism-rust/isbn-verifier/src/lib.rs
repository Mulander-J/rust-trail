/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut count = 0;
    for c in isbn.chars().rev() {
        match c {
            '0'..='9' => {
                sum += (count + 1) * c.to_digit(10).unwrap_or(0);
            }
            'X' if count == 0 => {
                sum += 10;
            }
            '-' => continue,
            _ => return false,
        }
        count += 1;
    }
    count == 10 && sum % 11 == 0
}
