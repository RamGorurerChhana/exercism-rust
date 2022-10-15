/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    ('a'..='z')
        .all(|ch| sentence.find(ch).is_some() || sentence.find(ch.to_ascii_uppercase()).is_some())
}
