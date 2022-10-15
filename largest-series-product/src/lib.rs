#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        Ok(1)
    } else {
        string_digits
            .chars()
            .map(to_digit)
            .collect::<Result<Vec<_>, Error>>()?
            .windows(span)
            .map(|v| v.iter().product::<u64>())
            .max()
            .ok_or(Error::SpanTooLong)
    }
}

fn to_digit(c: char) -> Result<u64, Error> {
    c.to_digit(10)
        .map(|n| n as u64)
        .ok_or(Error::InvalidDigit(c))
}
