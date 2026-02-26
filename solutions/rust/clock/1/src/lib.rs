use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: u8,
    minutes: u8,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    fn round_hours_minutes(mut hours: i32, mut minutes: i32) -> (u8, u8) {
        if minutes < 0 {
            let n = (-minutes + 59) / 60;
            hours -= n;
            minutes += 60 * n;
        } else if minutes >= 60 {
            hours += minutes / 60;
            minutes %= 60;
        }

        if hours < 0 {
            let n = (-hours + 23) / 24;
            hours += 24 * n;
        }
        ((hours % 24) as u8, minutes as u8)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Clock::round_hours_minutes(hours, minutes);
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) =
            Clock::round_hours_minutes(self.hours as i32, self.minutes as i32 + minutes);
        Self { hours, minutes }
    }
}
