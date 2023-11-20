use chrono::{Datelike, NaiveDate};
use liturgical::western::easter;
use time::util::is_leap_year;

use crate::rubrics::*;

const SUNDAYS_OF_ADVENT: [Office; 4] = [
    Office::Sunday {
        id: "dom-1-advent",
        matins_id: None,
        rank: SundayRank::FirstClass,
    },
    Office::Sunday {
        id: "dom-2-advent",
        matins_id: None,
        rank: SundayRank::SecondClass,
    },
    Office::Sunday {
        id: "dom-3-advent",
        matins_id: None,
        rank: SundayRank::SecondClass,
    },
    Office::Sunday {
        id: "dom-4-advent",
        matins_id: None,
        rank: SundayRank::SecondClass,
    },
];

const SUNDAYS_AFTER_EPIPHANY: [Office; 6] = [
    Office::Sunday {
        id: "dom-inf-oct-epiph",
        matins_id: None,
        rank: SundayRank::WithinOctave(OctaveRank::SecondOrder),
    },
    Office::Sunday {
        id: "dom-2-post-epiph",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-3-post-epiph",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-4-post-epiph",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-5-post-epiph",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-6-post-epiph",
        matins_id: None,
        rank: SundayRank::Common,
    },
];

const EPIPHANY: Office = Office::Feast(
    FeastDetails::new("in-epiphania-dnjc", FeastRank::DoubleFirstClass)
        .with_person(Person::OurLord)
        .make_feriata()
        .with_octave(OctaveRank::SecondOrder),
);

const EASTER: Office = Office::Feast(
    FeastDetails::new("dom-resurrectionis", FeastRank::DoubleFirstClass)
        .with_person(Person::OurLord)
        .make_feriata()
        .with_octave(OctaveRank::FirstOrder)
        .make_moveable(),
);

const ASCENSION: Office = Office::Feast(
    FeastDetails::new("in-ascensione-dnjc", FeastRank::DoubleFirstClass)
        .with_person(Person::OurLord)
        .make_feriata()
        .with_octave(OctaveRank::ThirdOrder)
        .make_moveable(),
);

const PENTECOST: Office = Office::Feast(
    FeastDetails::new("dom-pentecostes", FeastRank::DoubleFirstClass)
        .with_person(Person::Trinity)
        .make_feriata()
        .with_octave(OctaveRank::FirstOrder)
        .make_moveable(),
);

const TRINITY_SUNDAY: Office = Office::Feast(
    FeastDetails::new("dom-ss-trinitatis", FeastRank::DoubleFirstClass)
        .with_person(Person::Trinity)
        .make_feriata()
        .make_moveable(),
);

const CORPUS_CHRISTI: Office = Office::Feast(
    FeastDetails::new("ss-corporis-christi", FeastRank::DoubleFirstClass)
        .with_person(Person::OurLord)
        .make_feriata()
        .with_octave(OctaveRank::SecondOrder)
        .make_moveable(),
);

const SACRED_HEART: Office = Office::Feast(
    FeastDetails::new("ss-cordis-jesu", FeastRank::DoubleFirstClass)
        .with_person(Person::OurLady)
        .with_octave(OctaveRank::ThirdOrder)
        .make_moveable(),
);

