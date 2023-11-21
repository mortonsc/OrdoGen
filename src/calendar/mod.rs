use chrono::{Datelike, NaiveDate, Weekday};
use liturgical::western::easter;
use time::util::is_leap_year;

pub mod calendar1939;

#[cfg(test)]
mod tests;

use crate::rubrics::*;

#[derive(Debug, Clone, Copy)]
pub struct CalendarBuilder {
    year: i32,
    easter: usize,
    advent1: usize,
    christmas: usize,
}

impl CalendarBuilder {
    pub fn new(year: i32) -> Self {
        let easter = easter::date(year).unwrap().ordinal0() as usize;
        let christmas_date = NaiveDate::from_ymd_opt(year, 12, 25).unwrap();
        let christmas = christmas_date.ordinal0() as usize;
        let advent1 = christmas - (christmas_date.weekday().number_from_monday() as usize) - 21;
        Self {
            year,
            easter,
            advent1,
            christmas,
        }
    }
    fn ordinal0(self, month: u32, day: u32) -> usize {
        let day = if is_leap_year(self.year) && month == 2 && day >= 24 {
            day + 1
        } else {
            day
        };
        NaiveDate::from_ymd_opt(self.year, month, day)
            .expect("invalid date")
            .ordinal0() as usize
    }
    fn is_sunday(self, ord: usize) -> bool {
        NaiveDate::from_yo_opt(self.year, ord as u32)
            .expect("invalid ordinal date")
            .weekday()
            == Weekday::Sun
    }
    fn septuagesima(self) -> usize {
        self.easter - 63
    }
    fn lent1(self) -> usize {
        self.easter - 42
    }
    fn pentecost(self) -> usize {
        self.easter + 49
    }
    fn octave_permitted(self, day: usize) -> bool {
        if day >= self.lent1() - 4 && day <= self.easter + 7 {
            false
        } else if day >= self.pentecost() && day <= self.pentecost() + 7 {
            false
        } else if day >= self.ordinal0(12, 17) {
            false
        } else {
            true
        }
    }
}
