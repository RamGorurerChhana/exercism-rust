// This function determines whether a given number
// is an armstrong number or not.
// [Definition]: Armstrong number is a number
// that is the sum of its own digits each raised to
// the power of the number of digits.
// E.g: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = vec![];
    let mut x = num;
    // get all individual digits by repeatedly dividing the number by 10
    while x > 0 {
        let r = x % 10;
        digits.push(r);
        x /= 10;
    }
    // the power is the length of the digits vector
    let p = digits.len() as u32;
    // calculate the sum of power of each digits
    // then compare the sum with original number
    // if matches then return true
    num == digits.into_iter().map(|n| n.pow(p)).sum()
}
