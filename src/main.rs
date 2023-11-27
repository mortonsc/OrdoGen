use chrono::NaiveDate;

use crate::calendar::calendar1939::Calendar1939;
use crate::calendar::calendar1962::Calendar1962;
use crate::calendar::{Calendar, CalendarHelper};
use crate::ordo::Ordo;
use crate::rubrics::{Office, Rubrics1939, Rubrics1962, RubricsSystem};

pub mod calendar;
pub mod ordo;
pub mod rubrics;

#[allow(dead_code)]
fn print_temporal_cycle(year: i32, calendar: impl Calendar) {
    let ch = CalendarHelper::new(year);
    let mut days: Vec<Vec<Office<'_>>> = vec![Vec::new(); ch.n_days()];
    calendar.add_temporal_cycle(year, &mut days);
    for (day, entry) in days.iter().enumerate() {
        let entry_s = entry
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let date = NaiveDate::from_yo_opt(year, (day + 1) as u32).unwrap();
        println!("{}: {}", date.format("%m/%d"), entry_s);
    }
}

#[allow(dead_code)]
fn print_sanctoral_cycle(year: i32, calendar: impl Calendar) {
    let cal = calendar.sanctoral_cycle(year);
    for (day, entry) in cal.iter().enumerate() {
        let entry_s = entry
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let date = NaiveDate::from_yo_opt(year, (day + 1) as u32).unwrap();
        println!("{}: {}", date.format("%m/%d"), entry_s);
    }
}

#[allow(dead_code)]
fn print_ordo(year: i32, calendar: impl Calendar, rubrics: impl RubricsSystem) {
    let ordo = Ordo::new(calendar, rubrics, year);
    for day in 0..ordo.entries.len() {
        let date = NaiveDate::from_yo_opt(year, (day + 1) as u32).unwrap();
        let entry = &ordo.entries[day];
        println!("{} | Ad Laudes:   {}", date.format("%m/%d"), entry.lauds);
        println!("      | Ad Vesperas: {}", entry.vespers);
        println!("");
    }
}

fn main() {
    env_logger::init();
    // print_ordo(2023, calendar::calendar1939::Calendar1939, Rubrics1939);
    print_temporal_cycle(2023, Calendar1962);
}
