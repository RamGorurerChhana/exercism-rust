use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    digits: Vec<char>,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = self.digits.iter().collect::<String>();
        write!(f, "{s}")
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        let mut num = num;
        let mut digits = vec![];
        let divisors = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        divisors.into_iter().for_each(|(d, s)| {
            let n = num / d;
            (0..n).for_each(|_| s.chars().for_each(|ch| digits.push(ch)));
            num = num % d;
        });

        Self { digits }
    }
}
