use super::*;
use crate::rubrics::*;

pub mod sanctoral_cycle;
pub mod temporal_cycle;

#[derive(Clone, Copy)]
pub struct Calendar1939;

impl Calendar for Calendar1939 {
    fn add_temporal_cycle(&self, year: i32, days: &mut [Vec<Office<'_>>]) {
        let ch = CalendarHelper::new(year);
        self.add_temporal_cycle_h(ch, days);
    }
    fn calendar_of_saints<'a>(&self, year: i32) -> Vec<CalendarEntry<'a>> {
        let mut calendar = sanctoral_cycle::CALENDAR_OF_SAINTS[..].to_vec();
        let moveable = self.moveable_feasts(year);
        calendar.extend_from_slice(&moveable[..]);
        calendar
    }
    fn sanctoral_cycle<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>> {
        self.sanctoral_cycle_h(year, Rubrics1939)
    }
}
