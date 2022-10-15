const QUINTILLION: u64 = 10u64.pow(18);
const QUADRILLION: u64 = 10u64.pow(15);
const TRILLION: u64 = 10u64.pow(12);
const BILLION: u64 = 10u64.pow(9);
const MILLION: u64 = 10u64.pow(6);
const THOUSAND: u64 = 10u64.pow(3);
const HUNDRED: u64 = 100;

pub fn encode(n: u64) -> String {
    match n {
        x if x >= QUINTILLION => large_number(n, QUINTILLION, "quintillion"),
        x if x >= QUADRILLION => large_number(n, QUADRILLION, "quadrillion"),
        x if x >= TRILLION => large_number(n, TRILLION, "trillion"),
        x if x >= BILLION => large_number(n, BILLION, "billion"),
        x if x >= MILLION => large_number(n, MILLION, "million"),
        x if x >= THOUSAND => large_number(n, THOUSAND, "thousand"),
        x if x >= HUNDRED => large_number(n, HUNDRED, "hundred"),
        x if x > 90 && x < 100 => format!("{}-{}", small_number(90), small_number(n - 90)),
        x if x > 80 && x < 90 => format!("{}-{}", small_number(80), small_number(n - 80)),
        x if x > 70 && x < 80 => format!("{}-{}", small_number(70), small_number(n - 70)),
        x if x > 60 && x < 70 => format!("{}-{}", small_number(60), small_number(n - 60)),
        x if x > 50 && x < 60 => format!("{}-{}", small_number(50), small_number(n - 50)),
        x if x > 40 && x < 50 => format!("{}-{}", small_number(40), small_number(n - 40)),
        x if x > 30 && x < 40 => format!("{}-{}", small_number(30), small_number(n - 30)),
        x if x > 20 && x < 30 => format!("{}-{}", small_number(20), small_number(n - 20)),
        _ => small_number(n),
    }
}

fn small_number(n: u64) -> String {
    match n {
        90 => "ninety".to_string(),
        80 => "eighty".to_string(),
        70 => "seventy".to_string(),
        60 => "sixty".to_string(),
        50 => "fifty".to_string(),
        40 => "forty".to_string(),
        30 => "thirty".to_string(),
        20 => "twenty".to_string(),
        19 => "nineteen".to_string(),
        18 => "eighteen".to_string(),
        17 => "seventeen".to_string(),
        16 => "sixteen".to_string(),
        15 => "fifteen".to_string(),
        14 => "fourteen".to_string(),
        13 => "thirteen".to_string(),
        12 => "tweleve".to_string(),
        11 => "eleven".to_string(),
        10 => "ten".to_string(),
        9 => "nine".to_string(),
        8 => "eight".to_string(),
        7 => "seven".to_string(),
        6 => "six".to_string(),
        5 => "five".to_string(),
        4 => "four".to_string(),
        3 => "three".to_string(),
        2 => "two".to_string(),
        1 => "one".to_string(),
        0 => "zero".to_string(),
        _ => unreachable!(),
    }
}

fn large_number(n: u64, d: u64, s: &str) -> String {
    if n % d == 0 {
        format!("{} {}", encode(n / d), s)
    } else {
        format!("{} {} {}", encode(n / d), s, encode(n % d))
    }
}
