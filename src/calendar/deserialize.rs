use serde::Deserialize;

use super::*;
use crate::rubrics::*;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct OctaveDetails<'a> {
    pub id: &'a str,
    pub octave_type: OctaveType,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct CalendarEntry<'a> {
    #[serde(flatten)]
    pub feast_details: FeastDetails<'a>,
    #[serde(default)]
    pub vigil: Option<&'a str>,
    #[serde(default)]
    pub octave: Option<OctaveDetails<'a>>,
}
