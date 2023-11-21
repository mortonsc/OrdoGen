use super::*;
use crate::calendar::calendar1939::temporal_cycle::SUNDAYS_OF_ADVENT;
use crate::ordo::*;
use crate::rubrics::*;

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

impl Calendar1939 {
    // TODO: this doesn't allow for local calendar variations
    pub fn order_christmastide_h<'a>(
        &self,
        year: i32,
        lauds_dec23: OrderedOffice<'a>,
    ) -> Vec<OrdoEntry<'a>> {
        let ch = CalendarHelper::new(year);
        // translate the ordinal of Dec 23 to 0
        let idx = |ord: usize| ord - ch.ordinal0(12, 23);
        let ord = |idx: usize| idx + ch.ordinal0(12, 23);

        let sunday_idx = ch.sunday_after(ch.christmas).map(|ord| idx(ord));
        // TODO: account for the effect of local feasts on the placement of the Sunday
        let sunday_idx = match sunday_idx {
            None | Some(2..=5) => 7,
            Some(i) => i,
        };

        let vig_nat = NATIVITY.vigil().unwrap();
        let vigil_is_sunday = ch.is_sunday(ord(1));
        let inf_oct_nat = NATIVITY.day_within_octave().unwrap();

        let mut offices: Vec<Vec<Office<'a>>> = vec![Vec::new(); 9];
        offices[0].push(lauds_dec23.office_of_day);
        offices[1].push(vig_nat);
        if vigil_is_sunday {
            offices[1].push(SUNDAYS_OF_ADVENT[3]);
        }
        offices[2].push(NATIVITY);
        for i in 3..offices.len() {
            offices[i].push(inf_oct_nat);
        }
        offices[3].push(ST_STEPHEN);
        offices[4].push(ST_JOHN_EV);
        offices[5].push(HOLY_INNOCENTS);
        offices[6].push(ST_THOMAS_BECKET);
        offices[8].push(ST_SILVESTER);
        offices[sunday_idx].push(SUNDAY_WITHIN_OCT_NAT);

        // lauds obeys the usual rubrics
        let mut all_lauds: Vec<OrderedOffice<'a>> = Vec::new();
        all_lauds.push(lauds_dec23);
        for i in 1..offices.len() {
            let (lauds, no_translations) = Rubrics1939.order_office(&(offices[i])[..]);
            assert!(no_translations.is_empty());
            all_lauds.push(lauds);
        }

        let mut all_vespers: Vec<OrderedVespers<'a>> = Vec::new();
        // Dec 23 (idx 0)
        all_vespers.push(Rubrics1939.order_vespers(
            &all_lauds[0],
            &all_lauds[1],
            ch.is_sunday(ord(1)),
        ));
        // Dec 24 (idx 1)
        all_vespers.push(OrderedVespers::of(Vespers::FirstVespers(NATIVITY)));
        // Dec 25 (idx 2)
        all_vespers.push(
            OrderedVespers::of(Vespers::SecondVespers(NATIVITY))
                .with_comm(VespersComm::FirstVespers(ST_STEPHEN)),
        );
        // Dec 26 (idx 3)
        all_vespers.push(
            OrderedVespers::of(Vespers::SecondVespers(ST_STEPHEN))
                .with_comm(VespersComm::FirstVespers(ST_JOHN_EV))
                .with_comm(VespersComm::SecondVespers(inf_oct_nat)),
        );
        // Dec 27 (idx 4)
        all_vespers.push(
            OrderedVespers::of(Vespers::SecondVespers(ST_JOHN_EV))
                .with_comm(VespersComm::FirstVespers(HOLY_INNOCENTS))
                .with_comm(VespersComm::SecondVespers(inf_oct_nat)),
        );
        // Dec 28 - 31 (idx 5 - 7)
        for idx in 5..=7 {
            // doesn't actually matter for anything
            let seq_is_sunday = ch.is_sunday(ord(idx + 1));
            all_vespers.push(Rubrics1939.order_vespers(
                &all_lauds[idx],
                &all_lauds[idx + 1],
                seq_is_sunday,
            ));
        }
        // Dec 31 (idx 8)
        all_vespers.push(OrderedVespers::of(Vespers::FirstVespers(CIRCUMCISION)));

        all_lauds
            .into_iter()
            .zip(all_vespers.into_iter())
            .map(|(lauds, vespers)| OrdoEntry { lauds, vespers })
            .collect()
    }
}
