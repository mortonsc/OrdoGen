use chrono::NaiveDate;

use crate::calendar::Calendar;
use crate::ordo::Ordo;
use crate::rubrics::Rubrics1939;

pub mod calendar;
pub mod ordo;
pub mod rubrics;

fn print_temporal_cycle(year: i32) {
    let calendar = calendar::calendar1939::Calendar1939.temporal_cycle(year);
    for day in 0..calendar.len() {
        let entry = calendar[day]
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let date = NaiveDate::from_yo_opt(year, (day + 1) as u32).unwrap();
        println!("{}: {}", date.format("%m/%d"), entry);
    }
}

fn print_sanctoral_cycle(year: i32) {
    let calendar = calendar::calendar1939::Calendar1939.sanctoral_cycle(year);
    for day in 0..calendar.len() {
        let entry = calendar[day]
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let date = NaiveDate::from_yo_opt(year, (day + 1) as u32).unwrap();
        println!("{}: {}", date.format("%m/%d"), entry);
    }
}

fn print_ordo(year: i32) {
    let ordo = Ordo::new(calendar::calendar1939::Calendar1939, Rubrics1939, year);
    for day in 0..ordo.entries.len() {
        let date = NaiveDate::from_yo_opt(year, (day + 1) as u32).unwrap();
        let entry = &ordo.entries[day];
        println!("{} | Ad Laudes:   {}", date.format("%m/%d"), entry.lauds);
        println!("      | Ad Vesperas: {}", entry.vespers);
    }
}

fn main() {
    env_logger::init();
    print_ordo(2022);
}
