use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    let mut first = 1;
    while first <= sum / 3 {
        let mut second = first + 1;
        while second <= sum / 2 {
            let third = sum - first - second;
            let triplet = [first, second, third];
            if is_valid_triplet(triplet, sum) {
                set.insert(triplet);
            }
            second += 1;
        }
        first += 1;
    }
    set
}

fn is_valid_triplet(triplet: [u32; 3], num: u32) -> bool {
    triplet[0] < triplet[1]
        && triplet[1] < triplet[2]
        && triplet[0] + triplet[1] + triplet[2] == num
        && triplet[0] * triplet[0] + triplet[1] * triplet[1] == triplet[2] * triplet[2]
}



//
// THIS SOLUTION I FIND IN EXERCISM IS VERY FAST
// WORKS PERFECTLY FOR BIG NUMBERS
// 
// use std::collections::HashSet;
// use num_integer::gcd;
// pub fn find(sum: u32) -> HashSet<[u32; 3]> {
//     let mut result:HashSet<[u32; 3]> = HashSet::new();
//     let half_sum = sum as f64 / 2.0;
//     let m_limit = half_sum.sqrt().ceil() as u32 -1;
//     let (mut k, mut abc);
//     let half_sum = half_sum as u32;
//     for m in 2..=m_limit {
//         if half_sum % m == 0 {
//             let mut sm = half_sum / m;
//             while sm % 2 == 0 { sm /= 2; }
//             if m % 2 == 1 { k = m + 2; }
//             else {  k = m +1; }
//             while k < 2 * m && k <= sm {
//                 if sm % k == 0 && gcd(k, m) == 1 {
//                     let d = half_sum / (k * m);
//                     let n = k - m;
//                     let a = d * (m * m - n * n);
//                     let b = 2 * d * m * n;
//                     let c = d * (m * m + n * n);
//                     if a + b + c == sum {
//                         abc = [a, b, c];
//                         if a > b {  abc.swap(0, 1); }
//                         result.insert(abc);
//                     }
//                 }
//                 k = k +2;
//             }
//         }
//     }
//    result
// }
