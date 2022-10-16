// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTCH_ORBITAL_PERIOD: f64 = 31557600 as f64;
const MERCURY_WRT_EARTH: f64 = 0.2408467;
const VENUS_WRT_EARTH: f64 = 0.61519726;
const MARS_WRT_EARTH: f64 = 1.8808158;
const JUPITER_WRT_EARTH: f64 = 11.862615;
const SATURN_WRT_EARTH: f64 = 29.447498;
const URANUS_WRT_EARTH: f64 = 84.016846;
const NEPTUNE_WRT_EARTH: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    fn round_off_two_decimal_places(num: f64) -> f64 {
        f64::round(num * 100.0) / 100.0
    }
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        Self::round_off_two_decimal_places(d.0 / (EARTCH_ORBITAL_PERIOD * MERCURY_WRT_EARTH))
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        Self::round_off_two_decimal_places(d.0 / (EARTCH_ORBITAL_PERIOD * VENUS_WRT_EARTH))
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        Self::round_off_two_decimal_places(d.0 / EARTCH_ORBITAL_PERIOD)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        Self::round_off_two_decimal_places(d.0 / (EARTCH_ORBITAL_PERIOD * MARS_WRT_EARTH))
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        Self::round_off_two_decimal_places(d.0 / (EARTCH_ORBITAL_PERIOD * JUPITER_WRT_EARTH))
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        Self::round_off_two_decimal_places(d.0 / (EARTCH_ORBITAL_PERIOD * SATURN_WRT_EARTH))
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        Self::round_off_two_decimal_places(d.0 / (EARTCH_ORBITAL_PERIOD * URANUS_WRT_EARTH))
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        Self::round_off_two_decimal_places(d.0 / (EARTCH_ORBITAL_PERIOD * NEPTUNE_WRT_EARTH))
    }
}
