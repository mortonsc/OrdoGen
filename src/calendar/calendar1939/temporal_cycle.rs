use super::*;

static SUNDAYS_OF_ADVENT: [Office; 4] = [
    Office::sunday("dom-1-advent", SundayRank::FirstClass),
    Office::sunday("dom-2-advent", SundayRank::SecondClass),
    Office::sunday("dom-3-advent", SundayRank::SecondClass),
    Office::sunday("dom-4-advent", SundayRank::SecondClass),
];

const SUNDAY_WITHIN_OCT_NAT: Office = Office::sunday(
    "dom-inf-oct-nativitatis",
    SundayRank::WithinOctave(OctaveRank::ThirdOrder),
);

const HOLY_NAME: Office = Office::feast("ss-nominis-jesu", FeastRank::DoubleSecondClass)
    .with_person(Person::OurLord)
    .make_secondary()
    .done();

static SUNDAYS_AFTER_EPIPHANY: [Office; 6] = [
    Office::sunday(
        "dom-inf-oct-epiphaniae",
        SundayRank::WithinOctave(OctaveRank::SecondOrder),
    ),
    Office::sunday("dom-2-post-epiph", SundayRank::Common),
    Office::sunday("dom-3-post-epiph", SundayRank::Common),
    Office::sunday("dom-4-post-epiph", SundayRank::Common),
    Office::sunday("dom-5-post-epiph", SundayRank::Common),
    Office::sunday("dom-6-post-epiph", SundayRank::Common),
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
    .with_person(Person::OurLord)
    .make_feriatum()
    .with_vigil(VigilRank::FirstClass)
    .with_octave(OctaveRank::FirstOrder)
    .done();

const TRINITY_SUNDAY: Office = Office::feast("dom-ss-trinitatis", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
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
    Office::sunday("dom-in-septuagesima", SundayRank::SecondClass),
    Office::sunday("dom-in-sexagesima", SundayRank::SecondClass),
    Office::sunday("dom-in-quinquagesima", SundayRank::SecondClass),
    Office::sunday("dom-1-quad", SundayRank::FirstClass),
    Office::sunday("dom-2-quad", SundayRank::FirstClass),
    Office::sunday("dom-3-quad", SundayRank::FirstClass),
    Office::sunday("dom-4-quad", SundayRank::FirstClass),
    Office::sunday("dom-passionis", SundayRank::FirstClass),
    Office::sunday("dom-in-palmis", SundayRank::FirstClass),
    EASTER,
    Office::sunday("dom-1-post-pascha", SundayRank::FirstClass),
    Office::sunday("dom-2-post-pascha", SundayRank::Common),
    Office::sunday("dom-3-post-pascha", SundayRank::Common),
    Office::sunday("dom-4-post-pascha", SundayRank::Common),
    Office::sunday("dom-5-post-pascha", SundayRank::Common),
    Office::sunday(
        "dom-inf-oct-ascensionis",
        SundayRank::WithinOctave(OctaveRank::ThirdOrder),
    ),
    PENTECOST,
    Office::sunday("dom-1-post-pent", SundayRank::Common),
    Office::sunday(
        "dom-inf-oct-ss-corporis-christi",
        SundayRank::WithinOctave(OctaveRank::ThirdOrder),
    ),
    Office::sunday(
        "dom-inf-oct-ss-cordis-jesu",
        SundayRank::WithinOctave(OctaveRank::ThirdOrder),
    ),
    Office::sunday("dom-4-post-pent", SundayRank::Common),
    Office::sunday("dom-5-post-pent", SundayRank::Common),
    Office::sunday("dom-6-post-pent", SundayRank::Common),
    Office::sunday("dom-7-post-pent", SundayRank::Common),
    Office::sunday("dom-8-post-pent", SundayRank::Common),
    Office::sunday("dom-9-post-pent", SundayRank::Common),
    Office::sunday("dom-10-post-pent", SundayRank::Common),
    Office::sunday("dom-11-post-pent", SundayRank::Common),
    Office::sunday("dom-12-post-pent", SundayRank::Common),
    Office::sunday("dom-13-post-pent", SundayRank::Common),
    Office::sunday("dom-14-post-pent", SundayRank::Common),
    Office::sunday("dom-15-post-pent", SundayRank::Common),
    Office::sunday("dom-16-post-pent", SundayRank::Common),
    Office::sunday("dom-17-post-pent", SundayRank::Common),
    Office::sunday("dom-18-post-pent", SundayRank::Common),
    Office::sunday("dom-19-post-pent", SundayRank::Common),
    Office::sunday("dom-20-post-pent", SundayRank::Common),
    Office::sunday("dom-21-post-pent", SundayRank::Common),
    Office::sunday("dom-22-post-pent", SundayRank::Common),
    // We leave out the 23rd (because it is sometimes anticipated to Saturday) and the 24th
    // (because it always comes last before Advent and so is not fixed relative to Easter)
];

fn add_matins_id(ch: CalendarHelper, sunday: Office, ord: usize) -> Office {
    let matins_id = match ch.month_day(ord) {
        (7, 29..=31) | (8, 1..=4) => "dom-1-aug",
        (8, 5..=11) => "dom-2-aug",
        (8, 12..=18) => "dom-3-aug",
        (8, 19..=25) => "dom-4-aug",
        (8, 26..=28) => "dom-5-aug",
        (8, 29..=31) | (9, 1..=4) => "dom-1-sept",
        (9, 5..=11) => "dom-2-sept",
        (9, 12..=18) => "dom-3-sept",
        (9, 19..=25) => "dom-4-sept",
        (9, 26..=27) => "dom-5-sept",
        (9, 28..=30) | (10, 1..=4) => "dom-1-oct",
        (10, 5..=11) => "dom-2-oct",
        (10, 12..=18) => "dom-3-oct",
        (10, 19..=25) => "dom-4-oct",
        (10, 26..=28) => "dom-5-oct",
        // November is different because in years when it only has 4 weeks, the 2nd week is omitted
        (10, 29..=31) | (11, 1..=4) => "dom-1-nov",
        (11, 5) => "dom-2-nov",
        (11, 6..=12) => "dom-3-nov",
        (11, 13..=19) => "dom-4-nov",
        (11, 20..=26) => "dom-5-nov",
        _ => return sunday,
    };
    sunday.with_matins_id(matins_id)
}

impl Calendar1939 {
    pub fn add_temporal_cycle_h(&self, ch: CalendarHelper, days: &mut [Vec<Office<'_>>]) {
        // Advent cycle
        for week in 0..=1 {
            days[ch.advent1() + (week * 7)].push(SUNDAYS_OF_ADVENT[week]);
            for weekday in 1..=6 {
                days[ch.advent1() + (week * 7) + weekday]
                    .push(Office::feria(FeriaRank::ThirdClass, weekday < 6));
            }
        }
        days[ch.advent1() + 14].push(SUNDAYS_OF_ADVENT[2]);
        days[ch.advent1() + 15].push(Office::feria(FeriaRank::ThirdClass, true));
        days[ch.advent1() + 16].push(Office::feria(FeriaRank::ThirdClass, true));
        days[ch.advent1() + 17]
            .push(Office::feria(FeriaRank::ThirdClass, true).with_id("fer-4-qt-advent"));
        days[ch.advent1() + 18].push(Office::feria(FeriaRank::ThirdClass, true));
        days[ch.advent1() + 19]
            .push(Office::feria(FeriaRank::ThirdClass, true).with_id("fer-6-qt-advent"));
        days[ch.advent1() + 20]
            .push(Office::feria(FeriaRank::ThirdClass, false).with_id("sab-qt-advent"));
        days[ch.advent1() + 21].push(SUNDAYS_OF_ADVENT[3]);
        for day in 1..6 {
            let ord = ch.advent1() + 21 + day;
            if ord >= ch.christmas() - 1 {
                break;
            }
            days[ord].push(Office::feria(FeriaRank::ThirdClass, true));
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
        if dom_post_epiph == ch.ordinal0(1, 13) {
            days[dom_post_epiph - 1].push(SUNDAYS_AFTER_EPIPHANY[0]);
        } else {
            days[dom_post_epiph].push(SUNDAYS_AFTER_EPIPHANY[0]);
        }
        let mut last_sunday_after_epiph = 1;
        for (week, &sunday) in SUNDAYS_AFTER_EPIPHANY.iter().enumerate().skip(1) {
            let ord = dom_post_epiph + (week * 7);
            if ord >= ch.septuagesima() {
                break;
            }
            days[ord].push(sunday);
            last_sunday_after_epiph += 1;
        }

        // Easter cycle
        for week in 0..N_EASTER_CYCLE_SUNDAYS {
            let ord = ch.septuagesima() + (7 * week);
            days[ord].push(add_matins_id(ch, EASTER_CYCLE_SUNDAYS[week], ord));
        }
        // Lenten ferias
        let lent1 = ch.easter() - 42;
        days[lent1 - 4].push(Office::feria(FeriaRank::Privileged, true).with_id("dies-cinerum"));
        days[lent1 - 3]
            .push(Office::feria(FeriaRank::ThirdClass, true).with_id("fer-5-post-cineres"));
        days[lent1 - 2]
            .push(Office::feria(FeriaRank::ThirdClass, true).with_id("fer-6-post-cineres"));
        days[lent1 - 1]
            .push(Office::feria(FeriaRank::ThirdClass, false).with_id("sab-post-cineres"));
        days[lent1 + 1].push(Office::feria(FeriaRank::ThirdClass, true));
        days[lent1 + 2].push(Office::feria(FeriaRank::ThirdClass, true));
        days[lent1 + 3].push(Office::feria(FeriaRank::ThirdClass, true).with_id("fer-4-qt-quad"));
        days[lent1 + 4].push(Office::feria(FeriaRank::ThirdClass, true));
        days[lent1 + 5].push(Office::feria(FeriaRank::ThirdClass, true).with_id("fer-6-qt-quad"));
        days[lent1 + 6].push(Office::feria(FeriaRank::ThirdClass, false).with_id("sab-qt-quad"));
        for week in 1..=4 {
            for weekday in 1..=6 {
                days[lent1 + (7 * week) + weekday]
                    .push(Office::feria(FeriaRank::ThirdClass, weekday < 6));
            }
        }

        // Holy week
        let palm_sunday = ch.easter() - 7;
        for day in 1..4 {
            days[palm_sunday + day].push(Office::feria(FeriaRank::Privileged, true));
        }
        days[palm_sunday + 4]
            .push(Office::feria(FeriaRank::DoubleFirstClass, true).with_id("in-coena-domini"));
        days[palm_sunday + 5]
            .push(Office::feria(FeriaRank::DoubleFirstClass, true).with_id("in-parasceve"));
        days[palm_sunday + 6]
            .push(Office::feria(FeriaRank::DoubleFirstClass, false).with_id("sabbato-sancto"));

        // Easter week
        days[ch.easter() + 1]
            .push(Office::feria(FeriaRank::DoubleFirstClass, true).with_id("fer-2-paschatis"));
        days[ch.easter() + 2]
            .push(Office::feria(FeriaRank::DoubleFirstClass, true).with_id("fer-3-paschatis"));
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

        // The vigil and octave of the ascension will be added automatically
        let ascension = ch.easter() + 39;
        days[ascension].push(ASCENSION);
        days[ascension + 8].push(
            Office::feria(FeriaRank::FridayAfterOctAsc, true).with_id("fer-6-post-oct-ascensionis"),
        );

        // Pentecost and its octave
        // the vigil will be filled in automatically later
        let pentecost = ch.easter() + 49;
        let inf_oct_pent = PENTECOST.day_within_octave().unwrap();
        days[pentecost + 1]
            .push(Office::feria(FeriaRank::DoubleFirstClass, true).with_id("fer-2-pentecostes"));
        days[pentecost + 2]
            .push(Office::feria(FeriaRank::DoubleFirstClass, true).with_id("fer-3-pentecostes"));
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
        let dom_3_sept = ch.sunday_after(ch.ordinal0(9, 11)).unwrap();
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
        days[dom_3_sept + 6]
            .push(Office::feria(FeriaRank::ThirdClass, false).with_id("sab-qt-sept"));

        // End of the Pentecost cycle
        let post_pent_24 = ch.advent1() - 7;
        days[post_pent_24].push(add_matins_id(
            ch,
            Office::sunday("dom-24-post-pent", SundayRank::Common),
            post_pent_24,
        ));
        let post_pent_23 = pentecost + (23 * 7);
        if post_pent_23 == post_pent_24 {
            days[post_pent_24 - 1].push(
                Office::feria(FeriaRank::AnticipatedSunday, false).with_id("dom-23-post-pent"),
            );
        } else {
            days[post_pent_23].push(add_matins_id(
                ch,
                Office::sunday("dom-23-post-pent", SundayRank::Common),
                post_pent_23,
            ));
        }

        let mut first_resumed_sunday_after_epiph = 7;
        for i in 1..=6 {
            let ord = post_pent_24 - (i * 7);
            if ord <= post_pent_23 {
                break;
            }
            days[ord].push(add_matins_id(ch, SUNDAYS_AFTER_EPIPHANY[6 - i], ord));
            first_resumed_sunday_after_epiph -= 1;
        }
        assert!(last_sunday_after_epiph < first_resumed_sunday_after_epiph);
        let n_missing_sundays = first_resumed_sunday_after_epiph - last_sunday_after_epiph - 1;
        if n_missing_sundays == 1 {
            let missing_sunday = last_sunday_after_epiph + 1;
            days[ch.septuagesima() - 1].push(
                Office::feria(FeriaRank::AnticipatedSunday, false)
                    .with_id(SUNDAYS_AFTER_EPIPHANY[missing_sunday - 1].id().unwrap()),
            );
        } else {
            assert!(n_missing_sundays == 0);
        }

        // fill in the rest of the year with ferias / OLOS
        // strictly speaking we don't have to check that the days are open, because the rubrics
        // system will automatically omit common ferias / OLOS in occurrence
        for day in 10..340 {
            let weekday = ch.weekday(day);
            if weekday == Weekday::Sun {
                continue;
            } else if weekday == Weekday::Sat
                && days[day]
                    .iter()
                    .all(|&o| Rubrics1939.admits_our_lady_on_saturday(o))
            {
                days[day].push(Office::OurLadyOnSaturday);
            } else if days[day]
                .iter()
                .all(|&o| Rubrics1939.admits_common_feria(o))
            {
                days[day].push(Office::feria(FeriaRank::Common, true));
            }
        }
    }
}
