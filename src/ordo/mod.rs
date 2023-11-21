use chrono::{Datelike, NaiveDate};
use std::collections::VecDeque;

use crate::calendar::{Calendar, CalendarHelper};
use crate::rubrics::*;

pub struct OrdoEntry<'a> {
    pub lauds: OrderedOffice<'a>,
    pub vespers: OrderedVespers<'a>,
}

pub struct Ordo<'a> {
    pub year: i32,
    pub entries: Vec<OrdoEntry<'a>>,
}

impl<'a> Ordo<'a> {
    pub fn new(calendar: impl Calendar, rubrics_system: impl RubricsSystem, year: i32) -> Self {
        let ch = CalendarHelper::new(year);
        let temporal = calendar.temporal_cycle(year);
        let sanctoral = calendar.sanctoral_cycle(year);
        assert!(temporal.len() == sanctoral.len());
        // TODO: the octave of Christmas has to be special-cased
        let mut entries: Vec<OrdoEntry<'a>> = Vec::new();
        let mut all_lauds: Vec<OrderedOffice<'a>> = Vec::new();
        let mut to_translate: VecDeque<Office<'a>> = VecDeque::new();
        for day in 0..temporal.len() {
            // TODO: anticipated Sundays
            let mut offices = temporal[day].clone();
            offices.extend_from_slice(&(sanctoral[day])[..]);
            // if we're translating a feast with an octave, we delete days from its octave as we go
            let offices: Vec<Office<'a>> = offices
                .into_iter()
                .filter(|&o| !to_translate.iter().any(|t| t.is_of_same_feast(o)))
                .collect();
            let (lauds, new_to_translate) = rubrics_system.order_office(&offices[..]);
            let lauds = if !to_translate.is_empty()
                && rubrics_system.admits_translated_feast(lauds.office_of_day)
            {
                assert!(new_to_translate.is_empty());
                let mut new_offs_of_day = offices.clone();
                new_offs_of_day.push(to_translate.pop_front().unwrap());
                let (new_lauds, no_translations) = rubrics_system.order_office(&new_offs_of_day);
                assert!(no_translations.is_empty());
                new_lauds
            } else {
                lauds
            };
            for t in new_to_translate {
                to_translate.push_back(t);
            }
            all_lauds.push(lauds);
        }
        for day in 0..(temporal.len() - 1) {
            let vespers = rubrics_system.order_vespers(
                &all_lauds[day],
                &all_lauds[day + 1],
                ch.is_sunday(day),
            );
            entries.push(OrdoEntry {
                lauds: all_lauds[day].clone(),
                vespers,
            });
        }

        Self { year, entries }
    }
    pub fn for_day(&self, month: u32, day: u32) -> &OrdoEntry<'a> {
        let idx = NaiveDate::from_ymd_opt(self.year, month, day)
            .expect("invalid date")
            .ordinal0() as usize;
        &self.entries[idx]
    }
}
