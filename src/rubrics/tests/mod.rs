use super::*;

const CORPUS_CHRISTI: Office = Office::feast("ssmi-corporis-christi", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLord)
    .make_moveable()
    .make_feriatum()
    .with_octave(OctaveRank::SecondOrder)
    .done();

const THOMAS_AP: Office = Office::feast("s-thomas-ap", FeastRank::DoubleSecondClass)
    .with_person(Person::Apostle)
    .make_feriatum()
    .done();

const EXALT_CRUCIS: Office =
    Office::feast("in-exaltatione-s-crucis", FeastRank::GreaterDouble).done();

const INVENT_STEPHEN: Office =
    Office::feast("inventio-s-stephani-protomartyris", FeastRank::Semidouble)
        .make_secondary()
        .done();

const DOM_15_POST_PENT: Office = Office::Sunday {
    id: "dom-15-post-pent",
    matins_id: None,
    rank: SundayRank::Common,
};

const DOM_1_QUAD: Office = Office::Sunday {
    id: "dom-1-quad",
    matins_id: None,
    rank: SundayRank::FirstClass,
};

const DOM_SEPTUAGESIMA: Office = Office::Sunday {
    id: "dom-septuagesima",
    matins_id: None,
    rank: SundayRank::SecondClass,
};

const ASSUMPTION: Office = Office::feast("in-assumptione-bmv", FeastRank::DoubleFirstClass)
    .with_person(Person::OurLady)
    .with_octave(OctaveRank::Common)
    .with_vigil(VigilRank::Common)
    .make_feriatum()
    .done();

const ADVENT_FERIA: Office = Office::Feria {
    id: None,
    rank: FeriaRank::ThirdClassAdvent,
    has_second_vespers: true,
    commemorated_at_vespers: true,
};

const EMBER_WEDNESDAY: Office = Office::Feria {
    id: Some("fer-iv-quat-temp-sept"),
    rank: FeriaRank::SecondClass,
    has_second_vespers: true,
    commemorated_at_vespers: false,
};

const SIMPLE_FEAST: Office = Office::feast("_simple_", FeastRank::Simple).done();

const OUR_LADY_ON_SATURDAY: Office = Office::OurLadyOnSaturday;

#[test]
fn cmp_person() {
    assert!(Person::OurLady < Person::OurLord);
    assert!(Person::JohnBaptist > Person::Joseph);
    assert!(Person::Other < Person::Trinity);
}

#[test]
fn occurrence() {
    let rubrics = Rubrics1939;
    assert_eq!(
        rubrics.occurrence_outcome(CORPUS_CHRISTI, EMBER_WEDNESDAY),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::OfTheFirst,
            loser_is: LoserIs::Commemorated
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(ASSUMPTION.vigil().unwrap(), ADVENT_FERIA),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::OfTheSecond,
            loser_is: LoserIs::Omitted,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(DOM_1_QUAD, THOMAS_AP),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::OfTheFirst,
            loser_is: LoserIs::Translated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(DOM_15_POST_PENT, INVENT_STEPHEN),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::OfTheFirst,
            loser_is: LoserIs::Commemorated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(EXALT_CRUCIS, ASSUMPTION.octave_day().unwrap()),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::OfTheSecond,
            loser_is: LoserIs::Commemorated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(DOM_SEPTUAGESIMA, ASSUMPTION),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::OfTheSecond,
            loser_is: LoserIs::Commemorated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(
            ASSUMPTION.day_within_octave().unwrap(),
            OUR_LADY_ON_SATURDAY,
        ),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::OfTheFirst,
            loser_is: LoserIs::Omitted,
        }
    );
}

#[test]
fn concurrence() {
    let rubrics = Rubrics1939;
    let inf_oct_assump = ASSUMPTION.day_within_octave().unwrap();
    assert_eq!(
        rubrics.concurrence_outcome(inf_oct_assump, inf_oct_assump, false),
        ConcurrenceOutcome {
            office_to_celebrate: VespersIs::OfThePreceding,
            has_comm: false,
        }
    )
}

#[test]
fn consecutive_days_in_octave() {
    let rubrics = Rubrics1939;

    let inf_oct_assump = ASSUMPTION.day_within_octave().unwrap();
    let praec_day = OrderedLauds::of(inf_oct_assump);
    let seq_day = praec_day.clone();
    let ov = rubrics.order_vespers(&praec_day, &seq_day, false);
    assert_eq!(ov.vespers, Vespers::SecondVespers(inf_oct_assump));
    assert!(ov.to_commemorate.is_empty());
}

#[test]
fn feria_with_greater_feria_comm_simple() {
    let rubrics = Rubrics1939;

    let praec_day = OrderedLauds::of(Office::Empty);
    let offs = vec![EMBER_WEDNESDAY, SIMPLE_FEAST];
    let (seq_day, _) = rubrics.order_lauds(&offs[..]);
    assert_eq!(seq_day.office_of_day, EMBER_WEDNESDAY);
    assert_eq!(seq_day.to_commemorate[0], SIMPLE_FEAST);
    let ov = rubrics.order_vespers(&praec_day, &seq_day, false);
    assert_eq!(ov.vespers, Vespers::SecondVespers(Office::Empty));
    assert_eq!(
        ov.to_commemorate[0],
        VespersComm::FirstVespers(SIMPLE_FEAST)
    );
}
