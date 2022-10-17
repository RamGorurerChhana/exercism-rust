use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();
    note.into_iter().for_each(|s| {
        map.entry(s).and_modify(|count| *count += 1).or_insert(1);
    });

    for s in magazine {
        if let Some(count) = map.get_mut(s) {
            match *count {
                1 => {
                    map.remove(s);
                }
                _ => *count -= 1,
            };
        }
        if map.len() == 0 {
            break;
        }
    }

    map.len() == 0
}
