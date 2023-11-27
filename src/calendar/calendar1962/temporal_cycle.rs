use super::*;

// TODO: copied from 1939

static SUNDAYS_OF_ADVENT: [Office; 4] = [
    Office::sunday("dom-1-advent", SundayRank::FirstClass),
    Office::sunday("dom-2-advent", SundayRank::FirstClass),
    Office::sunday("dom-3-advent", SundayRank::FirstClass),
    Office::sunday("dom-4-advent", SundayRank::FirstClass),
];

const SUNDAY_WITHIN_OCT_NAT: Office =
    Office::sunday("dom-inf-oct-nativitatis", SundayRank::SecondClass);

const HOLY_NAME: Office = Office::feast("ss-nominis-jesu", FeastRank::DoubleSecondClass)
    .with_person(Person::OurLord)
    .done();

static SUNDAYS_AFTER_EPIPHANY: [Office; 6] = [
    Office::sunday("dom-1-post-epiph", SundayRank::SecondClass),
    Office::sunday("dom-2-post-epiph", SundayRank::SecondClass),
    Office::sunday("dom-3-post-epiph", SundayRank::SecondClass),
    Office::sunday("dom-4-post-epiph", SundayRank::SecondClass),
    Office::sunday("dom-5-post-epiph", SundayRank::SecondClass),
    Office::sunday("dom-6-post-epiph", SundayRank::SecondClass),
];