const N_EASTER_CYCLE_SUNDAYS: usize = 39;
const EASTER_CYCLE_SUNDAYS: [Office; N_EASTER_CYCLE_SUNDAYS] = [
    Office::Sunday {
        id: "dom-in-septuagesima",
        matins_id: None,
        rank: SundayRank::SecondClass,
    },
    Office::Sunday {
        id: "dom-in-sexagesima",
        matins_id: None,
        rank: SundayRank::SecondClass,
    },
    Office::Sunday {
        id: "dom-in-quinquagesima",
        matins_id: None,
        rank: SundayRank::SecondClass,
    },
    Office::Sunday {
        id: "dom-1-quad",
        matins_id: None,
        rank: SundayRank::FirstClass,
    },
    Office::Sunday {
        id: "dom-2-quad",
        matins_id: None,
        rank: SundayRank::FirstClass,
    },
    Office::Sunday {
        id: "dom-3-quad",
        matins_id: None,
        rank: SundayRank::FirstClass,
    },
    Office::Sunday {
        id: "dom-4-quad",
        matins_id: None,
        rank: SundayRank::FirstClass,
    },
    Office::Sunday {
        id: "dom-passionis",
        matins_id: None,
        rank: SundayRank::FirstClass,
    },
    Office::Sunday {
        id: "dom-in-palmis",
        matins_id: None,
        rank: SundayRank::FirstClass,
    },
    EASTER,
    Office::Sunday {
        id: "dom-1-post-pascha",
        matins_id: None,
        rank: SundayRank::FirstClass,
    },
    Office::Sunday {
        id: "dom-2-post-pascha",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-3-post-pascha",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-4-post-pascha",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-5-post-pascha",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-inf-oct-asc",
        matins_id: None,
        rank: SundayRank::WithinOctave(OctaveRank::ThirdOrder),
    },
    PENTECOST,
    Office::Sunday {
        id: "dom-1-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-inf-oct-ss-corporis-christi",
        matins_id: None,
        rank: SundayRank::WithinOctave(OctaveRank::SecondOrder),
    },
    Office::Sunday {
        id: "dom-inf-oct-ss-cordis-jesu",
        matins_id: None,
        rank: SundayRank::WithinOctave(OctaveRank::ThirdOrder),
    },
    Office::Sunday {
        id: "dom-4-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-5-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-6-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-7-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-8-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-9-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-10-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-11-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-12-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-13-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-14-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-15-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-16-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-17-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-18-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-19-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-20-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-21-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
    Office::Sunday {
        id: "dom-22-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    },
];

// returns a vector containing a vector of offices for each day, plus (possibly) an anticipated
// Sunday which needs to be added to the calendar after feasts
fn temporal_cycle<'a>(year: i32) -> Vec<Vec<Office<'a>>> {
    let n_days = if is_leap_year(year) { 366 } else { 365 };
    let mut days = vec![Vec::new(); n_days];
    let ordinal0 = |month: u32, day: u32| {
        let day = if is_leap_year(year) && month == 2 && day >= 24 {
            day + 1
        } else {
            day
        };
        NaiveDate::from_ymd_opt(year, month, day)
            .expect("invalid date")
            .ordinal0() as usize
    };

    // Easter cycle
    let easter = easter::date(year).unwrap().ordinal0() as usize;
    let septuagesima = easter - 63;
    for week in 0..N_EASTER_CYCLE_SUNDAYS {
        days[septuagesima + (7 * week)].push(EASTER_CYCLE_SUNDAYS[week])
    }
    // Lenten ferias
    let lent1 = easter - 42;
    days[lent1 - 4].push(Office::named_feria(
        "dies-cinerum",
        FeriaRank::Privileged,
        true,
    ));
    days[lent1 - 3].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
    days[lent1 - 2].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
    days[lent1 - 1].push(Office::unnamed_feria(FeriaRank::ThirdClass, false));
    days[lent1 + 1].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
    days[lent1 + 2].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
    days[lent1 + 3].push(Office::named_feria(
        "fer-4-qt-quad",
        FeriaRank::ThirdClass,
        true,
    ));
    days[lent1 + 4].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
    days[lent1 + 5].push(Office::named_feria(
        "fer-6-qt-quad",
        FeriaRank::ThirdClass,
        true,
    ));
    days[lent1 + 6].push(Office::named_feria(
        "sab-qt-quad",
        FeriaRank::ThirdClass,
        false,
    ));
    for week in 1..4 {
        for day in 1..5 {
            days[lent1 + (7 * week) + day].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        }
        days[lent1 + (7 * week) + 6].push(Office::unnamed_feria(FeriaRank::ThirdClass, false));
    }

    // Holy week
    let palm_sunday = easter - 7;
    for day in 1..4 {
        days[palm_sunday + day].push(Office::unnamed_feria(FeriaRank::Privileged, true));
    }
    days[palm_sunday + 4].push(Office::named_feria(
        "in-coena-domini",
        FeriaRank::DoubleFirstClass,
        true,
    ));
    days[palm_sunday + 5].push(Office::named_feria(
        "in-parasceve",
        FeriaRank::DoubleFirstClass,
        true,
    ));
    days[palm_sunday + 6].push(Office::named_feria(
        "sabbato-sancto",
        FeriaRank::DoubleFirstClass,
        false,
    ));

    // Easter week
    days[easter + 1].push(Office::named_feria(
        "fer-2-paschatis",
        FeriaRank::DoubleFirstClass,
        true,
    ));
    days[easter + 2].push(Office::named_feria(
        "fer-3-paschatis",
        FeriaRank::DoubleFirstClass,
        true,
    ));
    let inf_oct_pascha = EASTER.day_within_octave().unwrap();
    for day in 3..6 {
        days[easter + day].push(inf_oct_pascha);
    }
    days[easter + 6].push(Office::WithinOctave {
        feast_details: EASTER.feast_details().unwrap(),
        rank: OctaveRank::FirstOrder,
        has_second_vespers: false,
    });

    // Rogation Monday
    days[easter + 36].push(Office::Feria {
        id: Some("fer-2-in-rogationibus"),
        rank: FeriaRank::ThirdClass,
        has_second_vespers: true,
        commemorated_at_vespers: false,
    });

    // Ascension and its octave
    let ascension = easter + 39;
    days[ascension - 1].push(Office::Vigil {
        feast_details: ASCENSION.feast_details().unwrap(),
        rank: VigilRank::Common,
    });
    days[ascension].push(ASCENSION);
    let inf_oct_asc = ASCENSION.day_within_octave().unwrap();
    for day in 1..7 {
        days[ascension + day].push(inf_oct_asc);
    }
    days[ascension + 7].push(ASCENSION.octave_day().unwrap());
    // TODO: special status for the following Friday

    // Pentecost and its octave
    let pentecost = easter + 49;
    days[pentecost - 1].push(Office::Vigil {
        rank: VigilRank::FirstClass,
        feast_details: PENTECOST.feast_details().unwrap(),
    });
    let inf_oct_pent = PENTECOST.day_within_octave().unwrap();
    days[pentecost + 1].push(Office::named_feria(
        "fer-2-pentecostes",
        FeriaRank::DoubleFirstClass,
        true,
    ));
    days[easter + 2].push(Office::named_feria(
        "fer-3-pentecostes",
        FeriaRank::DoubleFirstClass,
        true,
    ));
    for day in 3..6 {
        days[pentecost + day].push(inf_oct_pent);
    }
    days[pentecost + 6].push(Office::WithinOctave {
        feast_details: PENTECOST.feast_details().unwrap(),
        rank: OctaveRank::FirstOrder,
        has_second_vespers: false,
    });
    days[pentecost + 7].push(TRINITY_SUNDAY);

    // Corpus Christi and its octave
    let corpus_christi = pentecost + 11;
    days[corpus_christi].push(CORPUS_CHRISTI);
    let inf_oct_cc = CORPUS_CHRISTI.day_within_octave().unwrap();
    for day in 1..7 {
        days[corpus_christi + day].push(inf_oct_cc);
    }
    days[corpus_christi + 7].push(CORPUS_CHRISTI.octave_day().unwrap());

    // Sacred heart and its octave
    let sacred_heart = corpus_christi + 8;
    days[sacred_heart].push(SACRED_HEART);
    let inf_oct_sacred_heart = SACRED_HEART.day_within_octave().unwrap();
    for day in 1..7 {
        days[sacred_heart + day].push(inf_oct_sacred_heart);
    }
    days[sacred_heart + 7].push(SACRED_HEART.octave_day().unwrap());

    // Advent cycle
    let christmas_date = NaiveDate::from_ymd_opt(year, 12, 25).unwrap();
    let christmas = christmas_date.ordinal0() as usize;
    let advent1 = christmas - (christmas_date.weekday().number_from_monday() as usize) - 21;
    for week in 0..2 {
        days[advent1 + (week * 7)].push(SUNDAYS_OF_ADVENT[week]);
        for day in 1..6 {
            days[advent1 + (week * 7) + day]
                .push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        }
        days[advent1 + (week * 7) + 6].push(Office::unnamed_feria(FeriaRank::ThirdClass, false));
    }
    days[advent1 + 14].push(SUNDAYS_OF_ADVENT[2]);
    days[advent1 + 15].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
    days[advent1 + 16].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
    days[advent1 + 17].push(Office::named_feria(
        "fer-4-qt-advent",
        FeriaRank::ThirdClass,
        true,
    ));
    days[advent1 + 18].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
    days[advent1 + 19].push(Office::named_feria(
        "fer-6-qt-advent",
        FeriaRank::ThirdClass,
        true,
    ));
    days[advent1 + 20].push(Office::named_feria(
        "sab-qt-advent",
        FeriaRank::ThirdClass,
        false,
    ));
    days[advent1 + 21].push(SUNDAYS_OF_ADVENT[3]);
    for day in 1..6 {
        days[advent1 + 21 + day].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
    }
    days[advent1 + 27].push(Office::unnamed_feria(FeriaRank::ThirdClass, false));
    // Epiphany cycle
    let epiphany = ordinal0(1, 6);
    days[epiphany].push(EPIPHANY);
    let inf_oct_epiph = EPIPHANY.day_within_octave().unwrap();
    for day in 1..7 {
        days[epiphany + day].push(inf_oct_epiph);
    }
    days[epiphany + 7].push(EPIPHANY.octave_day().unwrap());

    let epiphany_date = NaiveDate::from_ymd_opt(year, 1, 6).expect("year out of range");
    let epiphany = epiphany_date.ordinal0() as usize;
    let dom_post_epiph = epiphany + 7 - (epiphany_date.weekday().number_from_monday() as usize);
    let mut last_sunday_after_epiph = 0;
    for week in 0..7 {
        let ord = dom_post_epiph + (week * 7);
        if ord >= septuagesima {
            break;
        }
        days[ord].push(SUNDAYS_AFTER_EPIPHANY[week]);
        last_sunday_after_epiph += 1;
    }

    // End of the Pentecost cycle
    let post_pent_24 = advent1 - 7;
    days[post_pent_24].push(Office::Sunday {
        id: "dom-24-post-pent",
        matins_id: None,
        rank: SundayRank::Common,
    });
    let post_pent_23 = pentecost + 23 * 7;
    if post_pent_23 == post_pent_24 {
        days[post_pent_24 - 1].push(Office::Feria {
            id: Some("dom-23-post-pent"),
            rank: FeriaRank::AnticipatedSunday,
            has_second_vespers: false,
            commemorated_at_vespers: false,
        });
    } else {
        days[post_pent_23].push(Office::Sunday {
            id: "dom-23-post-pent",
            matins_id: None,
            rank: SundayRank::Common,
        });
    }

    let mut first_resumed_sunday_post_epiph = 7;
    for i in 0..6 {
        let ord = post_pent_24 - ((i + 1) * 7);
        if ord <= post_pent_23 {
            break;
        }
        days[ord].push(SUNDAYS_AFTER_EPIPHANY[5 - i]);
        first_resumed_sunday_post_epiph -= 1;
    }
    assert!(first_resumed_sunday_post_epiph > last_sunday_after_epiph);
    let n_missing_sundays = first_resumed_sunday_post_epiph - last_sunday_after_epiph - 1;
    if n_missing_sundays == 1 {
        let missing_sunday = last_sunday_after_epiph + 1;
        let Office::Sunday { id, .. } = SUNDAYS_AFTER_EPIPHANY[missing_sunday - 1] else {
            panic!()
        };
        days[septuagesima - 1].push(Office::Feria {
            id: Some(id),
            rank: FeriaRank::AnticipatedSunday,
            has_second_vespers: false,
            commemorated_at_vespers: false,
        });
    } else {
        assert!(n_missing_sundays == 0);
    }

    // fill in the rest of the year with ferias / OLOS
    for week in 0..50 {
        for day in 1..7 {
            let ord = dom_post_epiph + (week * 7) + day;
            if days[ord].is_empty() {
                let office = if day == 6 {
                    Office::OurLadyOnSaturday
                } else {
                    Office::unnamed_feria(FeriaRank::Common, true)
                };
                days[ord].push(office);
            }
        }
    }

    // omit everything from Christmas onwards; it will be dealt with later as a special case
    for day in christmas..n_days {
        days[day].truncate(0);
    }

    days
}
