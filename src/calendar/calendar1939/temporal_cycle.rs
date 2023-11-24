use super::*;

static SUNDAYS_OF_ADVENT: [Office; 4] = [
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

const SUNDAY_WITHIN_OCT_NAT: Office = Office::Sunday {
    id: "dom-inf-oct-nat",
    matins_id: None,
    rank: SundayRank::WithinOctave(OctaveRank::ThirdOrder),
};

const HOLY_NAME: Office = Office::feast("ss-nominis-jesu", FeastRank::DoubleSecondClass)
    .with_person(Person::OurLord)
    .make_secondary()
    .done();

static SUNDAYS_AFTER_EPIPHANY: [Office; 6] = [
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

const EASTER: Office = Office::feast("dom-resurrectionis", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .make_feriatum()
    .with_octave(OctaveRank::FirstOrder)
    .done();

const ASCENSION: Office = Office::feast("in-ascensione-dnjc", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .make_feriatum()
    .with_vigil(VigilRank::Common)
    .with_octave(OctaveRank::ThirdOrder)
    .done();

const PENTECOST: Office = Office::feast("dom-pentecostes", FeastRank::DoubleFirstClass)
    .with_person(Person::Trinity)
    .make_feriatum()
    .with_vigil(VigilRank::FirstClass)
    .with_octave(OctaveRank::FirstOrder)
    .done();

const TRINITY_SUNDAY: Office = Office::feast("dom-ss-trinitatis", FeastRank::DoubleFirstClass)
    .with_person(Person::Trinity)
    .make_feriatum()
    .done();

const CORPUS_CHRISTI: Office = Office::feast("ss-corporis-christi", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .make_feriatum()
    .with_octave(OctaveRank::SecondOrder)
    .done();

const SACRED_HEART: Office = Office::feast("ss-cordis-jesu", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLady)
    .with_octave(OctaveRank::ThirdOrder)
    .done();

const N_EASTER_CYCLE_SUNDAYS: usize = 39;
static EASTER_CYCLE_SUNDAYS: [Office; N_EASTER_CYCLE_SUNDAYS] = [
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
    pub fn add_temporal_cycle_h(&self, ch: CalendarHelper, days: &mut [Vec<Office<'_>>]) {
        // Easter cycle
        for week in 0..N_EASTER_CYCLE_SUNDAYS {
            days[ch.septuagesima() + (7 * week)].push(EASTER_CYCLE_SUNDAYS[week])
        }
        // Lenten ferias
        let lent1 = ch.easter() - 42;
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
        let palm_sunday = ch.easter() - 7;
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
        days[ch.easter() + 1].push(Office::named_feria(
            "fer-2-paschatis",
            FeriaRank::DoubleFirstClass,
            true,
        ));
        days[ch.easter() + 2].push(Office::named_feria(
            "fer-3-paschatis",
            FeriaRank::DoubleFirstClass,
            true,
        ));
        let inf_oct_pascha = EASTER.day_within_octave().unwrap();
        for day in 3..6 {
            days[ch.easter() + day].push(inf_oct_pascha);
        }
        days[ch.easter() + 6].push(Office::WithinOctave {
            feast_details: EASTER.feast_details().unwrap(),
            rank: OctaveRank::FirstOrder,
            has_second_vespers: false,
        });

        // Rogation Monday
        days[ch.easter() + 36].push(Office::Feria {
            id: Some("fer-2-in-rogationibus"),
            rank: FeriaRank::ThirdClass,
            has_second_vespers: true,
            commemorated_at_vespers: false,
        });

        // Ascension and its octave
        let ascension = ch.easter() + 39;
        days[ascension - 1].push(ASCENSION.vigil().unwrap());
        days[ascension].push(ASCENSION);
        days[ascension + 8].push(Office::Feria {
            id: Some("fer-6-post-oct-asc"),
            rank: FeriaRank::FridayAfterOctAsc,
            has_second_vespers: true,
            commemorated_at_vespers: true,
        });

        // Pentecost and its octave
        let pentecost = ch.easter() + 49;
        days[pentecost - 1].push(PENTECOST.vigil().unwrap());
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

        let corpus_christi = pentecost + 11;
        days[corpus_christi].push(CORPUS_CHRISTI);

        let sacred_heart = corpus_christi + 8;
        days[sacred_heart].push(SACRED_HEART);

        // fall ember days
        let dom_3_sept = ch.sunday_after(ch.ordinal0(9, 14)).unwrap();
        days[dom_3_sept + 3].push(Office::Feria {
            id: Some("fer-4-qt-sept"),
            rank: FeriaRank::ThirdClass,
            has_second_vespers: true,
            commemorated_at_vespers: false,
        });
        days[dom_3_sept + 5].push(Office::Feria {
            id: Some("fer-6-qt-sept"),
            rank: FeriaRank::ThirdClass,
            has_second_vespers: true,
            commemorated_at_vespers: false,
        });
        days[dom_3_sept + 6].push(Office::named_feria(
            "sab-qt-sept",
            FeriaRank::ThirdClass,
            false,
        ));

        // Advent cycle
        for week in 0..2 {
            days[ch.advent1() + (week * 7)].push(SUNDAYS_OF_ADVENT[week]);
            for day in 1..6 {
                days[ch.advent1() + (week * 7) + day]
                    .push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
            }
            days[ch.advent1() + (week * 7) + 6]
                .push(Office::unnamed_feria(FeriaRank::ThirdClass, false));
        }
        days[ch.advent1() + 14].push(SUNDAYS_OF_ADVENT[2]);
        days[ch.advent1() + 15].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        days[ch.advent1() + 16].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        days[ch.advent1() + 17].push(Office::named_feria(
            "fer-4-qt-advent",
            FeriaRank::ThirdClass,
            true,
        ));
        days[ch.advent1() + 18].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        days[ch.advent1() + 19].push(Office::named_feria(
            "fer-6-qt-advent",
            FeriaRank::ThirdClass,
            true,
        ));
        days[ch.advent1() + 20].push(Office::named_feria(
            "sab-qt-advent",
            FeriaRank::ThirdClass,
            false,
        ));
        days[ch.advent1() + 21].push(SUNDAYS_OF_ADVENT[3]);
        for day in 1..6 {
            let ord = ch.advent1() + 21 + day;
            if ord >= ch.christmas() - 1 {
                break;
            }
            days[ord].push(Office::unnamed_feria(FeriaRank::ThirdClass, true));
        }

        // Christmas cycle
        let sunday_inf_oct_nat = ch.sunday_after(ch.christmas());
        // If there is no Sunday within the octave, or it falls on one of the first 3 days,
        // the Sunday within the octave is celebrated on the 29th
        // TODO: account for local calendars
        let sunday_inf_oct_nat = match sunday_inf_oct_nat.map(|d| d - ch.christmas()) {
            None | Some(1..=3) => ch.ordinal0(12, 30),
            Some(_) => sunday_inf_oct_nat.unwrap(),
        };
        days[sunday_inf_oct_nat].push(SUNDAY_WITHIN_OCT_NAT);

        // TODO: account for the case when a higher-ranked feast falls in this period
        let holy_name = ch.sunday_after(0).unwrap();
        let holy_name = if holy_name > 4 { 1 } else { holy_name };
        days[holy_name].push(HOLY_NAME);

        // Epiphany cycle
        let dom_post_epiph = ch.sunday_after(ch.epiphany()).unwrap();
        let dom_post_epiph = if dom_post_epiph == ch.ordinal0(1, 13) {
            dom_post_epiph - 1
        } else {
            dom_post_epiph
        };
        days[dom_post_epiph].push(SUNDAYS_AFTER_EPIPHANY[0]);
        let mut last_sunday_after_epiph = 1;
        for (week, &sunday) in SUNDAYS_AFTER_EPIPHANY.iter().enumerate().skip(1) {
            let ord = dom_post_epiph + (week * 7);
            if ord >= ch.septuagesima() {
                break;
            }
            days[ord].push(sunday);
            last_sunday_after_epiph += 1;
        }

        // End of the Pentecost cycle
        let post_pent_24 = ch.advent1() - 7;
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
            for weekday in 1..7 {
                let day = dom_post_epiph + (week * 7) + weekday;
                if weekday == 6
                    && days[day]
                        .iter()
                        .all(|&o| Rubrics1939.admits_our_lady_on_saturday(o))
                {
                    days[day].push(Office::OurLadyOnSaturday);
                } else if days[day]
                    .iter()
                    .all(|&o| Rubrics1939.admits_common_feria(o))
                {
                    days[day].push(Office::unnamed_feria(FeriaRank::Common, true));
                }
            }
        }
    }
}
