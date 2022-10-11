/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        match is_palindrome(value) {
            false => None,
            true => Some(Palindrome(value)),
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

/// check if the given number is a palindrome or not
fn is_palindrome(value: u64) -> bool {
    let value = value.to_string();
    value == value.chars().rev().collect::<String>()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut smallest = None;
    // To find the smallest Palindrome number
    // loop from min number to max number
    // once found break the the loop so that
    // it does not keep on looping over large range
    for m in min..=max {
        // m * m is the smallest number that this loop can produce
        // if the smallest palidrome is found and smallest palindrome is
        // already less than m * m then no need to continue the loop anymore
        if smallest.is_some() && smallest.unwrap() <= m * m {
            break;
        }
        for n in m..=max {
            // replace the smallest with newly found palindrome
            if (smallest.is_none() || smallest.unwrap() > m * n) && is_palindrome(m * n) {
                smallest = Some(m * n);
                break;
            }
        }
    }
    // if the smallest palindrome not found at this point
    // then palindrome does not exists over given range
    // or invalid range is provided, return None
    let smallest = smallest?;
    let mut largest = smallest;
    // To find the largest palindrome number
    // loop from max to min
    // once palindrome is found break the loop
    for m in (min..=max).rev() {
        // m * m is the largest number that this loop can produce
        // if the largest is already greater than m * m
        // then no need to continue loop anymore
        if largest >= m * m {
            break;
        }
        for n in (min..=m).rev() {
            // replace largest with newly found palindrome
            if largest < m * n && is_palindrome(m * n) {
                largest = m * n;
                break;
            }
        }
    }
    // smallest and largest are known palindrome number
    // So Palindrome::new will not give None
    let smallest = Palindrome::new(smallest).unwrap();
    let largest = Palindrome::new(largest).unwrap();
    Some((smallest, largest))
}
