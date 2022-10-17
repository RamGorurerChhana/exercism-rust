/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // check if any non digit character except space exists
    if code.contains(|ch: char| !ch.is_ascii_whitespace() && !ch.is_ascii_digit()) {
        return false;
    }

    let mut code = code
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .collect::<Vec<_>>();
    // check length less than 2
    if code.len() < 2 {
        return false;
    }
    code.reverse();
    code.into_iter()
        .enumerate()
        .map(|(i, n)| match i % 2 == 0 {
            true => n,
            false if n < 5 =>  n * 2,
            _ => n * 2 - 9
        })
        .sum::<u32>() % 10 == 0
}
