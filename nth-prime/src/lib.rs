// 0 -> 2
// 1 -> 3
// 2 -> 5
pub fn nth(n: u32) -> u32 {
    // declare a vector to hold all calculated prime numbers
    let mut primes = vec![];
    // start calculating from 2
    let mut num = 2_u32;
    // for nth prime length of the vector would be n+1
    while primes.len() < (n as usize) + 1 {
        // push the prime number to primes vector
        if is_prime(num, &primes) {
            primes.push(num);
        }
        // increment num to check primality
        // all even numbers are skipped as those cannot be prime
        // push the prime number to primes vector
        num = if num == 2 { 3 } else { num + 2 };
    }

    // return the last number from primes vector
    // upwrap and dereference as `last` method return Option<&T>
    *primes.last().unwrap()
}

fn is_prime(num: u32, primes: &Vec<u32>) -> bool {
    // if num is divisible by any of the prime numbers
    // in the primes vector then its not a prime number
    // upperlimit is till num.sqrt()
    let upperlimit = ((num as f64).sqrt() as u32) + 1;
    for &p in primes.iter() {
        // if p is greater than upperlimit then break
        if p > upperlimit {
            break;
        }
        // if divisible not a prime
        if num % p == 0 {
            return false;
        }
    }

    true
}
