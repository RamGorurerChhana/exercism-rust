use std::{
    cmp::{max, Ordering},
    ops::{Add, Mul, Sub},
    str::FromStr,
};
use num_bigint::BigInt;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug)]
pub struct Decimal {
    // implement your type here
    integer_part: BigInt,
    decimal_part: BigInt,
    precision: usize,
    num_str: String,
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        self.integer_part == other.integer_part
            && self.precision == other.precision
            && self.decimal_part == other.decimal_part
    }
}
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let precision = max(self.precision, other.precision);
        let n1 = self.to_normalized_bigint(precision);
        let n2 = other.to_normalized_bigint(precision);
        Some(n1.cmp(&n2))
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let precision = max(self.precision, other.precision);
        let n1 = self.to_normalized_bigint(precision);
        let n2 = other.to_normalized_bigint(precision);
        let n = (n1 + n2).to_string();
        let n = format!("{:0>1$}", n, precision);
        let n = format!("{}.{}", &n[..n.len() - precision], &n[n.len() - precision..]);
        Self::try_from(&n).unwrap()
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let precision = max(self.precision, other.precision);
        let n1 = self.to_normalized_bigint(precision);
        let n2 = other.to_normalized_bigint(precision);
        let n = (n1 - n2).to_string();
        let n = format!("{:0>1$}", n, precision);
        let n = format!("{}.{}", &n[..n.len() - precision], &n[n.len() - precision..]);
        Self::try_from(&n).unwrap()
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let n1 = self.to_normalized_bigint(self.precision);
        let n2 = other.to_normalized_bigint(other.precision);
        let n = (n1 * n2).to_string();
        let precision = self.precision + other.precision;
        let n = format!("{:0>1$}", n, precision);
        let n = format!("{}.{}", &n[..n.len() - precision], &n[n.len() - precision..]);
        Self::try_from(&n).unwrap()
    }
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        // reject if input contains any non digit chars and more than 1 dot[.]
        if input.contains(|ch: char| !ch.is_ascii_digit() && ch != '.' && ch != '-' && ch != '+') {
            return None;
        }
        if !input.starts_with('-') && input.contains('-') {
            return None;
        }
        if input.starts_with('-') && input[1..].contains('-') {
            return None;
        }
        if !input.starts_with('+') && input.contains('+') {
            return None;
        }
        if input.starts_with('+') && input[1..].contains('+') {
            return None;
        }
        let dot_count = input.chars().filter(|&ch| ch == '.').count();
        if dot_count > 1 {
            return None;
        }
        let trimmed_input = if dot_count == 1 {
            input.trim_matches('0')
        } else {
            input
        };
        let (integer_part, decimal_part) = if dot_count == 1 {
            let idx = trimmed_input.find('.')?;
            (&trimmed_input[..idx], &trimmed_input[idx + 1..])
        } else {
            (&trimmed_input[..], "")
        };
        let integer_part = BigInt::from_str(integer_part).map_or(BigInt::from(0), |v| v);
        let precision = decimal_part.len();
        let decimal_part = BigInt::from_str(decimal_part).map_or(BigInt::from(0), |v| v);
        let precision = if decimal_part == BigInt::from(0) { 0 } else { precision };
        let decimal = Self {
            integer_part,
            precision,
            decimal_part,
            num_str: input.to_string(),
        };
        Some(decimal)
    }

    fn to_normalized_bigint(&self, precision: usize) -> BigInt {
        let trimmed_input = if self.num_str.contains('.') {
            self.num_str.trim_matches('0')
        } else {
            self.num_str.as_str()
        };
        let n = trimmed_input
            .replace(".", "")
            .chars()
            .chain((self.precision..precision).map(|_| '0'))
            .collect::<String>();
        BigInt::from_str(&n).map_or(BigInt::from(0), |v| v)
    }
}
