use std::collections::HashMap;

fn is_valid_num(s: &str) -> Option<bool> {
    if s.chars().all(|c| c.is_alphabetic() && c.is_uppercase()) {
        Some(true)
    } else {
        None
    }
}

fn get_lhs_rhs(input: &str) -> Option<(&str, &str)> {
    let idx = input.find("==")?;
    let (left, right) = input.split_at(idx);
    Some((left.trim(), right.trim()))
}



pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let map = HashMap::new();
    let (lhs, rhs) = get_lhs_rhs(input)?;
    is_valid_num(rhs)?;
    


    Some(map)
}
