pub fn encode(source: &str) -> String {
    let mut s = String::new();
    let mut last_char = None;
    let mut last_char_count = 0;
    for ch in source.chars() {
        if let Some(c) = last_char {
            if c == ch {
                last_char_count += 1;
            } else {
                if last_char_count > 1 {
                    s.push_str(last_char_count.to_string().as_str());
                }
                s.push(c);
                last_char = Some(ch);
                last_char_count = 1;
            }
        } else {
            last_char_count += 1;
            last_char = Some(ch);
        }
    }

    if let Some(c) = last_char {
        if last_char_count > 1 {
            s.push_str(last_char_count.to_string().as_str());
        }
        s.push(c);
    }

    s
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count = String::new();
    for ch in source.chars() {
        if ch.is_ascii_alphabetic() || ch == ' ' {
            match count.parse::<u32>() {
                Ok(count) => {
                    (0..count).for_each(|_| decoded.push(ch));
                }
                _ => {
                    decoded.push(ch);
                }
            }
            count = String::new();
        } else {
            count.push(ch);
        }
    }

    decoded
}
