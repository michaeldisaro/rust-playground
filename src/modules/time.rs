use std::fmt::Formatter;
use std::fmt::Display;
use std::ops::Add;

#[derive(Debug, Default)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }.normalize()
    }

    fn normalize(&self) -> Self {
        let mut hours = (self.hours + self.minutes / 60) % 24;
        let mut minutes = self.minutes % 60;

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        if hours < 0 {
            hours += 24;
        }

        Self { hours, minutes }
    }
}

/// Implement Display trait
impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "It's {:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

/// Implement Add trait
impl Add for Clock {
    type Output = Clock;

    fn add(self, rhs: Self) -> Self::Output {
        Clock::new(self.hours + rhs.hours, self.minutes + rhs.minutes)
    }
}