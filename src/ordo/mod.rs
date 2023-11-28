use chrono::{Datelike, NaiveDate};
use log::error;
use serde::{Deserialize, Serialize};
use std::iter::zip;

use crate::calendar::{Calendar, CalendarHelper};
use crate::rubrics::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct OrdoEntry<'a> {
    #[serde(borrow)]
    pub lauds: OrderedLauds<'a>,
    pub vespers: OrderedVespers<'a>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Ordo<'a> {
    pub year: i32,
    #[serde(borrow)]
    pub entries: Vec<OrdoEntry<'a>>,
}

impl<'a> Ordo<'a> {
    pub fn new(calendar: impl Calendar, rubrics_system: impl RubricsSystem, year: i32) -> Self {
        let ch = CalendarHelper::new(year);
        let days = calendar.generate(year);
        let mut all_lauds: Vec<OrderedLauds<'a>> = Vec::new();
        let mut all_vespers: Vec<OrderedVespers<'a>> = Vec::new();
        for (day, offs) in days.iter().enumerate() {
            let (lauds, to_translate) = rubrics_system.order_lauds(&offs[..]);
            // feast translation was handled earlier as part of generating the calendar
            if !to_translate.is_empty() {
                let fd = to_translate[0].feast_details().unwrap();
                let (m, d) = ch.month_day(day);
                error!("Unexpected translated feast: {} on {}/{}", fd.id, m, d);
            }
            all_lauds.push(lauds);
        }
        for day in 0..(days.len() - 1) {
            all_vespers.push(rubrics_system.order_vespers(
                &all_lauds[day],
                &all_lauds[day + 1],
                ch.is_sunday(day + 1),
            ));
        }
        let dec31 = ch.ordinal0(12, 31);
        // OrderedLauds of Jan 1 is the same every year
        all_vespers.push(rubrics_system.order_vespers(
            &all_lauds[dec31],
            &all_lauds[0],
            ch.is_sunday(dec31 - 6),
        ));

        let entries: Vec<OrdoEntry<'a>> = zip(all_lauds, all_vespers)
            .map(|(lauds, vespers)| OrdoEntry { lauds, vespers })
            .collect();

        Self { year, entries }
    }
    pub fn for_day(&self, month: u32, day: u32) -> &OrdoEntry<'a> {
        let Some(date) = NaiveDate::from_ymd_opt(self.year, month, day) else {
            panic!("invalid date: {}-{}-{}", self.year, month, day);
        };
        &self.entries[date.ordinal0() as usize]
    }
}
