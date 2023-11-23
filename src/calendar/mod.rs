use chrono::{Datelike, NaiveDate, Weekday};
use liturgical::western::easter;
use log::error;
use time::util::is_leap_year;

pub mod calendar1939;

#[cfg(test)]
mod tests;

use crate::rubrics::*;

#[derive(Debug, Clone, Copy)]
pub struct CalendarHelper {
    pub year: i32,
    easter: usize,
    advent1: usize,
    christmas: usize,
}

impl CalendarHelper {
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
    pub fn n_days(self) -> usize {
        if is_leap_year(self.year) {
            366
        } else {
            365
        }
    }
    pub fn ordinal0(self, month: u32, day: u32) -> usize {
        let date = NaiveDate::from_ymd_opt(self.year, month, day);
        if date.is_none() {
            error!("invalid date: ({}, {})", month, day);
        }
        NaiveDate::from_ymd_opt(self.year, month, day)
            .expect("invalid date")
            .ordinal0() as usize
    }
    pub fn month_day(self, ord: usize) -> (u32, u32) {
        let date = NaiveDate::from_yo_opt(self.year, (ord + 1) as u32).unwrap();
        (date.month(), date.day())
    }
    pub fn is_sunday(self, ord: usize) -> bool {
        NaiveDate::from_yo_opt(self.year, (ord + 1) as u32)
            .expect("invalid ordinal date")
            .weekday()
            == Weekday::Sun
    }
    pub fn easter(self) -> usize {
        self.easter
    }
    pub fn christmas(self) -> usize {
        self.christmas
    }
    pub fn epiphany(self) -> usize {
        NaiveDate::from_ymd_opt(self.year, 1, 6).unwrap().ordinal0() as usize
    }
    pub fn advent1(self) -> usize {
        self.advent1
    }
    pub fn septuagesima(self) -> usize {
        self.easter - 63
    }
    pub fn lent1(self) -> usize {
        self.easter - 42
    }
    pub fn ash_wednesday(self) -> usize {
        self.easter - 46
    }
    pub fn pentecost(self) -> usize {
        self.easter + 49
    }
    pub fn sunday_after(self, ord: usize) -> Option<usize> {
        let date = NaiveDate::from_yo_opt(self.year, (ord + 1) as u32)?;
        let result = ord + 8 - (date.weekday().number_from_sunday() as usize);
        if result < self.n_days() {
            Some(result)
        } else {
            None
        }
    }
    // note that this doesn't apply to the octaves of Easter and Pentecost themselves
    pub fn octave_permitted(self, day: usize) -> bool {
        !((day >= self.ash_wednesday() && day <= self.easter + 7)
            || (day >= self.pentecost() - 1 && day <= self.pentecost() + 7)
            || day >= self.ordinal0(12, 17) && day < self.christmas())
    }
}

pub type CalendarEntry<'a> = (u32, u32, FeastDetails<'a>);

pub trait Calendar {
    fn add_temporal_cycle(&self, year: i32, days: &mut [Vec<Office<'_>>]);
    fn calendar_of_saints<'a>(&self, year: i32) -> Vec<CalendarEntry<'a>>;
    fn sanctoral_cycle<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>>;
    fn generate<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>> {
        let mut days = self.sanctoral_cycle(year);
        self.add_temporal_cycle(year, &mut days);
        days
    }
    // generates a vec of vec of offices for each day from self.calendar_of_saints,
    // filling in octaves and vigils, as well as All Souls day
    fn sanctoral_cycle_h<'a>(&self, year: i32, rs: impl RubricsSystem) -> Vec<Vec<Office<'a>>> {
        let ch = CalendarHelper::new(year);
        let calendar = self.calendar_of_saints(year);
        let mut days: Vec<Vec<Office<'a>>> = vec![Vec::new(); ch.n_days()];
        for (month, day, feast_details) in calendar {
            let day = if month == 2 && day >= 24 && is_leap_year(ch.year) {
                day + 1
            } else {
                day
            };
            let ord = ch.ordinal0(month, day);
            let off = Office::Feast(feast_details);
            days[ord].push(off);
            if let Some(Office::Vigil { rank, .. }) = off.vigil() {
                // no vigils in the first couple days of the year, so we can subtract without
                // modding
                assert!(ord >= 2);
                let vigil_ord = if ch.is_sunday(ord - 1) && rs.anticipate_vigil(rank) {
                    ord - 2
                } else {
                    ord - 1
                };
                days[vigil_ord].push(off.vigil().unwrap())
            }
            if let Some(octave_day) = off.octave_day() {
                let octave_day_ord = (ord + 7) % ch.n_days();
                if ch.octave_permitted(octave_day_ord) {
                    days[octave_day_ord].push(octave_day);
                }
            }
            if let Some(day_within_octave) = off.day_within_octave() {
                for day in 1..7 {
                    let day_within_octave_ord = (ord + day) % ch.n_days();
                    if ch.octave_permitted(day_within_octave_ord) {
                        days[day_within_octave_ord].push(day_within_octave);
                    }
                }
            }
        }

        let all_souls = ch.ordinal0(11, 2);
        let all_souls = if ch.is_sunday(all_souls) {
            all_souls + 1
        } else {
            all_souls
        };
        days[all_souls].push(Office::AllSouls);

        days
    }
}
