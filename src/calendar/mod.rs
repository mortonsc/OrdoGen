use chrono::{Datelike, NaiveDate, Weekday};
use liturgical::western::easter;
use time::util::is_leap_year;

pub mod calendar1939;

#[cfg(test)]
mod tests;

use crate::ordo::*;
use crate::rubrics::*;

#[derive(Debug, Clone, Copy)]
pub struct CalendarHelper {
    pub year: i32,
    // TODO: maybe these should be exposed as functions
    pub easter: usize,
    pub advent1: usize,
    pub christmas: usize,
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
        let day = if is_leap_year(self.year) && month == 2 && day >= 24 {
            day + 1
        } else {
            day
        };
        NaiveDate::from_ymd_opt(self.year, month, day)
            .expect("invalid date")
            .ordinal0() as usize
    }
    pub fn is_sunday(self, ord: usize) -> bool {
        NaiveDate::from_yo_opt(self.year, (ord + 1) as u32)
            .expect("invalid ordinal date")
            .weekday()
            == Weekday::Sun
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
    pub fn octave_permitted(self, day: usize) -> bool {
        !((day >= self.ash_wednesday() && day <= self.easter - 7)
            || (day >= self.pentecost() - 1 && day <= self.pentecost() + 7)
            || day >= self.ordinal0(12, 17))
    }
}

// TODO: feast of Our Lord and Our Lady
const PURIFICATION: Office =
    Office::feast("in-purificatione-bmv", FeastRank::DoubleSecondClass).done();

const ANNUNCIATION: Office = Office::feast("in-annuntiatione-bmv", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLady)
    .make_feriatum()
    .done();

const CHRIST_THE_KING: Office = Office::feast("dnjc-regis", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .done();

pub type CalendarEntry<'a> = (u32, u32, FeastDetails<'a>);

pub trait Calendar {
    fn temporal_cycle<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>>;
    fn calendar_of_saints<'a>(&self) -> &[CalendarEntry<'a>];
    fn sanctoral_cycle<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>>;
    // returns a vec of the ordo entries for Dec 23 - 31
    fn order_christmastide<'a>(
        &self,
        year: i32,
        lauds_dec23: OrderedOffice<'a>,
    ) -> Vec<OrdoEntry<'a>>;
    fn sanctoral_cycle_h<'a>(
        &self,
        ch: CalendarHelper,
        rs: impl RubricsSystem,
        mut days: Vec<Vec<Office<'a>>>,
    ) -> Vec<Vec<Office<'a>>> {
        let calendar = self.calendar_of_saints();

        // place the feasts with special rules for their placement
        let purification = ch.ordinal0(2, 2);
        let purification = if ch.is_sunday(purification) && purification >= ch.septuagesima() {
            purification + 1
        } else {
            purification
        };
        days[purification].push(PURIFICATION);

        let annunciation = ch.ordinal0(3, 25);
        let annunciation = if annunciation >= ch.easter - 7 {
            ch.easter + 8
        } else if ch.is_sunday(annunciation) {
            annunciation + 1
        } else {
            annunciation
        };
        days[annunciation].push(ANNUNCIATION);

        let oct31_date = NaiveDate::from_ymd_opt(ch.year, 10, 31).unwrap();
        let ctk = (oct31_date.ordinal0() + 1 - oct31_date.weekday().number_from_sunday()) as usize;
        days[ctk].push(CHRIST_THE_KING);

        let all_souls = ch.ordinal0(11, 2);
        let all_souls = if ch.is_sunday(all_souls) {
            all_souls + 1
        } else {
            all_souls
        };
        days[all_souls].push(Office::AllSouls);

        for &(month, day, feast_details) in calendar {
            let ord = ch.ordinal0(month, day);
            let off = Office::Feast(feast_details);
            days[ord].push(off);
            if let Some(vigil) = off.vigil() {
                let vigil_ord = if ch.is_sunday(ord - 1) && rs.anticipate_vigils() {
                    ord - 2
                } else {
                    ord - 1
                };
                days[vigil_ord].push(vigil)
            }
            if let Some(octave_day) = off.octave_day() {
                let octave_day_ord = ord + 7;
                if ch.octave_permitted(octave_day_ord) {
                    days[octave_day_ord].push(octave_day);
                }
            }
            if let Some(day_within_octave) = off.day_within_octave() {
                for day in 1..7 {
                    let day_within_octave_ord = ord + day;
                    if ch.octave_permitted(day_within_octave_ord) {
                        days[day_within_octave_ord].push(day_within_octave);
                    }
                }
            }
        }

        days
    }
}
