use liturgical::western::easter;
use time::util::is_leap_year;

use super::*;
use crate::rubrics::*;

mod temporal_cycle;

struct Calendar1939;

impl<'a> Calendar1939 {
    fn matins_id(&self, year: i32, day: usize) -> Option<&'a str> {
        // TODO
        None
    }
    fn raw_temporal_cycle(&self, year: i32) -> Vec<Vec<Office<'a>>> {
        let n_days = if is_leap_year(year) { 366 } else { 365 };
        let mut result = vec![Vec::new(); n_days];
        return result;
    }
}

// impl<'a> Calendar<'a> for Calendar1939 {
//     type RS = Rubrics1910;
// }
