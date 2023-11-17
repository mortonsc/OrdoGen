use super::*;

const CORPUS_CHRISTI: Office = Office::Feast(FeastDetails {
    id: "corpus-christi",
    rank: FeastRank::DoubleFirstClass,
    sub_rank: FeastSubRank::Primary,
    person: Person::OurLord,
    is_patron_or_titular: false,
    is_local: false,
    is_moveable: true,
});

const THOMAS_AP: Office = Office::Feast(FeastDetails {
    id: "thomas-ap",
    rank: FeastRank::DoubleFirstClass,
    sub_rank: FeastSubRank::Primary,
    person: Person::Apostle,
    is_patron_or_titular: false,
    is_local: false,
    is_moveable: false,
});

const EXALT_CRUCIS: Office = Office::Feast(FeastDetails {
    id: "exalt-crucis",
    rank: FeastRank::GreaterDouble,
    sub_rank: FeastSubRank::Primary,
    person: Person::Other,
    is_patron_or_titular: false,
    is_local: false,
    is_moveable: false,
});

const INVENT_STEPHEN: Office = Office::Feast(FeastDetails {
    id: "invent-stephan",
    rank: FeastRank::Semidouble,
    sub_rank: FeastSubRank::Secondary,
    person: Person::Other,
    is_patron_or_titular: false,
    is_local: false,
    is_moveable: false,
});

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

const ASSUMPTION_DET: FeastDetails = FeastDetails {
    id: "in-assumptione-bmv",
    rank: FeastRank::DoubleFirstClass,
    sub_rank: FeastSubRank::Primary,
    person: Person::OurLady,
    is_patron_or_titular: false,
    is_local: false,
    is_moveable: false,
};

const ASSUMPTION: Office = Office::Feast(ASSUMPTION_DET);
const VIGIL_ASSUMPTION: Office = Office::Vigil {
    id: "in-vig-assumptionis-bmv",
    feast_details: ASSUMPTION_DET,
    rank: VigilRank::Common,
};

const IN_OCT_ASSUMPTION: Office = Office::WithinOctave {
    id: "inf-oct-assumptionis-bmv",
    feast_details: ASSUMPTION_DET,
    rank: OctaveRank::Common,
    has_second_vespers: true,
};

const OCT_DAY_ASSUMPTION: Office = Office::OctaveDay {
    id: "in-oct-assumptions-bmv",
    feast_details: ASSUMPTION_DET,
    rank: OctaveRank::Common,
};

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

const SIMPLE_FEAST: Office = Office::Feast(FeastDetails {
    id: "__simple__",
    rank: FeastRank::Simple,
    sub_rank: FeastSubRank::Primary,
    person: Person::Other,
    is_patron_or_titular: false,
    is_local: false,
    is_moveable: false,
});

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
        rubrics.occurrence_outcome(CORPUS_CHRISTI, EMBER_WEDNESDAY, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DePrimo,
            loser_is: LoserIs::Commemorated
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(VIGIL_ASSUMPTION, ADVENT_FERIA, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DeSecundo,
            loser_is: LoserIs::Omitted,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(DOM_1_QUAD, THOMAS_AP, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DePrimo,
            loser_is: LoserIs::Translated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(DOM_15_POST_PENT, INVENT_STEPHEN, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DePrimo,
            loser_is: LoserIs::Commemorated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(EXALT_CRUCIS, OCT_DAY_ASSUMPTION, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DeSecundo,
            loser_is: LoserIs::Commemorated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(DOM_SEPTUAGESIMA, ASSUMPTION, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DeSecundo,
            loser_is: LoserIs::Commemorated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(IN_OCT_ASSUMPTION, OUR_LADY_ON_SATURDAY, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DePrimo,
            loser_is: LoserIs::Omitted,
        }
    );
}

#[test]
fn concurrence() {
    let rubrics = Rubrics1939;
    assert_eq!(
        rubrics.concurrence_outcome(IN_OCT_ASSUMPTION, IN_OCT_ASSUMPTION, false),
        ConcurrenceOutcome {
            office_to_celebrate: VespersIs::DePraec,
            has_comm: false,
        }
    )
}

#[test]
fn consecutive_days_in_octave() {
    let rubrics = Rubrics1939;

    let praec_day = OrderedOffice::of_only(IN_OCT_ASSUMPTION);
    let seq_day = praec_day.clone();
    let ov = rubrics.order_vespers(praec_day, seq_day, false);
    assert_eq!(ov.vespers, Vespers::SecondVespers(IN_OCT_ASSUMPTION));
    assert!(ov.to_commemorate.is_empty());
}

#[test]
fn feria_with_greater_feria_comm_simple() {
    let rubrics = Rubrics1939;

    let praec_day = OrderedOffice::of_only(Office::Empty);
    let seq_day = rubrics.order_office(vec![EMBER_WEDNESDAY, SIMPLE_FEAST], true);
    assert_eq!(seq_day.office_of_day, EMBER_WEDNESDAY);
    assert_eq!(seq_day.to_commemorate[0], SIMPLE_FEAST);
    let ov = rubrics.order_vespers(praec_day, seq_day, false);
    assert_eq!(ov.vespers, Vespers::SecondVespers(Office::Empty));
    assert_eq!(ov.to_commemorate[0], SIMPLE_FEAST);
}
