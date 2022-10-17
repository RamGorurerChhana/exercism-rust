pub fn rotate(input: &str, key: i8) -> String {
    // nomalize negative value in the key
    let key = (((key % 26) + 26) % 26) as u8;
    let upper_z = 'Z' as u8;
    let lower_z = 'z' as u8;
    let upper_a = 'A' as u8;
    let lower_a = 'a' as u8;
    let shift_upper_alpha = |ch: u8| {
        if ch + key > upper_z {
            (upper_a + (key - (upper_z - ch) - 1)) as char
        } else {
            (ch + key) as char
        }
    };
    let shift_lower_alpha = |ch: u8| {
        if ch + key > lower_z {
            (lower_a + (key - (lower_z - ch) - 1)) as char
        } else {
            (ch + key) as char
        }
    };
    let shift_alphabets = |ch: char| {
        let ch = ch as u8;
        if ch < upper_z {
            shift_upper_alpha(ch)
        } else {
            shift_lower_alpha(ch)
        }
    };

    input
        .chars()
        .map(|ch| match ch.is_ascii_alphabetic() {
            true => shift_alphabets(ch),
            false => ch,
        })
        .collect::<String>()
}
