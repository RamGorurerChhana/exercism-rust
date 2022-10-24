use std::collections::HashSet;

type I = [(u8, u8)];
type V = Vec<(u8, u8)>;
type S = HashSet<usize>;

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut v = Vec::with_capacity(input.len());
    let mut set = HashSet::with_capacity(input.len());

    solve(input, &mut v, &mut set)?;
    Some(v)
}

fn solve(input: &I, v: &mut V, used: &mut S) -> Option<()> {
    if input.len() == 0 || (v.len() == input.len() && v.first()?.0 == v.last()?.1) {
        return Some(());
    }
    for curr in 0..input.len() {
        if used.contains(&curr) {
            continue;
        }
        let item = input[curr];
        let mut is_reverse_checked = item.0 == item.1;
        if can_be_pushed(v, item) {
            v.push(item);
            used.insert(curr);
        } else if can_be_pushed(v, (item.1, item.0)) {
            is_reverse_checked = true;
            v.push((item.1, item.0));
            used.insert(curr);
        } else {
            continue;
        }
        if solve(input, v, used).is_some() {
            return Some(());
        }
        v.pop();
        used.remove(&curr);
        if !is_reverse_checked && can_be_pushed(v, (item.1, item.0)) {
            v.push((item.1, item.0));
            used.insert(curr);
            if solve(input, v, used).is_some() {
                return Some(());
            }
            v.pop();
            used.remove(&curr);
        }
    }

    None
}

fn can_be_pushed(v: &V, item: (u8, u8)) -> bool {
    if v.len() == 0 {
        true
    } else {
        v.last().unwrap().1 == item.0
    }
}
