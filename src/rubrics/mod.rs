use itertools::chain;
use serde::Deserialize;
use std::cmp::Ordering;

mod display;
mod rubrics1910;
mod rubrics1939;

#[cfg(test)]
mod tests;

pub use rubrics1910::Rubrics1910;
pub use rubrics1939::Rubrics1939;

// TODO:
// * Anticipating vigils that fall on Sunday
// * Celebrating omitted Sundays on Saturday
// * Sundays within octaves have the same rank as the octave
// * All Souls Day

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

// listed from lowest-to-highest so the #derive works
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Deserialize)]
pub enum FeastRank {
    Commemoration,
    Simple,
    Semidouble,
    LesserDouble,
    GreaterDouble,
    DoubleSecondClass,
    DoubleFirstClass,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Deserialize)]
pub enum FeastSubRank {
    Secondary,
    Primary,
}

impl Default for FeastSubRank {
    fn default() -> Self {
        FeastSubRank::Primary
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Deserialize)]
pub enum OctaveRank {
    Simple,
    Common,
    ThirdOrder,
    SecondOrder,
    FirstOrder,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Deserialize)]
pub enum Person {
    Other,
    Doctor,
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

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum SundayRank {
    Common,
    SecondClass,
    FirstClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
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
        }
    }
    pub fn with_person(mut self, person: Person) -> Self {
        self.person = person;
        self
    }
    pub fn with_sub_rank(mut self, sub_rank: FeastSubRank) -> Self {
        self.sub_rank = sub_rank;
        self
    }
    pub fn make_patron_or_titular(mut self) -> Self {
        self.is_patron_or_titular = true;
        self
    }
    pub fn make_local(mut self) -> Self {
        self.is_local = true;
        self
    }
    pub fn make_moveable(mut self) -> Self {
        self.is_moveable = true;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum FeriaRank {
    // Third class vs second class is a distinction only in 1962 rubrics
    // Common ferias are equivalent to 1962 fourth class ferias
    // Privileged ferias are equivalent to 1962 first class ferias
    Common,
    ThirdClassAdvent,
    ThirdClass,
    SecondClass,
    Privileged,
    TriduumSacrum,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum VigilRank {
    Common, // Third class in the 1962 rubrics
    SecondClass,
    FirstClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        id: &'a str,
        feast_details: FeastDetails<'a>,
        rank: OctaveRank,
        // the last day within an octave lacks second vespers
        has_second_vespers: bool,
    },
    OctaveDay {
        id: &'a str,
        feast_details: FeastDetails<'a>,
        rank: OctaveRank,
    },
    Vigil {
        id: &'a str,
        feast_details: FeastDetails<'a>,
        rank: VigilRank,
    },
    OurLadyOnSaturday,
    // used both for ordinary ferias per annum and as a placeholder for an office that doesn't exist
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfficeCategory {
    Feast,
    Sunday,
    Feria,
    WithinOctave,
    OctaveDay,
    Vigil,
    OurLadyOnSaturday,
    Empty,
}

impl<'a> Office<'a> {
    pub fn category(self) -> OfficeCategory {
        match self {
            Self::Feast(_) => OfficeCategory::Feast,
            Self::Sunday { .. } => OfficeCategory::Sunday,
            Self::Feria { .. } => OfficeCategory::Feria,
            Self::WithinOctave { .. } => OfficeCategory::WithinOctave,
            Self::OctaveDay { .. } => OfficeCategory::OctaveDay,
            Self::Vigil { .. } => OfficeCategory::Vigil,
            Self::OurLadyOnSaturday => OfficeCategory::OurLadyOnSaturday,
            Self::Empty => OfficeCategory::Empty,
        }
    }
    pub fn is_feast(self) -> bool {
        self.category() == OfficeCategory::Feast
    }
    pub fn is_sunday(self) -> bool {
        self.category() == OfficeCategory::Sunday
    }
    pub fn feast_rank(self) -> Option<FeastRank> {
        if let Self::Feast(FeastDetails { rank, .. }) = self {
            Some(rank)
        } else {
            None
        }
    }
    pub fn is_ferial(self) -> bool {
        matches!(
            self.category(),
            OfficeCategory::Feria | OfficeCategory::Empty
        )
    }
    pub fn is_vigil(self) -> bool {
        self.category() == OfficeCategory::Vigil
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
    // TODO: this doesn't fully deal with separate feasts of the same person
    // if the person doesn't have an explicit Person variant
    // in practice though that doesn't come up
    pub fn is_of_same_person(self, other: Self) -> bool {
        // two days in the octave of the same feast are obviously of the same person
        // (and we can't always tell this by just looking at the Person field)
        if let (Some(fd1), Some(fd2)) = (self.assoc_feast_details(), other.assoc_feast_details()) {
            if fd1.id == fd2.id {
                return true;
            }
        }
        if let (Some(p1), Some(p2)) = (self.person(), other.person()) {
            // the Persons lower-ranked than Joseph are categories, not specific persons
            p1 >= Person::Joseph && p1 == p2
        } else {
            false
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
    pub to_translate: Vec<Office<'a>>,
}

impl<'a> OrderedOffice<'a> {
    pub fn of_only(off: Office<'a>) -> Self {
        Self {
            office_of_day: off,
            to_commemorate: Vec::new(),
            to_translate: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vespers<'a> {
    FirstVespers(Office<'a>),
    SecondVespers(Office<'a>),
    SplitAtCap(Office<'a>, Office<'a>),
}

#[derive(Debug)]
pub struct OrderedVespers<'a> {
    pub vespers: Vespers<'a>,
    // TODO: need to mark whether each of these is first or second vespers
    pub to_commemorate: Vec<Office<'a>>,
}

pub trait RubricsSystem {
    fn has_first_vespers(&self, off: Office, is_sunday: bool) -> bool;
    fn has_second_vespers(&self, off: Office) -> bool;
    fn admits_translated_feast(&self, off: Office) -> bool;
    fn occurrence_outcome(&self, occ1: Office, occ2: Office, at_vespers: bool)
        -> OccurrenceOutcome;
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
    // assuming the office of the day is winner, returns true iff loser is to be commemorated
    fn occ_admits_commemoration(&self, winner: Office, loser: Office, at_vespers: bool) -> bool;
    // assuming vespers is of praec, returns true if seq is to be commemorated
    fn praec_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool;
    // assuming vespers is of seq, returns true if praec is to be commemorated
    fn seq_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool;
    fn order_office<'a>(
        &self,
        occs: Vec<Office<'a>>,
        allow_translation: bool,
    ) -> OrderedOffice<'a> {
        let mut to_commemorate: Vec<Office<'a>> = Vec::new();
        let mut to_translate: Vec<Office<'a>> = Vec::new();
        if occs.is_empty() {
            return OrderedOffice {
                office_of_day: Office::Empty,
                to_commemorate,
                to_translate,
            };
        }
        let mut occs = occs.clone();
        occs.sort_by(|&occ1, &occ2| self.compare_precedence_occ(occ1, occ2));
        let office_of_day: Office = occs.pop().unwrap();
        // reverse because we want to deal with higher-ranked things first
        for &occ in occs.iter().rev() {
            let outcome = self.occurrence_outcome(office_of_day, occ, false);
            assert_eq!(outcome.office_to_celebrate, OfficeIs::DePrimo);
            if outcome.loser_is == LoserIs::Translated && allow_translation {
                to_translate.push(occ);
            } else if (outcome.loser_is == LoserIs::Commemorated
                || outcome.loser_is == LoserIs::Translated)
                && to_commemorate.iter().all(|c| !c.is_of_same_person(occ))
            {
                // ASSUMPTION: any feast that would be translated is commemorated if it can't be
                to_commemorate.push(occ);
            }
        }
        to_commemorate.sort_by(|&c1, &c2| self.compare_commemoration_order(c1, c2));
        OrderedOffice {
            office_of_day,
            to_commemorate,
            to_translate,
        }
    }
    fn order_vespers<'a>(
        &self,
        praec_day: OrderedOffice<'a>,
        seq_day: OrderedOffice<'a>,
        seq_is_sunday: bool,
    ) -> OrderedVespers<'a> {
        let praec = if self.has_second_vespers(praec_day.office_of_day) {
            praec_day.office_of_day
        } else {
            Office::Empty
        };
        let seq = if self.has_first_vespers(seq_day.office_of_day, seq_is_sunday) {
            seq_day.office_of_day
        } else {
            Office::Empty
        };
        let mut to_commemorate: Vec<Office<'a>> = Vec::new();
        let co = self.concurrence_outcome(praec, seq, seq_is_sunday);
        let vespers = co.office_to_celebrate.applied_to(praec, seq);
        if co.praec_wins() && co.has_comm {
            to_commemorate.push(seq);
        } else if co.seq_wins() && co.has_comm {
            to_commemorate.push(praec);
        }
        let comms_from_praec = praec_day.to_commemorate.iter().filter(|&&off| {
            self.has_second_vespers(off)
                // extra check because some occuring offices are commemorated at lauds but not 2V
                && self.occ_admits_commemoration(praec, off, true)
                && (co.praec_wins() || self.seq_admits_commemoration(off, seq, seq_is_sunday))
        });
        let comms_from_seq = seq_day.to_commemorate.iter().filter(|&&off| {
            self.has_first_vespers(off, false)
                && (co.seq_wins() || self.praec_admits_commemoration(praec, off, seq_is_sunday))
        });
        to_commemorate.extend(chain(comms_from_praec, comms_from_seq).copied());
        to_commemorate.sort_by(|&c1, &c2| self.compare_commemoration_order(c1, c2));
        // TODO: remove duplicate commemorations of the same person/octave
        OrderedVespers {
            vespers,
            to_commemorate,
        }
    }
}
