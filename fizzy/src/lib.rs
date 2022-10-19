use std::{fmt::Display, ops::Rem};
/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    is_match: fn(T) -> bool,
    say: &'static str,
}
impl<T> Matcher<T> {
    pub fn new(matcher: fn(T) -> bool, subs: &'static str) -> Self {
        Self {
            is_match: matcher,
            say: subs,
        }
    }
    fn check_match(&self, val: T) -> Option<&'static str> {
        if (self.is_match)(val) {
            return Some(self.say);
        }
        None
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
    matchers: Vec<Matcher<T>>,
}
impl<T> Fizzy<T>
where
    T: Display + Copy,
{
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }
    /// map this fizzy onto every element of an interator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |n| {
            let say: String = self
                .matchers
                .iter()
                .filter_map(|matcher| matcher.check_match(n))
                .collect();
            if say.is_empty() {
                return format!("{}", n);
            }
            say
        })
    }
}
impl<T> Default for Fizzy<T>
where
    T: Display + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}
/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Rem<T> + Copy + Display + From<u8>,
    <T as Rem<T>>::Output: PartialEq<T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % T::from(5) == T::from(0), "buzz"))
}