// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

// no of car produced per hour
const CAR_PRODUCED_PER_HOUR: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        1..=4 => (CAR_PRODUCED_PER_HOUR * speed as u32) as f64,
        5..=8 => (CAR_PRODUCED_PER_HOUR * speed as u32) as f64 * 0.9,
        9..=10 => (CAR_PRODUCED_PER_HOUR * speed as u32) as f64 * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
