pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    let mut n = n;
    while n > 1 {
        steps += 1;
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n.checked_mul(3)?;
            n = n.checked_add(1)?;
        }
    }
    Some(steps)
}
