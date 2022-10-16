use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let has_same_length = |ana: &&&str| ana.len() == word.len();
    let has_diff_char_in_same_pos = |ana: &&&str| {
        ana.chars().enumerate().any(|(i, ch)| {
            let nth = word.chars().nth(i).unwrap();
            match (ch.is_uppercase(), nth.is_uppercase()) {
                (true, true) => nth != ch,
                (true, false) => nth != ch.to_lowercase().next().unwrap(),
                (false, true) => nth != ch.to_uppercase().next().unwrap(),
                (false, false) => nth != ch,
            }
        })
    };
    let has_each_char_same_count = |ana: &&&str| {
        ana.chars().all(|ch| {
            let ch_inv = if ch.is_uppercase() {
                ch.to_lowercase().next().unwrap()
            } else {
                ch.to_uppercase().next().unwrap()
            };
            ana.chars().filter(|&k| k == ch || k == ch_inv).count()
                == word.chars().filter(|&k| k == ch || k == ch_inv).count()
        })
    };

    possible_anagrams
        .into_iter()
        .filter(has_same_length)
        .filter(has_diff_char_in_same_pos)
        .filter(has_each_char_same_count)
        .map(|&ana| ana)
        .collect()
}
