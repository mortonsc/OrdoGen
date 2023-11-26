use chrono::{Datelike, NaiveDate, Weekday};
use liturgical::western::easter;
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
        let Some(date) = NaiveDate::from_ymd_opt(self.year, month, day) else {
            panic!("invalid date: ({}, {})", month, day);
        };
        date.ordinal0() as usize
    }
    // same as ordinal0 except it accounts for the fact that feasts falling between Feb 24 and 28
    // are transferred forward a day in leap years
    pub fn ordinal0_of_feast_date(self, month: u32, day: u32) -> usize {
        if (month, day) == (2, 29) {
            panic!("2/29 is not a valid date for a feast");
        }
        let day = if month == 2 && day >= 24 && is_leap_year(self.year) {
            day + 1
        } else {
            day
        };
        self.ordinal0(month, day) as usize
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
    fn generate<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>>;
    fn calendar_of_saints<'a>(&self, year: i32) -> Vec<CalendarEntry<'a>>;
    // generates a vec of vec of offices for each day from self.calendar_of_saints,
    // and adds in All Souls Day
    fn sanctoral_cycle<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>> {
        let ch = CalendarHelper::new(year);
        let calendar = self.calendar_of_saints(year);
        let mut days: Vec<Vec<Office<'a>>> = vec![Vec::new(); ch.n_days()];
        for (month, day, feast_details) in calendar {
            days[ch.ordinal0_of_feast_date(month, day)].push(Office::Feast(feast_details));
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
    // takes the list of Vecs of offices for each day generated by sanctoral_cycle
    // and adds in the temporal cycle
    // Defined as all the Sundays of the year (including the Sunday in the octave of Christmas)
    // all the ferias of the year,
    // and all feasts whose date is fixed relative to Easter.
    // Christmas, its octave, and the feasts that follow it are considered part of the sanctoral
    // cycle here.
    fn add_temporal_cycle(&self, year: i32, days: &mut [Vec<Office>]);
    // takes a list of Vecs of offices for each day
    // and changes it by transferring feasts
    fn translate_feasts(&self, days: &mut [Vec<Office>]);
    fn translate_feasts_h(&self, rubrics_system: impl RubricsSystem, days: &mut [Vec<Office>]) {
        let mut to_translate: Vec<Office> = Vec::new();
        for day in 0..days.len() {
            let (lauds, new_to_translate) = rubrics_system.order_lauds(&days[day][..]);
            if !to_translate.is_empty()
                && rubrics_system.admits_translated_feast(lauds.office_of_day)
            {
                assert!(new_to_translate.is_empty());
                // if multiple feasts are being translated, the highest ranked gets first dibs on
                // the first open day
                to_translate.sort_by(|&o1, &o2| rubrics_system.compare_precedence_occ(o1, o2));
                days[day].push(to_translate.pop().unwrap());
            } else if !new_to_translate.is_empty() {
                days[day] = days[day]
                    .clone()
                    .into_iter()
                    .filter(|o| !new_to_translate.contains(o))
                    .collect();
                to_translate.extend_from_slice(&new_to_translate[..]);
            }
        }
    }
    // Adds the vigils of a rank for which include(rank) is true
    fn add_vigils(&self, year: i32, days: &mut [Vec<Office>], include: impl Fn(VigilRank) -> bool);
    // Adds the octaves for feasts with an octave rank where include(rank) is true
    fn add_octaves(
        &self,
        year: i32,
        days: &mut [Vec<Office>],
        include: impl Fn(OctaveRank) -> bool,
    ) {
        let ch = CalendarHelper::new(year);
        let mut to_add: Vec<(usize, Office)> = Vec::new();
        for (day, offs) in days.iter().enumerate() {
            for &off in offs {
                let Office::Feast(FeastDetails {
                    octave: Some(rank),
                    proper_date,
                    ..
                }) = off
                else {
                    continue;
                };
                if !include(rank) {
                    continue;
                }
                if !ch.octave_permitted(day) {
                    // assumption: if a local feast with an octave falls towards the end of advent
                    // its octave is omitted entirely, even if the last days of it would fall after
                    // Christmas, when octaves are permitted again
                    continue;
                }
                let octave_day_ord = if let Some((m, d)) = proper_date {
                    ch.ordinal0_of_feast_date(m, d) + 7
                } else {
                    day + 7
                };
                if octave_day_ord <= day {
                    continue;
                }
                if let Some(octave_day) = off.octave_day() {
                    let octave_day_ord = octave_day_ord % ch.n_days();
                    if ch.octave_permitted(octave_day_ord) {
                        to_add.push((octave_day_ord, octave_day));
                    }
                }
                if let Some(day_within_octave) = off.day_within_octave() {
                    for dwo_ord in (day + 1)..octave_day_ord {
                        let dwo_ord = dwo_ord % ch.n_days();
                        if ch.octave_permitted(dwo_ord) {
                            to_add.push((dwo_ord, day_within_octave));
                        }
                    }
                }
            }
        }
        for (day, off) in to_add {
            days[day].push(off);
        }
    }
    // add_temporal_cycle places anticipated Sundays on Saturdays
    // this function moves them to the available free day in the preceding week, if there is one
    fn transfer_anticipated_sundays(&self, days: &mut [Vec<Office>]);
    fn transfer_anticipated_sundays_h(
        &self,
        rubrics_system: impl RubricsSystem,
        days: &mut [Vec<Office>],
    ) {
        // rough bounds on where an anticipated Sunday can occur
        for day in 10..335 {
            let Some(idx) = days[day].iter().position(|o| o.is_anticipated_sunday()) else {
                continue;
            };
            let antic_sunday = days[day].swap_remove(idx);
            let mut new_day = day;
            for offset in 0..6 {
                if days[day - offset]
                    .iter()
                    .all(|&o| rubrics_system.admits_anticipated_sunday(o))
                {
                    new_day = day - offset;
                    break;
                }
            }
            days[new_day].push(antic_sunday);
        }
    }
}
