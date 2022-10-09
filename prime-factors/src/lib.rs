// 1. calculate upperlimit sqrt(num)
// 2. start from 2 and repeatedly divide the number until 1
// 3. collect all divisible numbers into the vector
// 4. Corner Case: n < 2 -> should return empty vector
pub fn factors(n: u64) -> Vec<u64> {
    // calculate upperlimit
    let upperlimit = ((n as f64).sqrt() as u64) + 1;
    // declare a vector to hold all factors
    let mut factors = vec![];
    // declare n as mutable so that it can be divided repeatedly
    let mut n = n;
    // start checking prime factors from 2
    let mut p = 2_u64;
    // n will be divided by the prime factors until n = 1
    while n > 1 {
        // prime factors cannot be greater than upperlimit
        // then n is prime factor of original number
        if p > upperlimit {
            factors.push(n);
            break;
        }
        // if n is divisible then p is a factor
        // set n = n / p
        // n can have multiple factors of p so don't increase p at this point
        if n % p == 0 {
            factors.push(p);
            n = n / p;
        } else {
            // increase p and skips the even numbers
            p = if p == 2 { 3 } else { p + 2 };
        }
    }

    // return factors vector
    factors
}
