use chrono::{TimeZone, Datelike, Duration, Timelike};
use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

fn get_new_time(clock: &Clock, hours: i32, minutes: i32) -> Clock {
    let now = chrono::Local::now();
    let dt =  chrono::Local.ymd(now.year(), now.month(), now.day()).and_hms(clock.hours as u32, clock.minutes as u32, 0);

    let new_time = dt + Duration::hours(hours as i64) + Duration::minutes(minutes as i64);

    return Clock { hours: new_time.hour() as i32, minutes: new_time.minute() as i32}
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_time = get_new_time(&Clock { hours:  0, minutes: 0 }, hours, minutes);
        
        Self { hours: new_time.hours, minutes: new_time.minutes}
    }


    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_time = get_new_time(self, 0, minutes);

        Self {
            hours: new_time.hours as i32,
            minutes: new_time.minutes as i32
        }
    }

    // pub fn to_string(&self) -> String {
    //     return format!("{:02}:{:02}", self.hours, self.minutes)
    // }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:02}:{:02}", self.hours, self.minutes);
    }
}
