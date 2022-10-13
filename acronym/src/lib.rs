pub fn abbreviate(phrase: &str) -> String {
    // define a closure to check alphabetic uppercase char
    let is_alpha_upper = |c: char| c.is_alphabetic() && c.is_uppercase();
    phrase
        .split_ascii_whitespace()
        .flat_map(|p| p.split("-"))
        .map(|p| p.trim_matches(|c: char| !c.is_alphabetic()))
        .filter(|p| !p.is_empty())
        .fold(vec![], |mut words, p| {
            // if the words is all uppercase then take it as a whole
            if p.chars().all(char::is_uppercase) {
                words.push(&p[..]);
            } else {
                // check for the uppercase letter and split using index
                let mut idx = 0;
                // check for the uppercase alphabetic char except the first character
                while let Some(split_idx) = p[idx + 1..].find(is_alpha_upper) {
                    words.push(&p[idx..idx + split_idx + 1]);
                    idx = idx + split_idx + 1;
                }
                words.push(&p[idx..]);
            }
            words
        })
        .into_iter()
        .map(|p| p[0..1].to_uppercase())
        .collect()
}


// pub fn abbreviate(phrase: &str) -> String {
//     phrase
//         .split(|c: char| c.is_ascii_whitespace() || c == '_' || c == '-')
//         .flat_map(|word| {
//             word.chars().take(1).chain(
//                 word.chars()
//                     .skip_while(|c| c.is_ascii_uppercase())
//                     .filter(|c| c.is_ascii_uppercase()),
//             )
//         })
//         .collect::<String>()
//         .to_ascii_uppercase()
// }