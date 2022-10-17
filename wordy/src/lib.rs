pub fn answer(command: &str) -> Option<i32> {
    if !command.ends_with('?') {
        return None;
    }
    let command = &command[..command.len() - 1];
    let mut iter = command.split_ascii_whitespace();
    if "What" != iter.next()? {
        return None;
    }
    if "is" != iter.next()? {
        return None;
    }
    let mut last_result = None;
    while let Some(val) = iter.next() {
        match val.parse::<i32>() {
            Ok(v) => {
                if last_result.is_some() {
                    return None;
                }
                last_result = Some(v);
            }
            Err(_) => {
                match val {
                    "plus" => {
                        let e1 = last_result?;
                        let e2 = iter.next()?.parse::<i32>().ok()?;
                        let r = e1 + e2;
                        last_result = Some(r);
                    }
                    "minus" => {
                        let e1 = last_result?;
                        let e2 = iter.next()?.parse::<i32>().ok()?;
                        let r = e1 - e2;
                        last_result = Some(r);
                    }
                    "multiplied" => {
                        // then blank move for "by"
                        iter.next()?;
                        let e1 = last_result?;
                        let e2 = iter.next()?.parse::<i32>().ok()?;
                        let r = e1 * e2;
                        last_result = Some(r);
                    }
                    "divided" => {
                        // then blank move for "by"
                        iter.next()?;
                        let e1 = last_result?;
                        let e2 = iter.next()?.parse::<i32>().ok()?;
                        let r = e1 / e2;
                        last_result = Some(r);
                    }
                    "raised" => {
                        // then blank move for "to the"
                        iter.next()?;
                        iter.next()?;
                        let e1 = last_result?;
                        let e2 = iter
                            .next()?
                            .replace(|ch: char| !ch.is_ascii_digit(), " ")
                            .trim()
                            .parse::<i32>()
                            .ok()?;
                        let r = e1.pow(e2 as u32);
                        last_result = Some(r);
                        // blank move for power
                        iter.next()?;
                    }
                    _ => {
                        return None;
                    }
                }
            }
        }
    }
    last_result
}
