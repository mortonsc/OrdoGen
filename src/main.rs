pub mod calendar;
pub mod ordo;
pub mod rubrics;

fn main() {
    let cb = calendar::CalendarBuilder::new(2023);
    let calendar = calendar::calendar1939::temporal_cycle::temporal_cycle(cb);
    for day in calendar {
        let entry = day
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("{}", entry);
    }
}
