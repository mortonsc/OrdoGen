use super::*;
use serde::Deserialize;

const ST_BRIDGET_RON: &str = r#"
        {
            id: "s-birgittae-vid",
            rank: "LesserDouble",
        }
    "#;

const ST_BRIDGET_CE: CalendarEntry = CalendarEntry {
    feast_details: FeastDetails::new("s-birgittae-vid", FeastRank::LesserDouble),
    vigil: None,
    octave: None,
};

#[test]
fn straightforward_feast() {
    let ce: CalendarEntry = ron::from_str(ST_BRIDGET_RON).unwrap();
    assert!(ce == ST_BRIDGET_CE);
}
