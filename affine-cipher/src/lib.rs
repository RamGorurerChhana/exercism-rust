/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

fn gcd(a: i32, b: i32) -> i32 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn is_coprime(a: i32, b: i32) -> bool {
    gcd(a, b) == 1
}

fn inverse(a: i32, n: i32) -> i32 {
    // See Wikipedia - https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;
    while newr != 0 {
        let quotient = r / newr;
        (t, newt) = (newt, t - quotient * newt);
        (r, newr) = (newr, r - quotient * newr);
    }
    if r > 1 {
        panic!("modular multiplicative inverse does not exists");
    }
    if t < 0 {
        t += n;
    }

    t
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let encode_char = |ch: char| {
        if ch.is_ascii_digit() {
            return ch;
        }
        let ch = if ch.is_ascii_uppercase() {
            ch.to_ascii_lowercase()
        } else {
            ch
        };
        let ch = ((ch as u8) - 'a' as u8) as i32;
        let e = ((a * ch + b) % 26) as u8;
        ('a' as u8 + e) as char
    };

    let encoded = plaintext
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(encode_char)
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ");
    Ok(encoded)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if !is_coprime(a, 26) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let inv = inverse(a, 26);

    let decode_char = |ch: char| {
        if ch.is_ascii_digit() {
            return ch;
        }
        let y = (ch as u8 - 'a' as u8) as i32;
        let y = ((((inv * (y - b)) % 26) + 26) % 26) as u8;
        ('a' as u8 + y) as char
    };

    let decoded = ciphertext
        .replace(" ", "")
        .chars()
        .map(decode_char)
        .collect::<String>();
    Ok(decoded)
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(2, 26), 2);
    assert_eq!(gcd(26, 2), 2);
    assert_eq!(gcd(14, 21), 7);
    assert!(is_coprime(5, 26));
    assert!(!is_coprime(13, 26));
}

#[test]
fn test_inverse() {
    assert_eq!(inverse(9, 26), 3);
    assert_eq!(inverse(15, 26), 7);
    assert_eq!(inverse(3, 11), 4);
}
