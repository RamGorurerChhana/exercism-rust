use std::fmt::Display;

pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        // check if any non digit character except space exists
        if self.0.contains(|ch: char| !ch.is_ascii_whitespace() && !ch.is_ascii_digit()) {
            return false;
        }

        let mut code = self.0.chars()
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
                false if n < 5 => n * 2,
                _ => n * 2 - 9,
            })
            .sum::<u32>() % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
