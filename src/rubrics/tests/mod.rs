use super::*;

const CORPUS_CHRISTI: Office = Office::Feast(FeastDetails {
    id: "corpus-christi",
    rank: FeastRank::DoubleFirstClass,
    sub_rank: FeastSubRank::Primary,
    person: Person::OurLord,
    is_patron_or_titular: false,
    is_privileged: true,
    is_local: false,
    is_moveable: true,
    octave: Some(OctaveType::Common), // TODO: haven't fleshed out octave types yet
});

const THOMAS_AP: Office = Office::Feast(FeastDetails {
    id: "thomas-ap",
    rank: FeastRank::DoubleFirstClass,
    sub_rank: FeastSubRank::Primary,
    person: Person::Apostle,
    is_patron_or_titular: false,
    is_privileged: false,
    is_local: false,
    is_moveable: false,
    octave: None,
});

const EXALT_CRUCIS: Office = Office::Feast(FeastDetails {
    id: "exalt-crucis",
    rank: FeastRank::GreaterDouble,
    sub_rank: FeastSubRank::Primary,
    person: Person::Other,
    is_patron_or_titular: false,
    is_privileged: false,
    is_local: false,
    is_moveable: false,
    octave: None,
});

const INVENT_STEPHEN: Office = Office::Feast(FeastDetails {
    id: "invent-stephan",
    rank: FeastRank::Semidouble,
    sub_rank: FeastSubRank::Secondary,
    person: Person::Other,
    is_patron_or_titular: false,
    is_privileged: false,
    is_local: false,
    is_moveable: false,
    octave: None,
});

const DOM_15_POST_PENT: Office = Office::Sunday {
    id: "dom-15-post-pent",
    rank: SundayRank::Common,
};

const DOM_1_QUAD: Office = Office::Sunday {
    id: "dom-1-quad",
    rank: SundayRank::FirstClass,
};

const DOM_SEPTUAGESIMA: Office = Office::Sunday {
    id: "dom-septuagesima",
    rank: SundayRank::SecondClass,
};

const ASSUMPTION_DET: FeastDetails = FeastDetails {
    id: "assumption",
    rank: FeastRank::DoubleFirstClass,
    sub_rank: FeastSubRank::Primary,
    person: Person::OurLady,
    is_patron_or_titular: false,
    is_privileged: false,
    is_local: false,
    is_moveable: false,
    octave: Some(OctaveType::Common), // TODO: haven't fleshed out octave types yet
};

const ASSUMPTION: Office = Office::Feast(ASSUMPTION_DET);
const VIGIL_ASSUMPTION: Office = Office::Vigil(ASSUMPTION_DET);
const IN_OCT_ASSUMPTION: Office = Office::WithinOctave(ASSUMPTION_DET);
const OCT_DAY_ASSUMPTION: Office = Office::OctaveDay(ASSUMPTION_DET);

const ADVENT_FERIA: Office = Office::GreaterFeria {
    id: None,
    is_privileged: false,
    commemorated_at_vespers: true,
};

const EMBER_WEDNESDAY: Office = Office::GreaterFeria {
    id: Some("fer-iv-quat-temp-sept"),
    is_privileged: false,
    commemorated_at_vespers: false,
};

const OUR_LADY_ON_SATURDAY: Office = Office::OurLadyOnSaturday;

#[test]
fn cmp_person() {
    assert!(Person::OurLady < Person::OurLord);
    assert!(Person::JohnBaptist > Person::Joseph);
    assert!(Person::Other < Person::Trinity);
}

#[test]
fn occurrence() {
    let rubrics = Rubrics1910;
    assert_eq!(
        rubrics.occurrence_outcome(CORPUS_CHRISTI, EMBER_WEDNESDAY, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DePrimo,
            loser_outcome: LoserIs::Commemorated
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(VIGIL_ASSUMPTION, ADVENT_FERIA, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DeSecundo,
            loser_outcome: LoserIs::Omitted,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(DOM_1_QUAD, THOMAS_AP, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DePrimo,
            loser_outcome: LoserIs::Translated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(DOM_15_POST_PENT, INVENT_STEPHEN, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DePrimo,
            loser_outcome: LoserIs::Commemorated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(EXALT_CRUCIS, OCT_DAY_ASSUMPTION, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DeSecundo,
            loser_outcome: LoserIs::Translated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(DOM_SEPTUAGESIMA, ASSUMPTION, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DeSecundo,
            loser_outcome: LoserIs::Commemorated,
        }
    );
    assert_eq!(
        rubrics.occurrence_outcome(IN_OCT_ASSUMPTION, OUR_LADY_ON_SATURDAY, false),
        OccurrenceOutcome {
            office_to_celebrate: OfficeIs::DePrimo,
            loser_outcome: LoserIs::Omitted,
        }
    );
}
