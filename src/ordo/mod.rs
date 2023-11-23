use chrono::{Datelike, NaiveDate};
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

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
        let mut entries: Vec<OrdoEntry<'a>> = Vec::new();
        let mut all_lauds: Vec<OrderedLauds<'a>> = Vec::new();
        let mut to_translate: VecDeque<Office<'a>> = VecDeque::new();
        for day in 0..days.len() {
            let mut offices = days[day].clone();
            // remove an anticipated Sunday to deal with it later
            let antic_sunday_idx = offices.iter().position(|o| {
                matches!(
                    o,
                    Office::Feria {
                        rank: FeriaRank::AnticipatedSunday,
                        ..
                    }
                )
            });
            let antic_sunday = if let Some(idx) = antic_sunday_idx {
                debug!("Found anticipated Sunday");
                Some(offices.swap_remove(idx))
            } else {
                None
            };
            // if we're translating a feast with an octave, we delete days from its octave as we go
            let offices: Vec<Office<'a>> = offices
                .into_iter()
                .filter(|&o| !to_translate.iter().any(|t| t.is_of_same_feast(o)))
                .collect();
            let (lauds, new_to_translate) = rubrics_system.order_lauds(&offices[..]);
            let lauds = if !to_translate.is_empty()
                && rubrics_system.admits_translated_feast(lauds.office_of_day)
            {
                assert!(new_to_translate.is_empty());
                let mut new_offs_of_day = offices.clone();
                new_offs_of_day.push(to_translate.pop_front().unwrap());
                let (new_lauds, no_translations) = rubrics_system.order_lauds(&new_offs_of_day);
                assert!(no_translations.is_empty());
                new_lauds
            } else {
                lauds
            };
            for t in new_to_translate {
                to_translate.push_back(t);
            }
            all_lauds.push(lauds);
            if let Some(antic_sunday) = antic_sunday {
                let mut antic_day = day;
                for offset in 0..6 {
                    if rubrics_system
                        .admits_anticipated_sunday(all_lauds[day - offset].office_of_day)
                    {
                        antic_day = day - offset;
                        break;
                    }
                }
                // now we re-generate the office for the day with the anticipated Sunday
                let mut offices = days[antic_day].clone();
                offices.push(antic_sunday);
                let (lauds, no_translations) = rubrics_system.order_lauds(&offices[..]);
                assert!(no_translations.is_empty());
                all_lauds[antic_day] = lauds
            }
        }
        for day in 0..(days.len() - 1) {
            let vespers = rubrics_system.order_vespers(
                &all_lauds[day],
                &all_lauds[day + 1],
                ch.is_sunday(day + 1),
            );
            entries.push(OrdoEntry {
                lauds: all_lauds[day].clone(),
                vespers,
            });
        }
        let dec31 = ch.ordinal0(12, 31);
        // lauds of Jan 1 is the same every year
        let vespers_dec31 =
            rubrics_system.order_vespers(&all_lauds[dec31], &all_lauds[0], ch.is_sunday(dec31));
        entries.push(OrdoEntry {
            lauds: all_lauds[dec31].clone(),
            vespers: vespers_dec31,
        });

        Self { year, entries }
    }
    pub fn for_day(&self, month: u32, day: u32) -> &OrdoEntry<'a> {
        let idx = NaiveDate::from_ymd_opt(self.year, month, day)
            .expect("invalid date")
            .ordinal0() as usize;
        &self.entries[idx]
    }
}
