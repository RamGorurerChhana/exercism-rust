pub fn number(user_number: &str) -> Option<String> {
    // extract all digits only
    let mut digits = user_number
        .chars()
        .filter(char::is_ascii_digit)
        .map(|ch| ch.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    // reject if fewer than 10 digits
    // reject if more than 11 digits
    if digits.len() < 10 || digits.len() > 11 {
        return None;
    }

    // when 11 digits the first digit must be 1
    if digits.len() == 11 {
        if *digits.first()? != 1 {
            return None;
        }
        // remove first digit and make it 10 digits
        digits.remove(0);
    }
    // 1st and 4th digits must be 2 - 9
    let first = digits.first()?;
    let fourth = digits.get(3)?;
    if *first < 2 || *fourth < 2 {
        return None;
    }

    Some(
        digits
            .into_iter()
            .map(|n| char::from_digit(n, 10).unwrap())
            .collect::<String>(),
    )
}
