use serde::Deserialize;
use time::util::is_leap_year;
use time::{Date, Month};

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

pub struct Calendar<'a> {
    year: i32,
    days: Vec<LiturgicalDay<'a>>,
}

impl<'a> Calendar<'a> {
    pub fn new<R: RubricsSystem>(year: i32, rubrics_system: R) -> Self {
        let n_days = if is_leap_year(year) { 366 } else { 365 };
        let days = vec![PLACEHOLDER; n_days];
        // TODO
        Calendar { year, days }
    }
    pub fn get_day(&self, month: Month, day: u8) -> Option<LiturgicalDay> {
        let ordinal = Date::from_calendar_date(self.year, month, day)
            .ok()?
            .ordinal();
        Some(self.days[ordinal as usize].clone())
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct OctaveDetails<'a> {
    pub id: &'a str,
    pub octave_type: OctaveType,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CalendarEntry<'a> {
    #[serde(flatten)]
    pub feast_details: FeastDetails<'a>,
    #[serde(default)]
    pub vigil: Option<&'a str>,
    #[serde(default)]
    pub octave: Option<OctaveDetails<'a>>,
}
