#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)).map(|d| d as u64))
        .collect::<Result<Vec<u64>, Error>>()?
        .windows(span)
        .map(|window| window.iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}
