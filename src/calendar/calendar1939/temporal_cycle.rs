use super::*;

pub const SUNDAYS_OF_ADVENT: [Office; 4] = [
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

const NATIVITY: Office = Office::feast("nativitas-dnjc", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .with_octave(OctaveRank::ThirdOrder)
    .with_vigil(VigilRank::FirstClass)
    .done();

const ST_STEPHEN: Office = Office::feast("s-stephani-protomartyris", FeastRank::DoubleSecondClass)
    .with_octave(OctaveRank::Simple)
    .done();

const ST_JOHN_EV: Office = Office::feast("s-joannis-ap-ev", FeastRank::DoubleSecondClass)
    .with_person(Person::Apostle)
    .with_octave(OctaveRank::Simple)
    .done();

const HOLY_INNOCENTS: Office = Office::feast("ss-innocentium-mm", FeastRank::DoubleSecondClass)
    .with_octave(OctaveRank::Simple)
    .done();

const ST_THOMAS_BECKET: Office = Office::feast("s-thomas-em", FeastRank::Double).done();

const ST_SILVESTER: Office = Office::feast("s-silvestri-i-pc", FeastRank::Double).done();

const SUNDAY_WITHIN_OCT_NAT: Office = Office::Sunday {
    id: "dom-inf-oct-nat",
    matins_id: None,
    rank: SundayRank::WithinOctave(OctaveRank::ThirdOrder),
};

const CIRCUMCISION: Office = Office::feast("in-circumcisione-domini", FeastRank::DoubleSecondClass)
    .with_person(Person::OurLord)
    .done();

const HOLY_NAME: Office = Office::feast("ss-nominis-jesu", FeastRank::DoubleSecondClass)
    .with_person(Person::OurLord)
    .done();

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
        .make_feriatum()
        .with_vigil(VigilRank::SecondClass)
        .with_octave(OctaveRank::SecondOrder),
);

const EASTER: Office = Office::Feast(
    FeastDetails::new("dom-resurrectionis", FeastRank::DoubleFirstClass)
        .with_person(Person::OurLord)
        .make_feriatum()
        .with_octave(OctaveRank::FirstOrder)
        .make_moveable(),
);

const ASCENSION: Office = Office::Feast(
    FeastDetails::new("in-ascensione-dnjc", FeastRank::DoubleFirstClass)
        .with_person(Person::OurLord)
        .make_feriatum()
        .with_octave(OctaveRank::ThirdOrder)
        .make_moveable(),
);

const PENTECOST: Office = Office::Feast(
    FeastDetails::new("dom-pentecostes", FeastRank::DoubleFirstClass)
        .with_person(Person::Trinity)
        .make_feriatum()
        .with_octave(OctaveRank::FirstOrder)
        .make_moveable(),
);

const TRINITY_SUNDAY: Office = Office::Feast(
    FeastDetails::new("dom-ss-trinitatis", FeastRank::DoubleFirstClass)
        .with_person(Person::Trinity)
        .make_feriatum()
        .make_moveable(),
);

const CORPUS_CHRISTI: Office = Office::Feast(
    FeastDetails::new("ss-corporis-christi", FeastRank::DoubleFirstClass)
        .with_person(Person::OurLord)
        .make_feriatum()
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

impl Calendar1939 {
    pub fn temporal_cycle_h<'a>(&self, ch: CalendarHelper) -> Vec<Vec<Office<'a>>> {
        let n_days = if is_leap_year(ch.year) { 366 } else { 365 };
        let mut days = vec![Vec::new(); n_days];

        // Easter cycle
        for week in 0..N_EASTER_CYCLE_SUNDAYS {
            days[ch.septuagesima() + (7 * week)].push(EASTER_CYCLE_SUNDAYS[week])
        }
        // Lenten ferias
        let lent1 = ch.easter - 42;
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
        for week in 1..5 {
            for day in 1..6 {
                days[lent1 + (7 * week) + day]
                    .push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
            }
            days[lent1 + (7 * week) + 6].push(Office::unnamed_feria(FeriaRank::ThirdClass, false));
        }

        // Holy week
        let palm_sunday = ch.easter - 7;
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
        days[ch.easter + 1].push(Office::named_feria(
            "fer-2-paschatis",
            FeriaRank::DoubleFirstClass,
            true,
        ));
        days[ch.easter + 2].push(Office::named_feria(
            "fer-3-paschatis",
            FeriaRank::DoubleFirstClass,
            true,
        ));
        let inf_oct_pascha = EASTER.day_within_octave().unwrap();
        for day in 3..6 {
            days[ch.easter + day].push(inf_oct_pascha);
        }
        days[ch.easter + 6].push(Office::WithinOctave {
            feast_details: EASTER.feast_details().unwrap(),
            rank: OctaveRank::FirstOrder,
            has_second_vespers: false,
        });

        // Rogation Monday
        days[ch.easter + 36].push(Office::Feria {
            id: Some("fer-2-in-rogationibus"),
            rank: FeriaRank::ThirdClass,
            has_second_vespers: true,
            commemorated_at_vespers: false,
        });

        // Ascension and its octave
        let ascension = ch.easter + 39;
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
        let pentecost = ch.easter + 49;
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
        days[pentecost + 2].push(Office::named_feria(
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
        for week in 0..2 {
            days[ch.advent1 + (week * 7)].push(SUNDAYS_OF_ADVENT[week]);
            for day in 1..6 {
                days[ch.advent1 + (week * 7) + day]
                    .push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
            }
            days[ch.advent1 + (week * 7) + 6]
                .push(Office::unnamed_feria(FeriaRank::ThirdClass, false));
        }
        days[ch.advent1 + 14].push(SUNDAYS_OF_ADVENT[2]);
        days[ch.advent1 + 15].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        days[ch.advent1 + 16].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        days[ch.advent1 + 17].push(Office::named_feria(
            "fer-4-qt-advent",
            FeriaRank::ThirdClass,
            true,
        ));
        days[ch.advent1 + 18].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        days[ch.advent1 + 19].push(Office::named_feria(
            "fer-6-qt-advent",
            FeriaRank::ThirdClass,
            true,
        ));
        days[ch.advent1 + 20].push(Office::named_feria(
            "sab-qt-advent",
            FeriaRank::ThirdClass,
            false,
        ));
        days[ch.advent1 + 21].push(SUNDAYS_OF_ADVENT[3]);
        for day in 1..6 {
            let ord = ch.advent1 + 21 + day;
            if ord >= ch.christmas - 1 {
                break;
            }
            days[ord].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        }

        // Christmas cycle
        days[ch.christmas - 1].push(NATIVITY.vigil().unwrap());
        days[ch.christmas].push(NATIVITY);
        let inf_oct_nat = NATIVITY.day_within_octave().unwrap();
        for day in 1..7 {
            days[ch.christmas + day].push(inf_oct_nat)
        }
        days[ch.christmas + 1].push(ST_STEPHEN);
        days[ch.christmas + 2].push(ST_JOHN_EV);
        days[ch.christmas + 3].push(HOLY_INNOCENTS);
        days[ch.christmas + 4].push(ST_THOMAS_BECKET);
        days[ch.christmas + 6].push(ST_SILVESTER);
        let sunday_inf_oct_nat = ch.sunday_after(ch.christmas);
        // If there is no Sunday within the octave, or it falls on one of the first 3 days,
        // the Sunday within the octave is celebrated on the 29th
        // TODO: account for local calendars
        let sunday_inf_oct_nat = match sunday_inf_oct_nat.map(|d| d - ch.christmas) {
            None | Some(1..=3) => ch.ordinal0(12, 30),
            Some(_) => sunday_inf_oct_nat.unwrap(),
        };
        days[sunday_inf_oct_nat].push(SUNDAY_WITHIN_OCT_NAT);
        days[0].push(CIRCUMCISION);
        days[1].push(ST_STEPHEN.octave_day().unwrap());
        days[2].push(ST_JOHN_EV.octave_day().unwrap());
        days[3].push(HOLY_INNOCENTS.octave_day().unwrap());

        // Holy Name is celebrated on the Sunday between Jan 2 and Jan 5,
        // or on Jan 2 if there is no such Sunday
        let holy_name = ch.sunday_after(0).unwrap();
        let holy_name = if holy_name > 4 { 1 } else { holy_name };
        days[holy_name].push(HOLY_NAME);

        // Epiphany cycle
        let epiphany_date = NaiveDate::from_ymd_opt(ch.year, 1, 6).expect("year out of range");
        let epiphany = epiphany_date.ordinal0() as usize;
        days[epiphany - 1].push(EPIPHANY.vigil().unwrap());
        days[epiphany].push(EPIPHANY);
        let inf_oct_epiph = EPIPHANY.day_within_octave().unwrap();
        for day in 1..7 {
            days[epiphany + day].push(inf_oct_epiph);
        }
        days[epiphany + 7].push(EPIPHANY.octave_day().unwrap());

        let dom_post_epiph = ch.sunday_after(epiphany).unwrap();
        let mut last_sunday_after_epiph = 0;
        for (week, &sunday) in SUNDAYS_AFTER_EPIPHANY.iter().enumerate() {
            let ord = dom_post_epiph + (week * 7);
            if ord >= ch.septuagesima() {
                break;
            }
            days[ord].push(sunday);
            last_sunday_after_epiph += 1;
        }

        // End of the Pentecost cycle
        let post_pent_24 = ch.advent1 - 7;
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
            days[ch.septuagesima() - 1].push(Office::Feria {
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

        days
    }
}
