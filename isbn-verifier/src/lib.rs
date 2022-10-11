/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // restrict the possibilities of incorrect dash characters
    if isbn.contains("--"){
        return false;
    }
    let mut digits = [0u32; 10];
    let mut idx = 0;
    for c in isbn.chars() {
        // if no of digits are more than 10 then return false
        if idx > 9 {
            return false;
        }

        // X can be valid character only at the last position
        if c == 'X' && idx == 9 {
            digits[idx] = 10;
            idx += 1;
        } else if c == '-' {
        } else {
            if let Some(d) = c.to_digit(10) {
                digits[idx] = d;
                idx += 1;
            } else {
                return false;
            }
        }
    }
    // if fewer than 10 digits are passed then return false
    if idx != 10 {
        return false;
    }

    // calculate the sum of all digits
    let sum = digits
        .iter()
        .enumerate()
        .map(|(idx, digit)| digit * (10 - idx) as u32)
        .sum::<u32>();

    // if sum is divided by 11 then valid ISBN
    sum % 11 == 0
}
