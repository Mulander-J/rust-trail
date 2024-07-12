use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

const MINUTES_OF_HOUR: i32 = 60;
const MINUTES_OF_DAY: i32 = 24 * MINUTES_OF_HOUR;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hh, mm) = (self.0.div_euclid(MINUTES_OF_HOUR), self.0.rem_euclid(MINUTES_OF_HOUR));
        write!(f, "{:02}:{:02}", hh, mm)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes:i32) -> Self {
        Clock((hours * MINUTES_OF_HOUR + minutes).rem_euclid(MINUTES_OF_DAY))
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.0 + minutes)
    }
    // pub fn new(hours: i32, minutes: i32) -> Self {
    //     Clock {
    //         hh: (hours + minutes.div_euclid(60)).rem_euclid(24),
    //         mm: minutes.rem_euclid(60),
    //     }
    // }
    // pub fn new(hours: i32, minutes: i32) -> Self {
    //     Clock { hh: 0, mm: 0 }.add_hour(hours).add_minutes(minutes)
    // }

    // pub fn add_minutes(&self, minutes: i32) -> Self {
    //     if minutes == 0 {
    //         Clock { ..*self }
    //     } else {
    //         let sum = self.mm + minutes;
    //         let mod_m = sum % 60;
    //         let mm = if mod_m < 0 { 60 + mod_m } else { mod_m };
    //         let h = sum / 60;
    //         Clock { mm, ..*self }.add_hour(if mod_m < 0 { h - 1 } else { h })
    //     }
    // }

    // pub fn add_hour(&self, h: i32) -> Self {
    //     if h == 0 {
    //         Clock { ..*self }
    //     } else {
    //         let mod_h = (h + self.hh) % 24;
    //         let hh = if mod_h < 0 { 24 + mod_h } else { mod_h };
    //         Clock { hh, ..*self }
    //     }
    // }
}

// impl ToString for Clock {
//     fn to_string(&self) -> String {
//         format!("{}:{}", pad_start(self.hh as u32), pad_start(self.mm as u32))
//     }
// }

// impl PartialEq for Clock {
//     fn eq(&self, other: &Self) -> bool {
//         self.hh == other.hh && self.mm == other.mm
//     }
// }

// fn pad_start(n: u32) -> String {
//     if n > 9 {
//         format!("{n}")
//     } else {
//         format!("0{}", n.to_string())
//     }
// }
