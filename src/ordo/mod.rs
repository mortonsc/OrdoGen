use chrono::{Datelike, NaiveDate, Weekday};
use std::collections::VecDeque;

use crate::rubrics::*;

pub struct OrdoEntry<'a> {
    pub lauds: OrderedOffice<'a>,
    pub vespers: OrderedVespers<'a>,
}

pub fn complete_ordo<'a>(
    rubrics_system: impl RubricsSystem,
    year: i32,
    offices: Vec<Vec<Office<'a>>>,
) -> Vec<OrdoEntry<'a>> {
    // TODO: the octave of Christmas has to be special-cased
    let mut ordo: Vec<OrdoEntry<'a>> = Vec::new();
    let mut all_lauds: Vec<OrderedOffice<'a>> = Vec::new();
    let mut to_translate: VecDeque<Office<'a>> = VecDeque::new();
    for day in 0..offices.len() {
        let (lauds, new_to_translate) = rubrics_system.order_office(&offices[day]);
        let lauds = if !to_translate.is_empty()
            && rubrics_system.admits_translated_feast(lauds.office_of_day)
        {
            assert!(new_to_translate.is_empty());
            let mut new_offs_of_day = offices[day].clone();
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
    for day in 0..(offices.len() - 1) {
        let is_sunday = NaiveDate::from_yo_opt(year, day as u32).unwrap().weekday() == Weekday::Sun;
        let vespers = rubrics_system.order_vespers(&all_lauds[day], &all_lauds[day + 1], is_sunday);
        ordo.push(OrdoEntry {
            lauds: all_lauds[day].clone(),
            vespers,
        });
    }
    ordo
}
