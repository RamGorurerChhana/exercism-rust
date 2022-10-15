use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours;
        let mut minutes = minutes;
        if minutes < 0 {
            let d = minutes / 60;
            let r = minutes % 60;
            if r == 0 {
                minutes = 0;
                hours += d;
            } else {
                minutes = r + 60;
                hours += d - 1;
            }
        } else {
            hours += minutes / 60;
            minutes %= 60;
        }

        hours %= 24;
        if hours < 0 {
            hours += 24;
        }

        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}
