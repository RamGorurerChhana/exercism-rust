#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

enum BaseType {
    Input,
    Output,
}

fn check_base(num: u32, base_type: BaseType) -> Result<bool, Error> {
    match num {
        0 | 1 => match base_type {
            BaseType::Input => Err(Error::InvalidInputBase),
            BaseType::Output => Err(Error::InvalidOutputBase),
        },
        _ => Ok(true),
    }
}

fn check_digit(num: u32, base: u32) -> Result<bool, Error> {
    if num < base {
        Ok(true)
    } else {
        Err(Error::InvalidDigit(num))
    }
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    check_base(from_base, BaseType::Input)?;
    check_base(to_base, BaseType::Output)?;
    let mut sum = 0;
    for (idx, &n) in number.into_iter().rev().enumerate() {
        if check_digit(n, from_base)? {
            sum += n * from_base.pow(idx as u32)
        }
    }
    let mut v = if sum > 0 { vec![] } else { vec![0] };
    while sum > 0 {
        v.push(sum % to_base);
        sum = sum / to_base;
    }

    Ok(v.into_iter().rev().collect())
}
