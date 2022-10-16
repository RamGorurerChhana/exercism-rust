const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const VOWELS_Y: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

pub fn translate(input: &str) -> String {
    // if input is multiple words then split and translate each word
    if input.contains(char::is_whitespace) {
        return input
            .split_ascii_whitespace()
            .map(|word| translate(word))
            .collect::<Vec<_>>()
            .join(" ");
    }

    // Rule #1: if words starts with vowel or xr or yt
    if input.starts_with(&VOWELS) || input.starts_with("xr") || input.starts_with("yt") {
        return format!("{input}ay");
    }

    // Rule#3: if word contains qu after all consonants characters
    if input.contains("qu") {
        let idx = input.find("qu").unwrap();
        if input.chars().take(idx).all(|ch| !VOWELS_Y.contains(&ch)) {
            return format!("{}{}ay", &input[idx + 2..], &input[0..idx + 2]);
        }
    }

    // Rule#2 & #4: if word contains vowel or y after all consonants
    if input.contains(&VOWELS_Y) {
        let idx = input.find(&VOWELS_Y).unwrap();
        // if y appears at the beginning then shift idx
        // other vowels cannot appear at the beginning at this point
        let idx = if idx == 0 { 1 } else { idx };
        return format!("{}{}ay", &input[idx..], &input[0..idx]);
    }

    // if no case matched return the original string
    input.to_string()
}
