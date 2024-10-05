use std::fmt;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours_from_minutes, minutes_remaining) = Self::normalize_minutes(minutes);

        let modified_hours = hours + hours_from_minutes;

        let final_hours = Self::normalize_hours(modified_hours);

        Clock{hours: final_hours.abs(), minutes: minutes_remaining.abs()}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours_to_add, minutes_to_add) = Self::normalize_minutes(minutes);

        let new_hours = self.hours + hours_to_add;
        let new_minutes = self.minutes + minutes_to_add;

        Self::new(new_hours, new_minutes)
    }

    fn normalize_minutes(minutes: i32) -> (i32, i32) {
        let hours_from_minutes = minutes / 60;
        let minutes_remaining = minutes % 60;

        if minutes_remaining < 0 {
            return (hours_from_minutes - 1, 60 + minutes_remaining)
        }

        (hours_from_minutes, minutes_remaining)
    }

    fn normalize_hours(hours: i32) -> i32 {
        let remaining_hours = hours % 24;

        if remaining_hours < 0 {
            return 24 + remaining_hours
        }

        remaining_hours
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
