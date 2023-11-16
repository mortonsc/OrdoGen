use chrono::{Datelike, NaiveDate};
use liturgical::western::easter;
use time::util::is_leap_year;

use super::{Calendar, LiturgicalDay};
use crate::rubrics::RubricsSystem;

pub fn temporal_cycle_ids<'a, R: RubricsSystem>(year: i32, _rubrics_system: R) -> Vec<String> {
    let n_days = if is_leap_year(year) { 366 } else { 365 };
    let mut days = vec!["___".to_string(); n_days];
    let jan_1 = NaiveDate::from_ymd_opt(year, 1, 1).expect("year out of range");

    // Sundays first
    let first_sunday = (jan_1.ordinal0() + 7 - jan_1.weekday().number_from_monday()) as usize;
    for i in 1..days.len() {
        if i % 7 == first_sunday % 7 {
            days[i] = "dom_vacat".to_string();
        }
    }

    let epiphany_date = NaiveDate::from_ymd_opt(year, 1, 6).expect("year out of range");
    let epiphany = epiphany_date.ordinal0() as usize;
    let dom_post_epiph = epiphany + 7 - (epiphany_date.weekday().number_from_monday() as usize);
    for i in 0..6 {
        let id = format!("dom-{}-post-epiph", i + 1);
        days[dom_post_epiph + (7 * i)] = id;
    }

    let easter = easter::date(year).expect("year out of range").ordinal0() as usize;
    days[easter] = "dom-resurrectionis".to_string();
    for i in 1..7 {
        let id = format!("dom-{}-post-pascha", i);
        days[easter + (7 * i)] = id;
    }
    let pentecost = easter + 49;
    days[pentecost] = "dom-pentecostes".to_string();
    for i in 1..24 {
        let id = format!("dom-{}-post-pent", i);
        days[pentecost + (7 * i)] = id;
    }
    let septuagesima = easter - 63;
    days[septuagesima] = "dom-septuagesima".to_string();
    days[septuagesima + 7] = "dom-sexagesima".to_string();
    days[septuagesima + 14] = "dom-quinqagesima".to_string();
    for i in 1..5 {
        let id = format!("dom-{}-quad", i);
        days[septuagesima + 14 + (7 * i)] = id;
    }
    days[easter - 14] = "dom-passionis".to_string();
    days[easter - 7] = "dom-in-palmis".to_string();

    let christmas_date = NaiveDate::from_ymd_opt(year, 12, 25).expect("year out of range");
    let christmas = christmas_date.ordinal0() as usize;
    let dom_24_post_pent =
        christmas - (christmas_date.weekday().number_from_monday() as usize) - 28;
    days[dom_24_post_pent] = "dom-24-post-pent".to_string();
    for i in 1..5 {
        let id = format!("dom-{}-advent", i);
        days[dom_24_post_pent + (7 * i)] = id;
    }

    let dom_23_post_pent = pentecost + (7 * 23);
    let extra_sundays = ((dom_24_post_pent - dom_23_post_pent) / 7) - 1;
    for i in 1..(extra_sundays + 1) {
        let id = format!("dom-{}-quae-superfuit-post-epiph", 7 - i);
        days[dom_24_post_pent - (7 * i)] = id;
    }

    days
}
