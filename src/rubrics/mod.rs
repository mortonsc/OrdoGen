use itertools::chain;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

mod display;
mod rubrics1939;

#[cfg(test)]
mod tests;

pub use rubrics1939::Rubrics1939;

// TODO:
// * The Purification is a feast both of our Lord and our Lady

// convenience functions for comparison chains.
fn true_is_greater(rhs: bool, lhs: bool) -> Ordering {
    match (rhs, lhs) {
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        _ => Ordering::Equal,
    }
}

fn false_is_greater(rhs: bool, lhs: bool) -> Ordering {
    true_is_greater(rhs, lhs).reverse()
}

// listed from lowest-to-highest so the ordering is correct
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeastRank {
    Commemoration,
    Simple,
    Semidouble,
    LesserDouble,
    GreaterDouble,
    DoubleSecondClass,
    DoubleFirstClass,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeastSubRank {
    Secondary,
    Primary,
}

impl Default for FeastSubRank {
    fn default() -> Self {
        FeastSubRank::Primary
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum OctaveRank {
    Simple,
    Common,
    ThirdOrder,
    SecondOrder,
    FirstOrder,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum Person {
    Other,
    Doctor,
    Evangelist,
    Apostle,
    Joseph,
    JohnBaptist,
    Angels,
    OurLady,
    OurLord,
    Trinity,
}

impl Default for Person {
    fn default() -> Self {
        Person::Other
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum SundayRank {
    Common,
    WithinOctave(OctaveRank),
    SecondClass,
    FirstClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeastDetails<'a> {
    pub id: &'a str,
    pub rank: FeastRank,
    #[serde(default)]
    pub sub_rank: FeastSubRank,
    #[serde(default)]
    pub person: Person,
    #[serde(default)]
    pub is_patron_or_titular: bool,
    #[serde(default)]
    pub is_local: bool,
    #[serde(default)]
    pub is_moveable: bool,
    #[serde(default)]
    pub octave: Option<OctaveRank>,
    #[serde(default)]
    pub vigil: Option<VigilRank>,
    #[serde(default)]
    pub is_feriatum: bool,
}

impl<'a> FeastDetails<'a> {
    pub const fn new(id: &'a str, rank: FeastRank) -> Self {
        Self {
            id,
            rank,
            sub_rank: FeastSubRank::Primary,
            person: Person::Other,
            is_patron_or_titular: false,
            is_local: false,
            is_moveable: false,
            octave: None,
            vigil: None,
            is_feriatum: false,
        }
    }
    pub const fn with_person(mut self, person: Person) -> Self {
        self.person = person;
        self
    }
    pub const fn with_sub_rank(mut self, sub_rank: FeastSubRank) -> Self {
        self.sub_rank = sub_rank;
        self
    }
    pub const fn make_patron_or_titular(mut self) -> Self {
        self.is_patron_or_titular = true;
        self
    }
    pub const fn make_local(mut self) -> Self {
        self.is_local = true;
        self
    }
    pub const fn make_moveable(mut self) -> Self {
        self.is_moveable = true;
        self
    }
    pub const fn make_feriatum(mut self) -> Self {
        self.is_feriatum = true;
        self
    }
    pub const fn with_octave(mut self, rank: OctaveRank) -> Self {
        self.octave = Some(rank);
        self
    }
    pub const fn with_vigil(mut self, rank: VigilRank) -> Self {
        self.vigil = Some(rank);
        self
    }
    // this technical meaning of "solemn" is relevant to determining feast precedence
    pub fn is_solemn(self) -> bool {
        self.is_feriatum || self.octave.is_some()
    }
    pub const fn done(self) -> Office<'a> {
        Office::Feast(self)
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeriaRank {
    // Third class vs second class is a distinction only in 1962 rubrics
    // Common ferias are equivalent to 1962 fourth class ferias
    // Privileged ferias are equivalent to 1962 first class ferias
    Common,
    AnticipatedSunday,
    ThirdClassAdvent,
    ThirdClass,
    SecondClass,
    Privileged,
    DoubleFirstClass,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum VigilRank {
    Common, // Third class in the 1962 rubrics
    SecondClass,
    FirstClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Office<'a> {
    Feast(FeastDetails<'a>),
    Sunday {
        id: &'a str,
        matins_id: Option<&'a str>,
        rank: SundayRank,
    },
    Feria {
        id: Option<&'a str>,
        // false for Saturdays
        has_second_vespers: bool,
        // false for days without second vespers and also for certain days which have second
        // vespers normally, but it isn't commemorated if superseded by a feast
        commemorated_at_vespers: bool,
        rank: FeriaRank,
    },
    WithinOctave {
        feast_details: FeastDetails<'a>,
        rank: OctaveRank,
        // the last day within an octave lacks second vespers
        has_second_vespers: bool,
    },
    OctaveDay {
        feast_details: FeastDetails<'a>,
        rank: OctaveRank,
    },
    Vigil {
        feast_details: FeastDetails<'a>,
        rank: VigilRank,
    },
    AllSouls,
    OurLadyOnSaturday,
    // used as a placeholder for an office that doesn't exist
    Empty,
}

impl<'a> Office<'a> {
    pub fn is_feast(self) -> bool {
        matches!(self, Self::Feast(_))
    }
    pub fn is_sunday(self) -> bool {
        matches!(self, Self::Sunday { .. })
    }
    pub fn feast_rank(self) -> Option<FeastRank> {
        if let Self::Feast(FeastDetails { rank, .. }) = self {
            Some(rank)
        } else {
            None
        }
    }
    pub fn is_feria(self) -> bool {
        matches!(self, Self::Feria { .. })
    }
    pub fn is_vigil(self) -> bool {
        matches!(self, Self::Vigil { .. })
    }
    pub fn is_greater_feria(self) -> bool {
        if let Self::Feria { rank, .. } = self {
            rank > FeriaRank::Common
        } else {
            false
        }
    }
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
    pub fn feast_details(self) -> Option<FeastDetails<'a>> {
        if let Self::Feast(fd) = self {
            Some(fd)
        } else {
            None
        }
    }
    pub fn assoc_feast_details(self) -> Option<FeastDetails<'a>> {
        match self {
            Self::Feast(fd)
            | Self::WithinOctave {
                feast_details: fd, ..
            }
            | Self::OctaveDay {
                feast_details: fd, ..
            }
            | Self::Vigil {
                feast_details: fd, ..
            } => Some(fd),
            _ => None,
        }
    }
    pub fn person(self) -> Option<Person> {
        match self {
            Self::OurLadyOnSaturday => Some(Person::OurLady),
            _ => Some(self.assoc_feast_details()?.person),
        }
    }
    pub fn is_of_same_feast(self, other: Self) -> bool {
        if let (Some(fd1), Some(fd2)) = (self.assoc_feast_details(), other.assoc_feast_details()) {
            fd1.id == fd2.id
        } else {
            false
        }
    }
    // TODO: this doesn't fully deal with separate feasts of the same person
    // if the person doesn't have an explicit Person variant
    // TODO: in particular there probably needs to be a special case for the commemoration of St
    // Paul, which isn't commemorated at 2V of the Sts Peter and Paul the preceding day
    pub fn is_of_same_subject(self, other: Self) -> bool {
        // two days in the octave of the same feast are obviously of the same person
        // (and we can't always tell this by just looking at the Person field)
        if self.is_of_same_feast(other) {
            return true;
        }
        if let (Some(p1), Some(p2)) = (self.person(), other.person()) {
            // the Persons lower-ranked than Joseph are categories, not specific persons
            // while different feasts of the Lord are considered to have different subjects
            // as they deal with different mysteries
            p1 >= Person::Joseph && p1 != Person::OurLord && p1 == p2
        } else {
            false
        }
    }
    pub fn is_feast_of_the_lord(self) -> bool {
        matches!(self.person(), Some(Person::OurLord))
    }
    pub fn named_feria(id: &'a str, rank: FeriaRank, has_second_vespers: bool) -> Self {
        Self::Feria {
            id: Some(id),
            rank,
            has_second_vespers,
            commemorated_at_vespers: has_second_vespers,
        }
    }
    pub const fn feast(id: &'a str, rank: FeastRank) -> FeastDetails<'a> {
        FeastDetails::new(id, rank)
    }
    pub const fn unnamed_feria(rank: FeriaRank, has_second_vespers: bool) -> Self {
        Self::Feria {
            id: None,
            rank,
            has_second_vespers,
            commemorated_at_vespers: has_second_vespers,
        }
    }
    pub const fn common_feria() -> Self {
        Self::Feria {
            id: None,
            rank: FeriaRank::Common,
            has_second_vespers: true,
            commemorated_at_vespers: true,
        }
    }
    pub fn vigil(self) -> Option<Self> {
        let feast_details = self.feast_details()?;
        let rank = feast_details.vigil?;
        Some(Self::Vigil {
            feast_details,
            rank,
        })
    }
    pub fn day_within_octave(self) -> Option<Self> {
        let feast_details = self.feast_details()?;
        let rank = feast_details.octave?;
        if rank == OctaveRank::Simple {
            None
        } else {
            Some(Self::WithinOctave {
                feast_details,
                rank,
                has_second_vespers: true,
            })
        }
    }
    pub fn octave_day(self) -> Option<Self> {
        let feast_details = self.feast_details()?;
        let rank = feast_details.octave?;
        Some(Self::OctaveDay {
            feast_details,
            rank,
        })
    }
    pub fn with_matins_id(self, new_matins_id: &'a str) -> Self {
        if let Self::Sunday { id, rank, .. } = self {
            Self::Sunday {
                id,
                matins_id: Some(new_matins_id),
                rank,
            }
        } else {
            self
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfficeIs {
    DePrimo,
    DeSecundo,
}

impl OfficeIs {
    fn winner_first<K: Copy>(self, first: K, second: K) -> (K, K) {
        match self {
            Self::DePrimo => (first, second),
            Self::DeSecundo => (second, first),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoserIs {
    Translated,
    Commemorated,
    Omitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OccurrenceOutcome {
    office_to_celebrate: OfficeIs,
    loser_is: LoserIs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hour {
    FirstVespers,
    Lauds,
    SecondVespers,
}

// note that when Vespers is split at the cap,
// we consider the Vespers of the following day to be the "winner"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VespersIs {
    DePraec,
    DeSeq,
    ACapSeq,
}

impl VespersIs {
    fn winner_first<K: Copy>(self, praec: K, seq: K) -> (K, K) {
        match self {
            Self::DePraec => (praec, seq),
            Self::DeSeq | Self::ACapSeq => (seq, praec),
        }
    }
    pub fn applied_to<'a>(&self, praec: Office<'a>, seq: Office<'a>) -> Vespers<'a> {
        match self {
            VespersIs::DePraec => Vespers::SecondVespers(praec),
            VespersIs::DeSeq => Vespers::FirstVespers(seq),
            VespersIs::ACapSeq => Vespers::SplitAtCap(praec, seq),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConcurrenceOutcome {
    office_to_celebrate: VespersIs,
    has_comm: bool,
}

impl ConcurrenceOutcome {
    pub fn seq_wins(&self) -> bool {
        matches!(
            self.office_to_celebrate,
            VespersIs::DeSeq | VespersIs::ACapSeq
        )
    }
    pub fn praec_wins(&self) -> bool {
        !self.seq_wins()
    }
}

#[derive(Debug, Clone)]
pub struct OrderedOffice<'a> {
    pub office_of_day: Office<'a>,
    pub to_commemorate: Vec<Office<'a>>,
}

impl<'a> OrderedOffice<'a> {
    pub fn of_only(off: Office<'a>) -> Self {
        Self {
            office_of_day: off,
            to_commemorate: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vespers<'a> {
    FirstVespers(Office<'a>),
    SecondVespers(Office<'a>),
    SplitAtCap(Office<'a>, Office<'a>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VespersComm<'a> {
    FirstVespers(Office<'a>),
    SecondVespers(Office<'a>),
}

impl<'a> VespersComm<'a> {
    pub fn office(self) -> Office<'a> {
        match self {
            Self::FirstVespers(off) => off,
            Self::SecondVespers(off) => off,
        }
    }
}

#[derive(Debug)]
pub struct OrderedVespers<'a> {
    pub vespers: Vespers<'a>,
    pub to_commemorate: Vec<VespersComm<'a>>,
}

pub trait RubricsSystem {
    fn has_first_vespers(&self, off: Office, is_sunday: bool) -> bool;
    fn has_second_vespers(&self, off: Office) -> bool;
    fn admits_translated_feast(&self, off: Office) -> bool;
    fn occurrence_outcome(&self, occ1: Office, occ2: Office) -> OccurrenceOutcome;
    // assumes both praec and seq are the office of their respective days
    fn concurrence_outcome(
        &self,
        praec: Office,
        seq: Office,
        seq_is_sunday: bool,
    ) -> ConcurrenceOutcome;
    fn compare_precedence_occ(&self, occ1: Office, occ2: Office) -> Ordering;
    fn compare_precedence_conc(&self, praec: Office, seq: Office) -> VespersIs;
    fn compare_commemoration_order(&self, comm1: Office, comm2: Office) -> Ordering;
    // assuming the office of the day is winner and loser occurs on the same day,
    // returns whether loser should be commemorated at the given hour
    fn occ_admits_commemoration(&self, winner: Office, loser: Office, hour: Hour) -> bool;
    // assuming Vespers is of praec, returns true if seq is to be commemorated
    fn praec_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool;
    // assuming Vespers is of seq, returns true if praec is to be commemorated
    fn seq_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool;
    // returns true if vigils that fall on Sunday should be anticipated
    // doesn't apply to the vigils of Christmas or Epiphany
    fn anticipate_vigils(&self) -> bool;
    fn order_office<'a>(&self, occs: &[Office<'a>]) -> (OrderedOffice<'a>, Vec<Office<'a>>) {
        let mut to_commemorate: Vec<Office<'a>> = Vec::new();
        let mut to_translate: Vec<Office<'a>> = Vec::new();
        if occs.is_empty() {
            return (
                OrderedOffice {
                    office_of_day: Office::Empty,
                    to_commemorate,
                },
                to_translate,
            );
        }
        let mut occs = occs.to_vec();
        occs.sort_by(|&occ1, &occ2| self.compare_precedence_occ(occ1, occ2));
        let office_of_day: Office = occs.pop().unwrap();
        // reverse because we want to deal with higher-ranked things first
        for &occ in occs.iter().rev() {
            let outcome = self.occurrence_outcome(office_of_day, occ);
            assert_eq!(outcome.office_to_celebrate, OfficeIs::DePrimo);
            if outcome.loser_is == LoserIs::Translated {
                to_translate.push(occ);
            } else if outcome.loser_is == LoserIs::Commemorated
                && to_commemorate.iter().all(|c| !c.is_of_same_subject(occ))
            {
                to_commemorate.push(occ);
            }
        }
        to_commemorate.sort_by(|&c1, &c2| self.compare_commemoration_order(c1, c2));
        (
            OrderedOffice {
                office_of_day,
                to_commemorate,
            },
            to_translate,
        )
    }
    fn order_vespers<'a>(
        &self,
        praec_day: &OrderedOffice<'a>,
        seq_day: &OrderedOffice<'a>,
        seq_is_sunday: bool,
    ) -> OrderedVespers<'a> {
        let praec = if self.has_second_vespers(praec_day.office_of_day) {
            praec_day.office_of_day
        } else {
            // this covers the case where praec has no 2V and seq has no 1V,
            // which happens in pre-55 rubrics when a simple feast is followed by a feria
            // (the rubrics actually analyze this case as the following feria gaining 1st vespers)
            Office::Feria {
                id: None,
                rank: FeriaRank::Common,
                has_second_vespers: true,
                commemorated_at_vespers: false,
            }
        };
        let seq = if self.has_first_vespers(seq_day.office_of_day, seq_is_sunday) {
            seq_day.office_of_day
        } else {
            Office::Empty
        };
        let mut to_commemorate: Vec<VespersComm<'a>> = Vec::new();
        let co = self.concurrence_outcome(praec, seq, seq_is_sunday);
        let vespers = co.office_to_celebrate.applied_to(praec, seq);
        if co.praec_wins() && co.has_comm {
            to_commemorate.push(VespersComm::FirstVespers(seq));
        } else if co.seq_wins() && co.has_comm {
            to_commemorate.push(VespersComm::SecondVespers(praec));
        }
        let comms_from_praec: Vec<VespersComm<'a>> = praec_day
            .to_commemorate
            .iter()
            .filter(|&&off| {
                if co.praec_wins() {
                    self.occ_admits_commemoration(praec, off, Hour::SecondVespers)
                } else {
                    self.seq_admits_commemoration(off, seq, seq_is_sunday)
                }
            })
            .map(|&off| VespersComm::SecondVespers(off))
            .collect();
        let comms_from_seq: Vec<VespersComm<'a>> = seq_day
            .to_commemorate
            .iter()
            .filter(|&&off| {
                if co.praec_wins() {
                    self.praec_admits_commemoration(praec, off, seq_is_sunday)
                } else {
                    self.occ_admits_commemoration(seq, off, Hour::FirstVespers)
                }
            })
            .filter(|&&off| {
                // (pre-55) when two consecutive days within octaves are commemorated,
                // the commemoration at Vespers is 2V of the first day
                // this also removes 1V of the first day in an octave (though we would do that
                // later anyways)
                // in 1962 days in octaves don't have 1V so this does nothing
                if let Office::WithinOctave { .. } = off {
                    !comms_from_praec
                        .iter()
                        .any(|c| c.office().is_of_same_feast(off))
                } else {
                    true
                }
            })
            .filter(|&&off| {
                // this takes care of one specific case, where a day within an octave is followed
                // by the octave day, but the octave day is superseded by a feast
                // in which case (in pre-55 rubrics) the commemoration at 1V of the feast is taken
                // from 2V of the preceding day within the octave, not 1V of the octave day
                // the 1939 breviary has an explicit rubric about this before the octave day of
                // Corpus Christi
                !off.is_of_same_feast(praec)
            })
            .map(|&off| VespersComm::FirstVespers(off))
            .collect();

        to_commemorate.extend(chain(comms_from_praec, comms_from_seq));
        // TODO: is there a specific ordering that should hold between 1V and 2V commemorations?
        to_commemorate
            .sort_by(|&c1, &c2| self.compare_commemoration_order(c1.office(), c2.office()));
        // remove commemorations of the same subject
        // assumes that the commemoration that comes later is the one that should be removed
        // in practice this doesn't come up much because we've already covered all the cases with
        // octaves above
        let mut to_commemorate_final: Vec<VespersComm<'a>> = Vec::new();
        for comm in to_commemorate {
            if !to_commemorate_final
                .iter()
                .any(|c| c.office().is_of_same_subject(comm.office()))
            {
                to_commemorate_final.push(comm);
            }
        }

        OrderedVespers {
            vespers,
            to_commemorate: to_commemorate_final,
        }
    }
}
