use chrono::{Datelike, NaiveDate};
use time::util::is_leap_year;

pub mod calendar1939;
pub mod generate;

#[cfg(test)]
mod tests;

use crate::rubrics::*;

#[derive(Debug, Clone)]
pub struct LiturgicalDay<'a> {
    pub office: Office<'a>,
    pub comms_at_lauds: Vec<Office<'a>>,
    pub vespers: Vespers<'a>,
    pub comms_at_vespers: Vec<Office<'a>>,
}

const PLACEHOLDER: LiturgicalDay = LiturgicalDay {
    office: Office::Empty,
    comms_at_lauds: Vec::new(),
    vespers: Vespers::SecondVespers(Office::Empty),
    comms_at_vespers: Vec::new(),
};

pub struct Ordo<'a> {
    year: i32,
    days: Vec<LiturgicalDay<'a>>,
}

impl<'a> Ordo<'a> {
    pub fn new<R: RubricsSystem>(year: i32, _rubrics_system: R) -> Self {
        let n_days = if is_leap_year(year) { 366 } else { 365 };
        let days = vec![PLACEHOLDER; n_days];
        // TODO
        Ordo { year, days }
    }
    pub fn get_day(&self, month: u32, day: u32) -> Option<LiturgicalDay> {
        let ordinal = NaiveDate::from_ymd_opt(self.year, month, day)?.ordinal0() as usize;
        Some(self.days[ordinal].clone())
    }
}
pub trait Calendar<'a> {
    type RS: RubricsSystem;
    fn raw_ordo(&self, year: i32) -> Vec<Vec<Office<'a>>>;
}
