use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
enum LiturgicalDate {
    Fixed { month: u32, day: u32 },
    RelativeToEaster(i32),
    RelativeToFirstSundayOfAdvent(i32),
    HardCoded,
}

#[derive(Serialize, Deserialize, Debug)]
enum LiturgicalDay<'a> {
    Sunday {
        name: &'a str,
        rank: Rank,
        date: LiturgicalDate,
    },
    Feast {
        name: &'a str,
        rank: Rank,
        date: LiturgicalDate,
        #[serde(default = "OctaveType::no_octave")]
        octave: OctaveType,
        #[serde(default)]
        of_our_lord: bool,
        #[serde(default)]
        of_our_lady: bool,
        #[serde(default)]
        transferrable: bool, // isn't this just based on rank?
    },
    Vigil {
        name: &'a str,
        rank: Rank,
        date: LiturgicalDate,
    },
    Feria {
        name: &'a str,
        rank: Rank,
        date: LiturgicalDate,
        #[serde(default)]
        privileged: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
enum LiturgicalEvent<'a> {
    Sunday {
        name: &'a str,
        rank: Rank,
        transferred: bool,
        anticipated: bool,
    },
    Feast {
        name: &'a str,
        rank: Rank,
        transferred: bool,
    },
    Vigil {
        name: &'a str,
        rank: Rank,
        anticipated: bool,
    },
    Feria {
        name: &'a str,
        rank: Rank,
        privileged: bool,
    },
    DayInOctave {
        name: &'a str,
        rank: Rank,
    },
    OctaveDay {
        name: &'a str,
        rank: Rank,
    },
    // the office+mass of Our Lady on Saturday when no feast occurs
    OurLadyOnSaturday,
    // includes cases such as when the mass of an impeded Sunday is transferred
    // and the mass of Our Lady on Saturday in Advent
    MassOnly {
        name: &'a str,
    },
    // the "first Friday" type votives
    PrivilegedVotive {
        name: &'a str,
    },
    // a permitted external solemnity
    ExternalSolemnity {
        name: &'a str,
    },
}

#[derive(Serialize, Deserialize, Debug)]
enum Rank {
    Duplex1Cl,
    Duplex2Cl,
    DuplexMaj1Cl, // only Low Sunday, as far as I can tell
    DuplexMaj,
    DuplexMin,
    Semiduplex1Cl, // Sundays in Lent/Passiontide
    Semiduplex2Cl, // Gesima sundays and Sundays in Advent
    Semiduplex,
    Simplex,       // also includes most ferias and vigils
    Commemoration, // for feasts, this means a simplex impeded by another feast
    MatinsCycle,   // for "1st Sunday of October" etc, which coexists with a Sunday after Pentecost
}

#[derive(Serialize, Deserialize, Debug)]
enum OctaveType {
    // ???
    Common,
    Simple,
    NoOctave,
}

impl OctaveType {
    // need this for serde default value
    fn no_octave() -> Self {
        OctaveType::NoOctave
    }
}

fn main() {
    let dom_1_quad = LiturgicalDay::Sunday {
        name: "dom-1-quad",
        rank: Rank::Semiduplex1Cl,
        date: LiturgicalDate::RelativeToEaster(-42),
    };

    let serialized = serde_json::to_string(&dom_1_quad).unwrap();
    println!("serialized = {}", serialized);

    let mut file = File::open("assets/temporal_cycle.json").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let deserialized: Vec<LiturgicalDay> = serde_json::from_str(&s).unwrap();
    println!("deserialized = {:?}", deserialized[0]);
}
