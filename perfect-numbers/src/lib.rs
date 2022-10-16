use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let sum = factors(num)?.into_iter().sum::<u64>();
    match num.cmp(&sum) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Deficient),
        Ordering::Less => Some(Classification::Abundant),
    }
}

fn factors(num: u64) -> Option<Vec<u64>> {
    match num {
        0 => None,
        1 => Some(vec![]),
        2 | 3 => Some(vec![1]),
        _ => {
            let mid = ((num as f64).sqrt()) as u64 + 2;
            let factors = (2..mid).fold(vec![1_u64], |mut acc, n| {
                if num % n == 0 {
                    acc.push(n);
                    if num / n >= mid {
                        acc.push(num / n);
                    }
                }
                acc
            });
            Some(factors)
        }
    }
}
