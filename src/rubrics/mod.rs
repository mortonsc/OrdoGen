use serde::Deserialize;
use std::cmp::Ordering;

mod display;
mod rubrics1910;

#[cfg(test)]
mod tests;

pub use rubrics1910::Rubrics1910;

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
    // commemorations are effectively just perpetually superseded simples
    // but many days are simple feast + commemoration
    // and we need a separate category for commemorations to know which is which
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum OctaveType {
    // TODO: others
    Common,
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
    pub is_privileged: bool,
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
            is_privileged: false,
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
    pub fn make_privileged(mut self) -> Self {
        self.is_privileged = true;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Office<'a> {
    Feast(FeastDetails<'a>),
    Sunday {
        id: &'a str,
        matins_id: Option<&'a str>,
        rank: SundayRank,
    },
    GreaterFeria {
        // weekdays of Advent/Lent don't have individual id's
        id: Option<&'a str>,
        // false both for Saturdays (which really don't have second vespers at all)
        // and for a few days which can have second vespers but aren't commemorated at second
        // vespers if superseded by a feast
        commemorated_at_vespers: bool,
        is_privileged: bool,
    },
    WithinOctave {
        id: &'a str,
        feast_details: FeastDetails<'a>,
        octave_type: OctaveType,
    },
    OctaveDay {
        id: &'a str,
        feast_details: FeastDetails<'a>,
        octave_type: OctaveType,
    },
    Vigil {
        id: &'a str,
        feast_details: FeastDetails<'a>,
    },
    OurLadyOnSaturday,
    // used both for ordinary ferias per annum and as a placeholder for an office that doesn't exist
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfficeCategory {
    Feast,
    Sunday,
    GreaterFeria,
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
            Self::GreaterFeria { .. } => OfficeCategory::GreaterFeria,
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
            OfficeCategory::GreaterFeria | OfficeCategory::Empty
        )
    }
    pub fn is_vigil(self) -> bool {
        self.category() == OfficeCategory::Vigil
    }
    pub fn is_greater_feria(self) -> bool {
        matches!(self, Self::GreaterFeria { .. })
    }
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
    pub fn is_privileged(self) -> bool {
        matches!(
            self,
            Self::GreaterFeria {
                is_privileged: true,
                ..
            } | Self::Feast(FeastDetails {
                is_privileged: true,
                ..
            })
        )
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
    fn has_first_vespers(&self, off: Office) -> bool;
    fn has_second_vespers(&self, off: Office) -> bool;
    fn admits_translated_feast(&self, off: Office) -> bool;
    fn occurrence_outcome(&self, occ1: Office, occ2: Office, at_vespers: bool)
        -> OccurrenceOutcome;
    // assumes both praec and seq are the office of their respective days
    fn concurrence_outcome(&self, praec: Office, seq: Office) -> ConcurrenceOutcome;
    fn order_office<'a>(&self, occs: Vec<Office<'a>>, allow_translation: bool)
        -> OrderedOffice<'a>;
    fn order_vespers<'a>(
        &self,
        praec_day: OrderedOffice<'a>,
        seq_day: OrderedOffice<'a>,
    ) -> OrderedVespers<'a>;
    fn date_of_moveable_feast(&self, id: &str, year: i32) -> Option<usize>;
}
