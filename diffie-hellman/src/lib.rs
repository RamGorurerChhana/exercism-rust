use rand::Rng;

// generate a random number between 2 and p
// check if it is prime
// or return the next prime number
// return 2 if there is no prime number between the random number and p
pub fn private_key(p: u64) -> u64 {
    // for ease of calculation we will restrict the upperlimit to 100
    // so even if a big number is passed in the parameter 
    // generated random prime number will be less than 100 
    let p = if p < 100 {p} else {100};
    // generate a random number between 2 and p
    let mut num = rand::thread_rng().gen_range(2..p);
    // find and return the prime number less than p
    while num < p {
        if is_prime(num) {
            return num;
        }
        num += 1;
    }
    // by default return 2
    2
}

// g^a mod p 
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    calc_mod_pow(g, a, p)
}

// b_pub^a mod p 
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    calc_mod_pow(b_pub, a, p)
}


// check if a number is prime number or not
fn is_prime(p: u64) -> bool {
    let upperlimit = ((p as f64).sqrt() as u64) + 1;
    (2..upperlimit).all(|x| p % x != 0)
}


// Right-to-left binary method will be used for faster calculation 
// See here - https://en.wikipedia.org/wiki/Modular_exponentiation
// convert base, exponent, modulus to u128 before making any calculation
// this is to prevent overflow for big-primes
// Alternatively check if the numbers are big enough to overflow
// then only convert to u128 otherwise continue operation in u64
fn calc_mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    // if modulus is 1 then result will always be zero 
    if modulus == 1 {
        return 0;
    }

    let m = modulus as u128;
    let mut e = exponent as u128;
    let mut b = (base as u128) % m;
    let mut r = 1;
    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % m;
        }
        b = (b*b) % m;
        e = e >> 1;
    }
    // convert the result back to u64
    r as u64
}
