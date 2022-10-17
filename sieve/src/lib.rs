use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }
    let mut primes = vec![];
    let mut non_primes = HashSet::new();
    let mut curr_prime = 2;
    while curr_prime <= upper_bound {
        if non_primes.get(&curr_prime).is_none() {
            primes.push(curr_prime);
            let mut num = curr_prime + curr_prime;
            while num <= upper_bound {
                non_primes.insert(num);
                num += curr_prime;
            }
        }
        curr_prime += 1;
    }

    primes
}
