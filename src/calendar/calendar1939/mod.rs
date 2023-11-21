use super::*;
use crate::rubrics::*;

pub mod sanctoral_cycle;
pub mod temporal_cycle;

pub struct Calendar1939;

impl Calendar for Calendar1939 {
    fn temporal_cycle<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>> {
        let ch = CalendarHelper::new(year);
        self.temporal_cycle_h(ch)
    }
    fn calendar_of_saints<'a>(&self) -> &[CalendarEntry<'a>] {
        &sanctoral_cycle::CALENDAR_OF_SAINTS[..]
    }
    fn sanctoral_cycle<'a>(&self, year: i32) -> Vec<Vec<Office<'a>>> {
        let ch = CalendarHelper::new(year);
        self.sanctoral_cycle_h(ch, Rubrics1939)
    }
}
