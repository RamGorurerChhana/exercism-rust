/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|ch| ch.to_ascii_lowercase())
        .map(|ch| match ch.is_ascii_alphabetic() {
            true => ('a' as u8 + ('z' as u8 - ch as u8)) as char,
            false => ch,
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|ch| match ch.is_ascii_alphabetic() {
            true => ('a' as u8 + ('z' as u8 - ch as u8)) as char,
            false => ch,
        })
        .collect()
}
