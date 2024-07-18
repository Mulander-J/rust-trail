/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut index = 0;
    let mut sum: u32 = 0;

    for c in code.chars().rev() {
        match c {
            '0'..='9' => {
                let mut n = (c as u32 - '0' as u32) * if index % 2 == 1 { 2 } else { 1 };
                if n > 9 {
                    n -= 9;
                }
                sum += n;
                index += 1;
            }
            s if s.is_whitespace() => {}
            _ => return false,
        }
    }

    sum % 10 == 0 && index > 1
}
