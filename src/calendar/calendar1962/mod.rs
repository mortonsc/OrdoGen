use super::*;
use crate::rubrics::*;

pub mod sanctoral_cycle;
pub mod temporal_cycle;

#[derive(Clone, Copy)]
pub struct Calendar1962;

impl Calendar for Calendar1962 {
    fn generate<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>> {
        let mut days = self.sanctoral_cycle(year);
        self.add_temporal_cycle(year, &mut days);
        self.add_vigils(year, &mut days, |rank| rank == VigilRank::FirstClass);
        self.add_octaves(year, &mut days, |rank| rank == OctaveRank::SecondOrder);
        self.translate_feasts(&mut days);
        self.add_vigils(year, &mut days, |rank| rank < VigilRank::FirstClass);
        days
    }
    fn add_temporal_cycle(&self, year: i32, days: &mut [Vec<Office>]) {
        let ch = CalendarHelper::new(year);
        self.add_temporal_cycle_h(ch, days);
    }
    fn calendar_of_saints<'a>(&self, year: i32) -> Vec<CalendarEntry<'a>> {
        let mut calendar: Vec<CalendarEntry<'a>> = sanctoral_cycle::CALENDAR_OF_SAINTS
            .iter()
            .map(|&(month, day, feast_details)| {
                (month, day, feast_details.with_proper_date(month, day))
            })
            .collect();
        let moveable = self.moveable_feasts(year);
        calendar.extend_from_slice(&moveable[..]);
        calendar
    }
    fn translate_feasts(&self, days: &mut [Vec<Office>]) {
        self.translate_feasts_h(Rubrics1962, days);
    }
    fn transfer_anticipated_sundays(&self, _days: &mut [Vec<Office>]) {}
    fn add_vigils(&self, year: i32, days: &mut [Vec<Office>], include: impl Fn(VigilRank) -> bool) {
        let ch = CalendarHelper::new(year);
        let mut to_add: Vec<(usize, Office)> = Vec::new();
        for (day, offs) in days.iter().enumerate() {
            let (lauds, _) = Rubrics1962.order_lauds(offs);
            let Some(
                vigil @ Office::Vigil {
                    feast_details,
                    rank,
                },
            ) = lauds.office_of_day.vigil()
            else {
                continue;
            };
            if !include(rank) {
                continue;
            }
            let vigil_day = day - 1;
            if let Some((m, d)) = feast_details.proper_date {
                // Vigils of translated feasts are omitted
                if vigil_day != ch.ordinal0_of_feast_date(m, d) - 1 {
                    continue;
                }
            }
            to_add.push((vigil_day, vigil));
        }
        for (day, off) in to_add {
            days[day].push(off);
        }
    }
}
