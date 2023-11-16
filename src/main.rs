use chrono::{Datelike, NaiveDate};

pub mod calendar;
pub mod rubrics;

fn main() {
    let year = 2024;
    let calendar = calendar::generate::temporal_cycle_ids(year, rubrics::Rubrics1910);
    let jan_1 = NaiveDate::from_ymd_opt(year, 1, 1).expect("year out of range");
    let first_sunday = (jan_1.ordinal0() + 7 - jan_1.weekday().number_from_monday()) as usize;
    let sundays: Vec<&String> = calendar.iter().skip(first_sunday).step_by(7).collect();
    for s in sundays {
        println!("{}", s);
    }
}
