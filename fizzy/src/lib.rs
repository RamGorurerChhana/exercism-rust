// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::{fmt::Display, ops::Rem};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    sub: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, sub: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: Display,
    {
        Self {
            matcher: Box::new(matcher),
            sub: sub.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    custom_matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Display + From<u8> + Rem<Output = T> + Copy + PartialEq,
{
    pub fn new() -> Self {
        Self {
            custom_matchers: vec![],
        }
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.custom_matchers.push(matcher);
        self
    }

    fn apply_matcher(&self, element: T) -> String {
        if self.custom_matchers.len() == 0 {
            return self.default_matcher(element);
        }

        let mut s = String::new();
        for m in self.custom_matchers.iter() {
            if (m.matcher)(element) {
                s.push_str(m.sub.as_str());
            }
        }

        if s.is_empty() {
            element.to_string()
        } else {
            s
        }
    }

    fn default_matcher(&self, element: T) -> String {
        let three: T = 3u8.into();
        let five: T = 5u8.into();
        let zero: T = 0u8.into();
        match (element % three == zero, element % five == zero) {
            (true, true) => "fizzbuzz".to_string(),
            (false, true) => "buzz".to_string(),
            (true, false) => "fizz".to_string(),
            (false, false) => element.to_string(),
        }
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        iter.map(move |element| self.apply_matcher(element))
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Display + From<u8> + Rem<Output = T> + Copy + PartialEq,
{
    Fizzy::new()
}
