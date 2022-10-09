// Leap year rules -
// 1. divided by 4 e.g - 1996
// 2. not divided by 100 e.g - 1700
// 3. divided by 400 then override rule 2 e.g - 2000
pub fn is_leap_year(year: u64) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}
