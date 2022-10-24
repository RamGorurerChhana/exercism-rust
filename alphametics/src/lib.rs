use std::collections::{HashMap, HashSet};


pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let splitted = input.split("==").collect::<Vec<_>>();
    let mut lhs = splitted[0].split("+").map(|s| s.trim().to_string()).collect::<Vec<_>>();
    let mut rhs = splitted[1].trim().to_string();
    let mut unit_wise = Vec::with_capacity(rhs.len());
    let mut non_zeros = HashSet::new();
    lhs.iter().for_each(|s|{
        if s.len() > 1 {
            non_zeros.insert(s.chars().nth(0).unwrap());
        }
    });
    non_zeros.insert(rhs.chars().nth(0)?);
    if lhs.iter().any(|s| s.len() > rhs.len()) {
        return None;
    }

    for _ in 0..rhs.len() {
        let mut v = vec![];
        let mut s = vec![];
        for i in 0..lhs.len() {
            if let Some(e) = lhs[i].pop() {
                v.push(e);
                if !s.contains(&e){
                    s.push(e);
                }
            }
        }
        unit_wise.push((v, rhs.pop().unwrap(), s));
    }
    let mut map = HashMap::new();
    if let Some(r) = try_solve(&mut map, &unit_wise, &non_zeros, 0, 0, 0) {
        println!("recusrion count: {}", r);
        Some(map)
    } else {
        None
    }
}


fn try_solve(map: &mut HashMap<char, u8>, unit_wise: &Vec<(Vec<char>, char, Vec<char>)>, non_zeros: &HashSet<char>, curr_idx: usize, previous_carry: u8, recursion_count: u64) -> Option<u64> {
    let recursion_count = recursion_count + 1;
    println!("recursion_count: {recursion_count}");
    // let mut char_to_pop = None;
    if curr_idx >= unit_wise.len() {
        return Some(recursion_count);
    }

    // try to assign some value for all character in the lhs
    for ch in &unit_wise[curr_idx].2 {
        if map.contains_key(&ch){
            continue;
        }
        for n in 0u8..10{
            if n == 0 && non_zeros.contains(ch) {
                continue;
            }
            if map.values().any(|&v| v==n){
                continue;
            }
            println!("Insert lhs {}: {} into map", *ch, n);
            map.insert(*ch, n);
            // char_to_pop = Some(*ch);
            if let Some(r) =  try_solve(map, unit_wise, non_zeros, curr_idx, previous_carry,  recursion_count){
                return Some(r);
            }
            map.remove(&ch);
        }
        println!("Exhausted possibilities for lhs {}", *ch);
        // exhausted all possibilities for rhs return None
        return None;
    }

    let rhs = unit_wise[curr_idx].1;
    if !map.contains_key(&rhs){
        // try assign some value to rhs
        for n in 0u8..10{
            if map.values().any(|&v| v==n){
                continue;
            }
            if n == 0 && non_zeros.contains(&rhs) {
                continue;
            }
            println!("Insert rhs {}: {} into map", rhs, n);
            map.insert(rhs, n);
            if let Some(r) =  try_solve(map, unit_wise, non_zeros, curr_idx, previous_carry, recursion_count) {
                return Some(r);
            }
            map.remove(&rhs);
        }
        println!("Exhausted possibilities for rhs {}", rhs);
        // exhausted all possibilities for rhs return None
        return None;
    }

    // all values in lhs and all values in rhs are assigned values 
    // now check if equation holds till so far 
    let mut sum = previous_carry as u32;
    for ch in &unit_wise[curr_idx].0 {
        sum += *(map.get(ch).unwrap()) as u32;
    }
    let right = map.get(&rhs).unwrap();
    if sum % 10 == (*right) as u32 {
        println!("{:?}", map);
        println!("Solution found for {curr_idx}, moving forward -->");
        // equation holds in the current unit position check next 
        if let Some(r) = try_solve(map, unit_wise, non_zeros, curr_idx + 1, (sum/10) as u8, recursion_count) {
            return Some(r);
        }
    }
    println!("{:?}", map);
    println!("Solution not found for {curr_idx}, moving backward <--");
    // otherwise current value is not fit for rhs 
    // map.remove(&rhs);
    None
}



