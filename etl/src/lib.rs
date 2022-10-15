use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.into_iter()
        .flat_map(|(&n, v)| v.into_iter().map(move |&ch| ((ch as u8 + 32) as char, n)))
        .collect()
}
