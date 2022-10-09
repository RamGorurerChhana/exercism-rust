pub fn square(s: u32) -> u64 {
    // validate no of square should be between 1 - 64
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }

    // no grains on a square is 2^(s-1)
    2_u64.pow(s - 1)
}

// sum(2^0 + 2^1 + 2^2 + ... + 2^63)
// = 2^(n+1) - 1 = 2^64 - 1
pub fn total() -> u64 {
    // Note: 2^64 will overflow in u64 type
    // but actual result is 2^64 - 1 which should fit in u64 without overflowing
    // Workaround: calculate below -
    // (2^63 - 1) * 2  + 1 = 2^64 - 2 + 1 = 2^64 - 1
    // Alternative: just return   u64::MAX
    (2_u64.pow(63) - 1) * 2 + 1
}
