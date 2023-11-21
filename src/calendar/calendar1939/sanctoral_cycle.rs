use super::*;

pub const CALENDAR_OF_SAINTS: [CalendarEntry; 1] = [(
    11,
    1,
    FeastDetails::new("omnium-sanctorum", FeastRank::DoubleFirstClass)
        .make_feriatum()
        .with_vigil(VigilRank::Common)
        .with_octave(OctaveRank::Common),
)];
