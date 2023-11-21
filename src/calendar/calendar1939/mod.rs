use super::*;
use crate::ordo::*;
use crate::rubrics::*;

pub mod christmastide;
pub mod sanctoral_cycle;
pub mod temporal_cycle;

#[derive(Clone, Copy)]
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
        let mut days: Vec<Vec<Office<'a>>> = vec![Vec::new(); ch.n_days()];
        self.add_moveable_feasts(ch, &mut days);
        self.sanctoral_cycle_h(ch, Rubrics1939, days)
    }
    fn order_christmastide<'a>(
        &self,
        year: i32,
        lauds_dec23: OrderedOffice<'a>,
    ) -> Vec<OrdoEntry<'a>> {
        self.order_christmastide_h(year, lauds_dec23)
    }
}
