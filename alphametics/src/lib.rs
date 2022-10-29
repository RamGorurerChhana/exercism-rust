use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct Puzzle {
    lhs: Vec<(char, u64)>,
    rhs: Vec<(char, u64)>,
    non_zeros: HashMap<char, ()>,
    all_chars: Vec<char>,
    solution: HashMap<char, u8>
}

impl Puzzle {
    fn new(input: &str) -> Option<Self> {
        let splitted = input.split("==").map(|s| s.trim()).collect::<Vec<_>>();
        let left = splitted[0].split('+').map(|s| s.trim()).collect::<Vec<_>>();
        let right = splitted[1].trim();
        if left.iter().any(|s| s.len() > right.len()) {
            return None;
        }
        let mut all_chars = vec![];
        let mut lhs: Vec<(char, u64)> = vec![];
        let mut rhs: Vec<(char, u64)> = vec![];
        let mut non_zeros = HashMap::new();
        left.iter().for_each(|s| collect_chars_unit_wise(s, &mut lhs, &mut all_chars, &mut non_zeros));
        collect_chars_unit_wise(right, &mut rhs, &mut all_chars, &mut non_zeros);
        Some(Self{lhs, rhs, non_zeros, all_chars, solution: HashMap::new()})
    }

    fn try_solve(&mut self, _idx: usize) -> bool {
        let permutations = (0..10).permutations(self.all_chars.len());
        'outer: for perm in permutations {
            let mut map = HashMap::new();
            for (i, &ch) in self.all_chars.iter().enumerate() {
                let val = perm[i];
                if val == 0 && self.non_zeros.contains_key(&ch){
                    continue 'outer;
                }
                map.insert(ch, val as u8);
            }
            if self.check_solution(&map){
                self.solution = map;
                return true;
            }
        }
        false
    }


    fn check_solution(&self, map: &HashMap<char, u8>) -> bool {
        let mut left_sum = 0u64;
        let mut right_sum = 0u64;
        for (c, u) in self.lhs.iter() {
            let v = map.get(&c).unwrap();
            left_sum += (*v  as u64) * (*u as u64);
        }
        for (c, u) in self.rhs.iter() {
            let v = map.get(&c).unwrap();
            right_sum += (*v  as u64) * (*u as u64);
        }
        left_sum == right_sum
    }

}

fn collect_chars_unit_wise(
    s: &str,
    unit_wise: &mut Vec<(char, u64)>, 
    all_chars: &mut Vec<char>, 
    non_zeros: &mut HashMap<char, ()>)
{
    let mut s = s.to_string();
    let mut unit_value = 1;
    if s.len() > 0 {
        non_zeros.insert(s.chars().nth(0).unwrap(), ());
    }
    while let Some(c) = s.pop() {
        if !all_chars.contains(&c){
            all_chars.push(c);
        }
        if let Some(v) = unit_wise.iter_mut().find(|v| v.0 == c){
            v.1 += unit_value;
        } else {
            unit_wise.push((c, unit_value));
        }
        unit_value *= 10;
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut puzzle = Puzzle::new(input)?;
    if puzzle.try_solve(0) {
        return Some(puzzle.solution);
    }
    None
}








