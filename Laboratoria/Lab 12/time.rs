// TODO
// new(dd, hh, mm, ss)
// to_string24() -> "Wedsnday 13:27:54"
// to_string12() -> "Wedsnday 01:27:54 PM"
//
// Time < Time           <--- core::ops::PartialOrd
// #[derive(PartialOrd)] <--- Tego nie robimy w tym projekcie
// impl core::ops::PartialOrd for Time {
//      ...
// }
//
// Time == Time     <--- core::ops::PartialEq
// Time + Time      <--- core::ops::PartialAdd
// Time - Time      <--- core::ops::PartialSub
// 
#[derive(Debug)]
pub enum Weekday {Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday}

impl Weekday {
    pub fn next_weekday(&self) {
        use Weekday::*;
        match *self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        };
    }
}

#[derive(Debug)]
pub struct Time {
    pub time: u32,
    pub weekday: Weekday,
}

impl Time {
    pub fn build_time() -> Time {
        Time{time: 0, weekday: Weekday::Monday}
    }
    
    pub fn get_seconds(&self) -> u32 {
        let seconds = self.time.clone();
        seconds
    }
    
    pub fn get_minutes(&self) -> u32 {
        let seconds = self.get_seconds();
        let mut minutes = 0;
        for i in 0..seconds {
            if i == 60 {
                minutes += 1;
            }
        }
        minutes
    }
    
    pub fn get_hours(&self) -> u32 {
        let minutes = self.get_minutes();
        let mut hours = 0;
        for i in 0..minutes {
            if i == 60 {
                hours += 1;
            }
        }
        hours
    }

    pub fn new(/*day: enum,*/ &self, hours: u32, minutes: u32, seconds: u32) -> Time {
        let mut hours = 0;
        let mut minutes = 0;
        let mut seconds = 0;
        let mut i = 0;
        while i < self.time {
            seconds += 1;
            if seconds >= 60 {
                minutes += 1;
                seconds = 0;
            } if minutes >= 60 {
                hours += 1;
                minutes = 0;
            } if hours == 24 {
                hours = 0;
                minutes = 0;
                seconds = 0;
                self.weekday.next_weekday();
            }
        }
        Time {time: time_seconds, weekday:self.weekday}
    }

    pub fn to_string24(&self) -> String {
        let mut hours = 0;
        let mut minutes = 0;
        let mut seconds = 0;
        let mut i = 0;
        while i < self.time {
            seconds += 1;
            if seconds >= 60 {
                minutes += 1;
                seconds = 0;
            } if minutes >= 60 {
                hours += 1;
                minutes = 0;
            } if hours == 24 {
                hours = 0;
                minutes = 0;
                seconds = 0;
                self.weekday.next_weekday();
            }
        }
        format!("{}:{}:{}", hours, minutes, seconds)
    }
    
    pub fn to_string12(&self) -> String {
        let mut hours = 0;
        let mut minutes = 0;
        let mut seconds = 0;
        let mut i = 0;
        while i < self.time {
            seconds += 1;
            if seconds >= 60 {
                minutes += 1;
                seconds = 0;
            } if minutes >= 60 {
                hours += 1;
                minutes = 0;
            } if hours == 12 {
                hours = 0;
                minutes = 0;
                seconds = 0;
            }
        }
        format!("{}:{}:{}", hours, minutes, seconds)
    }
}
