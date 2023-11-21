use super::*;

// TODO: feast of Our Lord and Our Lady
const PURIFICATION: Office =
    Office::feast("in-purificatione-bmv", FeastRank::DoubleSecondClass).done();

const ANNUNCIATION: Office = Office::feast("in-annuntiatione-bmv", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLady)
    .make_feriatum()
    .done();

const CALENDAR_OF_SAINTS: [(u32, u32, FeastDetails); 1] = [(
    11,
    1,
    FeastDetails::new("omnium-sanctorum", FeastRank::DoubleFirstClass)
        .make_feriatum()
        .with_vigil(VigilRank::Common)
        .with_octave(OctaveRank::Common),
)];

pub fn sanctoral_cycle<'a>(cb: CalendarBuilder) -> Vec<Vec<Office<'a>>> {
    let n_days = if is_leap_year(cb.year) { 366 } else { 365 };
    let mut days = vec![Vec::new(); n_days];

    // place the feasts with special rules for their placement
    let purification = cb.ordinal0(2, 2);
    let purification = if cb.is_sunday(purification) && purification >= cb.septuagesima() {
        purification + 1
    } else {
        purification
    };
    days[purification].push(PURIFICATION);

    let annunciation = cb.ordinal0(3, 25);
    let annunciation = if annunciation >= cb.easter - 7 {
        cb.easter + 8
    } else if cb.is_sunday(annunciation) {
        annunciation + 1
    } else {
        annunciation
    };
    days[annunciation].push(ANNUNCIATION);

    let all_souls = cb.ordinal0(11, 2);
    let all_souls = if cb.is_sunday(all_souls) {
        all_souls + 1
    } else {
        all_souls
    };
    days[all_souls].push(Office::AllSouls);

    days
}