const EASTER: Office = Office::feast("dom-resurrectionis", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .with_octave(OctaveRank::FirstOrder)
    .done();

const ASCENSION: Office = Office::feast("in-ascensione-dnjc", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .with_vigil(VigilRank::Common)
    .done();

const PENTECOST: Office = Office::feast("dom-pentecostes", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .with_vigil(VigilRank::FirstClass)
    .with_octave(OctaveRank::FirstOrder)
    .done();

const TRINITY_SUNDAY: Office = Office::feast("dom-ss-trinitatis", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .done();

const CORPUS_CHRISTI: Office = Office::feast("ss-corporis-christi", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .done();

const SACRED_HEART: Office = Office::feast("ss-cordis-jesu", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLady)
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
    Office::sunday("dom-2-post-pascha", SundayRank::SecondClass),
    Office::sunday("dom-3-post-pascha", SundayRank::SecondClass),
    Office::sunday("dom-4-post-pascha", SundayRank::SecondClass),
    Office::sunday("dom-5-post-pascha", SundayRank::SecondClass),
    Office::sunday("dom-post-ascensionem", SundayRank::SecondClass),
    PENTECOST,
    Office::sunday("dom-1-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-2-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-3-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-4-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-5-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-6-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-7-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-8-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-9-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-10-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-11-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-12-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-13-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-14-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-15-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-16-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-17-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-18-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-19-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-20-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-21-post-pent", SundayRank::SecondClass),
    Office::sunday("dom-22-post-pent", SundayRank::SecondClass),
    // We leave out the 23rd (because it is sometimes omitted) and the 24th
    // (because it always comes last before Advent and so is not fixed relative to Easter)
];

fn add_matins_id(ch: CalendarHelper, sunday: Office, ord: usize) -> Office {
    let matins_id = match ch.month_day(ord) {
        (8, 1..=7) => "dom-1-aug",
        (8, 8..=14) => "dom-2-aug",
        (8, 15..=21) => "dom-3-aug",
        (8, 22..=28) => "dom-4-aug",
        (8, 29..=31) => "dom-5-aug",
        (9, 1..=7) => "dom-1-sept",
        (9, 8..=14) => "dom-2-sept",
        (9, 15..=21) => "dom-3-sept",
        (9, 22..=28) => "dom-4-sept",
        (9, 29..=30) => "dom-5-sept",
        (10, 1..=7) => "dom-1-oct",
        (10, 8..=14) => "dom-2-oct",
        (10, 15..=21) => "dom-3-oct",
        (10, 22..=28) => "dom-4-oct",
        (10, 29..=31) => "dom-5-oct",
        // The second week of November is always omitted
        // And the first week is omitted if there isn't space for it
        (11, 1..=5) => "dom-1-nov",
        (11, 6..=12) => "dom-3-nov",
        (11, 13..=19) => "dom-4-nov",
        (11, 20..=26) => "dom-5-nov",
        _ => return sunday,
    };
    sunday.with_matins_id(matins_id)
}

impl Calendar1962 {
    pub fn add_temporal_cycle_h(&self, ch: CalendarHelper, days: &mut [Vec<Office<'_>>]) {
        // Advent cycle
        for week in 0..2 {
            days[ch.advent1() + (week * 7)].push(SUNDAYS_OF_ADVENT[week]);
            for weekday in 1..=6 {
                days[ch.advent1() + (week * 7) + weekday]
                    .push(Office::feria(FeriaRank::ThirdClassAdvent, weekday < 6));
            }
        }
        days[ch.advent1() + 14].push(SUNDAYS_OF_ADVENT[2]);
        days[ch.advent1() + 15].push(Office::feria(FeriaRank::ThirdClassAdvent, true));
        days[ch.advent1() + 16].push(Office::feria(FeriaRank::ThirdClassAdvent, true));
        days[ch.advent1() + 17]
            .push(Office::feria(FeriaRank::SecondClass, true).with_id("fer-4-qt-advent"));
        days[ch.advent1() + 18].push(Office::feria(FeriaRank::ThirdClassAdvent, true));
        days[ch.advent1() + 19]
            .push(Office::feria(FeriaRank::SecondClass, true).with_id("fer-6-qt-advent"));
        days[ch.advent1() + 20]
            .push(Office::feria(FeriaRank::SecondClass, false).with_id("sab-qt-advent"));
        days[ch.advent1() + 21].push(SUNDAYS_OF_ADVENT[3]);
        for weekday in 1..=5 {
            let ord = ch.advent1() + 21 + weekday;
            if ord >= ch.christmas() - 1 {
                break;
            }
            days[ord].push(Office::feria(FeriaRank::ThirdClassAdvent, true));
        }

        // Christmas cycle
        let sunday_inf_oct_nat = ch.sunday_after(ch.christmas());
        if let Some(ord) = sunday_inf_oct_nat {
            days[ord].push(SUNDAY_WITHIN_OCT_NAT);
        }

        let holy_name = ch.sunday_after(0).unwrap();
        let holy_name = if holy_name > 4 { 1 } else { holy_name };
        days[holy_name].push(HOLY_NAME);

        // Epiphany cycle
        let dom_post_epiph = ch.sunday_after(ch.epiphany()).unwrap();
        for (week, &sunday) in SUNDAYS_AFTER_EPIPHANY.iter().enumerate() {
            let ord = dom_post_epiph + (week * 7);
            if ord >= ch.septuagesima() {
                break;
            }
            days[ord].push(sunday);
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
        days[lent1 + 3].push(Office::feria(FeriaRank::SecondClass, true).with_id("fer-4-qt-quad"));
        days[lent1 + 4].push(Office::feria(FeriaRank::ThirdClass, true));
        days[lent1 + 5].push(Office::feria(FeriaRank::SecondClass, true).with_id("fer-6-qt-quad"));
        days[lent1 + 6].push(Office::feria(FeriaRank::SecondClass, false).with_id("sab-qt-quad"));
        for week in 1..=4 {
            for weekday in 1..=6 {
                days[lent1 + (7 * week) + weekday]
                    .push(Office::feria(FeriaRank::ThirdClass, weekday < 6));
            }
        }

        // Holy week
        let palm_sunday = ch.easter() - 7;
        for weekday in 1..4 {
            days[palm_sunday + weekday].push(Office::feria(FeriaRank::Privileged, true));
        }
        days[palm_sunday + 4]
            .push(Office::feria(FeriaRank::Privileged, true).with_id("in-coena-domini"));
        days[palm_sunday + 5]
            .push(Office::feria(FeriaRank::Privileged, true).with_id("in-parasceve"));
        days[palm_sunday + 6]
            .push(Office::feria(FeriaRank::Privileged, true).with_id("sabbato-sancto"));

        // Easter week
        let inf_oct_pascha = EASTER.day_within_octave().unwrap();
        for weekday in 1..=5 {
            days[ch.easter() + weekday].push(inf_oct_pascha);
        }
        days[ch.easter() + 6].push(Office::WithinOctave {
            feast_details: EASTER.feast_details().unwrap(),
            rank: OctaveRank::FirstOrder,
            has_second_vespers: false,
        });

        // The vigil of the ascension will be added automatically
        let ascension = ch.easter() + 39;
        days[ascension].push(ASCENSION);

        // Pentecost and its octave
        // the vigil will be filled in automatically later
        let pentecost = ch.easter() + 49;
        let inf_oct_pent = PENTECOST.day_within_octave().unwrap();
        for weekday in 1..=5 {
            days[pentecost + weekday].push(inf_oct_pent);
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
        days[dom_3_sept + 3]
            .push(Office::feria(FeriaRank::SecondClass, true).with_id("fer-4-qt-sept"));
        days[dom_3_sept + 5]
            .push(Office::feria(FeriaRank::SecondClass, true).with_id("fer-6-qt-sept"));
        days[dom_3_sept + 6]
            .push(Office::feria(FeriaRank::SecondClass, false).with_id("sab-qt-sept"));

        // End of the Pentecost cycle
        let post_pent_24 = ch.advent1() - 7;
        days[post_pent_24].push(add_matins_id(
            ch,
            Office::sunday("dom-24-post-pent", SundayRank::SecondClass),
            post_pent_24,
        ));
        let post_pent_23 = pentecost + (23 * 7);
        if post_pent_23 < post_pent_24 {
            days[post_pent_23].push(add_matins_id(
                ch,
                Office::sunday("dom-23-post-pent", SundayRank::SecondClass),
                post_pent_23,
            ));
        }

        // Resumed Sundays after Epiphany
        for i in 1..=6 {
            let ord = post_pent_24 - (i * 7);
            if ord <= post_pent_23 {
                break;
            }
            days[ord].push(add_matins_id(ch, SUNDAYS_AFTER_EPIPHANY[6 - i], ord));
        }

        // fill in the rest of the year with ferias / OLOS
        // strictly speaking we don't have to check that the days are open, because the rubrics
        // system will automatically omit common ferias / OLOS in occurrence
        for day in 0..340 {
            let weekday = ch.weekday(day);
            if weekday == Weekday::Sun {
                continue;
            } else if weekday == Weekday::Sat
                && days[day]
                    .iter()
                    .all(|&o| Rubrics1962.admits_our_lady_on_saturday(o))
            {
                days[day].push(Office::OurLadyOnSaturday);
            } else if days[day]
                .iter()
                .all(|&o| Rubrics1962.admits_common_feria(o))
            {
                days[day].push(Office::feria(FeriaRank::Common, true));
            }
        }
    }
}
