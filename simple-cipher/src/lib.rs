use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    // if key is invalid return None
    if key.len() == 0 || key.chars().any(|c| !c.is_alphabetic() || c.is_uppercase()) {
        return None;
    }
    // if s is blank then just return empty string
    if s.len() == 0 {
        return Some("".to_string());
    }
    // if s contains invalid character return None
    if s.chars().any(|c| !c.is_alphabetic() || c.is_uppercase()) {
        return None;
    }

    // encode s
    let a = 'a' as u8;
    let s = s
        .chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| {
            let k = k as u8;
            let c = c as u8;
            ((((k - a) + (c - a)) % 26) + a) as char
        })
        .collect::<String>();

    Some(s)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    // if key is invalid return None
    if key.len() == 0 || key.chars().any(|c| !c.is_alphabetic() || c.is_uppercase()) {
        return None;
    }
    // if s is blank then just return empty string
    if s.len() == 0 {
        return Some("".to_string());
    }
    // if s contains invalid character return None
    if s.chars().any(|c| !c.is_alphabetic() || c.is_uppercase()) {
        return None;
    }

    let a = 'a' as u8;
    // decode s
    let s = s
        .chars()
        .zip(key.chars().cycle())
        .map(|(c, k)| {
            let k = k as u8;
            let c = c as u8;
            (((26 - (k - a)) + (c - a)) % 26 + a) as char
        })
        .collect::<String>();

    Some(s)
}

pub fn encode_random(s: &str) -> (String, String) {
    // generate a random key length between 100 - 150
    let length = rand::thread_rng().gen_range(100..150);
    // for the chosen key length generate random character and convert to String
    let key = (0..length)
        .map(|_| rand::thread_rng().gen_range('a'..='z'))
        .collect::<String>();
    // enncode with the generated key
    let s = encode(&key, s).unwrap();
    // return key and encoded string
    (key, s)
}
