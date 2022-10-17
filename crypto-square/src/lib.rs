pub fn encrypt(input: &str) -> String {
    if input.len() < 3 {
        return String::from(input);
    }

    let v = input
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|ch| ch.to_ascii_lowercase())
        .collect::<Vec<char>>();
    let c = (v.len() as f64).sqrt() as usize;
    let c = if c * c == v.len() { c } else { c + 1 };
    let v = v.chunks(c).collect::<Vec<_>>();
    (0..c)
        .map(|c| {
            (0..v.len())
                .map(|r| match v[r].get(c) {
                    Some(ch) => *ch,
                    _ => ' ',
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
