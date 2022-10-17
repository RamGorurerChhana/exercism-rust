pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a, T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
    let code = self.to_string();
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
}
