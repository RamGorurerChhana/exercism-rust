// use std::collections::HashMap;
// use crossbeam::thread;  

// pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
//     let mut counter = HashMap::new();
//     for worker_no in 0..worker_count{
//         thread::scope(|s|{
//             let handle = s.spawn(|_|{
//                 let mut counter = HashMap::new();
//                 let mut i = worker_no;
//                 while let Some(input) = input.get(i) {
//                     input.chars().filter(|ch| ch.is_alphabetic()).for_each(|ch|{
//                         let ch = if ch.is_uppercase() {ch.to_lowercase().next().unwrap()} else {ch};
//                         counter.entry(ch).and_modify(|count| *count += 1).or_insert(1usize);
//                     });
//                     i += worker_count;
//                 }
//                 counter
//             });
//             let c = handle.join().unwrap();
//             c.iter().for_each(|(&k, &v)|{
//                 counter.entry(k).and_modify(|c| *c += v).or_insert(v);
//             });
//         }).unwrap();
//     }

//     counter
// }


use rayon::prelude::*;
use std::collections::HashMap;
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    (0..worker_count)
        .into_par_iter()
        .map(|i| {
            let mut tallies = HashMap::new();
            input
                .join("")
                .chars()
                .skip(i)
                .step_by(worker_count)
                .filter(|c| c.is_alphabetic())
                .flat_map(|c| c.to_lowercase())
                .for_each(|c| {
                    *tallies.entry(c).or_insert(0) += 1;
                });
            tallies
        })
        .reduce(HashMap::new, |mut result, m| {
            m.iter().for_each(|(&k, &v)| {
                *result.entry(k).or_insert(0) += v;
            });
            result
        })
}
