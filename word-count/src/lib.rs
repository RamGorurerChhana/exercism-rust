use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    words
        .split_ascii_whitespace()
        .flat_map(|w| w.split(","))
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .for_each(|w| {
            map.entry(w.to_lowercase().to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

    map
}
