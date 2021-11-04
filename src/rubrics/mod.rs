use std::cmp::Ordering;

#[cfg(test)]
mod tests;

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
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum FeastSubRank {
    Secondary,
    Primary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OctaveType {
    // TODO: others
    Common,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum SundayRank {
    Common,
    SecondClass,
    FirstClass,
}

#[derive(Debug, Clone, Copy)]
pub struct FeastDetails<'a> {
    id: &'a str,
    rank: FeastRank,
    sub_rank: FeastSubRank,
    person: Person,
    is_patron_or_titular: bool,
    is_privileged: bool,
    is_local: bool,
    is_moveable: bool,
    octave: Option<OctaveType>,
}

#[derive(Debug, Clone, Copy)]
pub enum Office<'a> {
    Feast(FeastDetails<'a>),
    Sunday {
        id: &'a str,
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
    WithinOctave(FeastDetails<'a>),
    OctaveDay(FeastDetails<'a>),
    Vigil(FeastDetails<'a>),
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
            Self::WithinOctave(_) => OfficeCategory::WithinOctave,
            Self::OctaveDay(_) => OfficeCategory::OctaveDay,
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
        match self.category() {
            OfficeCategory::GreaterFeria | OfficeCategory::Empty => true,
            _ => false,
        }
    }
    pub fn is_vigil(self) -> bool {
        self.category() == OfficeCategory::Vigil
    }
    pub fn is_greater_feria(self) -> bool {
        if let Self::GreaterFeria { .. } = self {
            true
        } else {
            false
        }
    }
    pub fn is_empty(self) -> bool {
        if let Self::Empty = self {
            true
        } else {
            false
        }
    }
    pub fn is_privileged(self) -> bool {
        if let Self::GreaterFeria {
            is_privileged: true,
            ..
        } = self
        {
            true
        } else if let Self::Feast(FeastDetails {
            is_privileged: true,
            ..
        }) = self
        {
            true
        } else {
            false
        }
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
            Self::Feast(fd) => Some(fd),
            Self::WithinOctave(fd) => Some(fd),
            Self::OctaveDay(fd) => Some(fd),
            Self::Vigil(fd) => Some(fd),
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
    Anticipated, // specifically for vigils which fall on Sunday
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConcurrenceOutcome {
    office_to_celebrate: VespersIs,
    has_comm: bool,
}

impl ConcurrenceOutcome {
    pub fn seq_wins(&self) -> bool {
        match self.office_to_celebrate {
            VespersIs::DeSeq | VespersIs::ACapSeq => true,
            _ => false,
        }
    }
    pub fn praec_wins(&self) -> bool {
        !self.seq_wins()
    }
}

pub struct OrderedOffice<'a> {
    office_of_day: Office<'a>,
    to_commemorate: Vec<Office<'a>>,
    to_translate: Vec<Office<'a>>,
}

pub trait RubricsSystem {
    fn has_first_vespers(&self, off: Office) -> bool;
    fn has_second_vespers(&self, off: Office) -> bool;
    fn occurrence_outcome(&self, occ1: Office, occ2: Office, at_vespers: bool)
        -> OccurrenceOutcome;
    // assumes both praec and seq are the office of their respective days
    fn concurrence_outcome(&self, praec: Office, seq: Office) -> ConcurrenceOutcome;
    fn order_office<'a>(&self, occs: Vec<Office<'a>>, allow_translation: bool)
        -> OrderedOffice<'a>;
}

pub struct Rubrics1910;

impl Rubrics1910 {
    // implementing all of these as methods rather than class functions
    // to make it easier later to copy this code for rubrics specified in an external format
    fn precedence_key_occ(&self, off: Office) -> u32 {
        match off {
            Office::Sunday {
                rank: SundayRank::FirstClass,
                ..
            } => 16,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                ..
            }) => 14,
            Office::Sunday {
                rank: SundayRank::SecondClass,
                ..
            } => 13,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                ..
            }) => 12,
            Office::OctaveDay(_) => 11,
            Office::Feast(FeastDetails {
                rank: FeastRank::GreaterDouble,
                ..
            }) => 10,
            Office::Feast(FeastDetails {
                rank: FeastRank::LesserDouble,
                ..
            }) => 9,
            Office::Sunday {
                rank: SundayRank::Common,
                ..
            } => 8,
            Office::Feast(FeastDetails {
                rank: FeastRank::Semidouble,
                ..
            }) => 7,
            Office::WithinOctave(_) => 6,
            Office::GreaterFeria { .. } => 5,
            Office::Vigil(_) => 4,
            Office::OurLadyOnSaturday => 3,
            Office::Feast(FeastDetails {
                rank: FeastRank::Simple,
                ..
            }) => 2,
            // ranking between Commemoration and "Empty" (feria per annum) is arbitrary
            // as Commemorations only occur on other feasts in these rubrics
            // but it makes things simpler to always have Empty rank last
            Office::Feast(FeastDetails {
                rank: FeastRank::Commemoration,
                ..
            }) => 1,
            Office::Empty => 0,
        }
    }
    fn precedence_key_conc(&self, is_second_vespers: bool, off: Office) -> u32 {
        match (is_second_vespers, off) {
            (
                _,
                Office::Feast(FeastDetails {
                    is_patron_or_titular: true,
                    ..
                }),
            ) => 9,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::DoubleFirstClass,
                    ..
                }),
            ) => 8,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::DoubleSecondClass,
                    ..
                }),
            ) => 7,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::GreaterDouble,
                    ..
                }),
            ) => 6,
            (_, Office::OctaveDay(_)) => 6,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::LesserDouble,
                    ..
                }),
            ) => 6,
            (true, Office::Sunday { .. }) => 5,
            (false, Office::Sunday { .. }) => 4,
            (_, Office::WithinOctave(_)) => 4,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::Semidouble,
                    ..
                }),
            ) => 4,
            // the rubrics don't actually explicitly say this
            (
                true,
                Office::GreaterFeria {
                    commemorated_at_vespers: true,
                    ..
                },
            ) => 3,
            // 1V of simples and OLOS should compare equal to empty (= feria per annum)
            // because their vespers starts after the cap
            (
                false,
                Office::Feast(FeastDetails {
                    rank: FeastRank::Simple,
                    ..
                }),
            ) => 0,
            (false, Office::OurLadyOnSaturday) => 0,
            (_, Office::Empty) => 0,
            _ => panic!(
                "unexpected vespers in concurrence: {:?} (is_second_vespers: {})",
                off, is_second_vespers
            ),
        }
    }
    fn commemoration_ordering_key(&self, off: Office) -> u32 {
        match off {
            Office::Feast(FeastDetails { rank, .. }) if rank >= FeastRank::GreaterDouble => 9,
            Office::OctaveDay(_) => 8, // not explicit in the rubrics
            Office::Feast(FeastDetails {
                rank: FeastRank::LesserDouble,
                ..
            }) => 7,
            Office::Sunday { .. } => 6,
            Office::Feast(FeastDetails {
                rank: FeastRank::Semidouble,
                ..
            }) => 5,
            Office::WithinOctave(_) => 4,
            Office::GreaterFeria { .. } => 3,
            Office::OurLadyOnSaturday => 2,
            Office::Feast(FeastDetails {
                rank: FeastRank::Simple | FeastRank::Commemoration,
                ..
            }) => 1,
            _ => 0, // TODO maybe should be a panic since nothing else should be commemorated
        }
    }
    // if d1 and d2 are both feasts, returns an ordering indicating which takes precedence
    // otherwise returns Ordering::Equal (so it can easily be included in a chain of comparisons
    // between arbitrary Offices)
    fn compare_feast_precedence(&self, off1: Office, off2: Office) -> Ordering {
        if let (Some(fd1), Some(fd2)) = (off1.feast_details(), off2.feast_details()) {
            true_is_greater(fd1.is_privileged, fd2.is_privileged)
                .then(fd1.rank.cmp(&fd2.rank))
                .then(fd1.person.cmp(&fd2.person))
                .then(true_is_greater(fd1.is_local, fd2.is_local))
                .then(false_is_greater(fd1.is_moveable, fd2.is_moveable))
        } else {
            Ordering::Equal
        }
    }
    fn compare_precedence_occ(&self, occ1: Office, occ2: Office) -> Ordering {
        true_is_greater(occ1.is_privileged(), occ2.is_privileged())
            .then(
                self.precedence_key_occ(occ1)
                    .cmp(&self.precedence_key_occ(occ2)),
            )
            .then(self.compare_feast_precedence(occ1, occ2))
    }
    // Less = is commemorated first
    // (which generally means higher ranked, so we reverse it at the end)
    fn compare_commemoration_order(&self, comm1: Office, comm2: Office) -> Ordering {
        self.commemoration_ordering_key(comm1)
            .cmp(&self.commemoration_ordering_key(comm2))
            .then(self.compare_feast_precedence(comm1, comm2)) // not explicitly in the rubrics but makes sense
            .reverse()
    }
    fn is_translated(&self, off: Office) -> bool {
        if let Some(fd) = off.feast_details() {
            fd.rank >= FeastRank::GreaterDouble
                || (fd.rank == FeastRank::LesserDouble && fd.person == Person::Doctor)
        } else {
            false
        }
    }
    // assuming the office of the day is winner, returns true iff loser is to be commemorated
    fn occ_admits_commemoration(&self, winner: Office, loser: Office, at_vespers: bool) -> bool {
        let loser_wants_commemoration = match loser {
            Office::GreaterFeria {
                commemorated_at_vespers: false,
                ..
            } if at_vespers => false,
            Office::OurLadyOnSaturday | Office::Empty => false,
            _ => true,
        };
        if !loser_wants_commemoration {
            return false;
        }
        if winner.is_of_same_person(loser) {
            return false;
        }
        if winner.is_greater_feria() && loser.is_vigil() {
            return false;
        }
        if winner.feast_rank() == Some(FeastRank::DoubleFirstClass) {
            return match loser.category() {
                OfficeCategory::Sunday
                | OfficeCategory::OctaveDay
                | OfficeCategory::GreaterFeria => true,
                OfficeCategory::Feast => loser.feast_rank().unwrap() >= FeastRank::GreaterDouble,
                _ => false,
            };
        }
        true
    }
    // assuming vespers is of praec, returns true if seq is to be commemorated
    fn praec_admits_commemoration(&self, praec: Office, seq: Office) -> bool {
        assert!(self.has_second_vespers(praec));
        if !self.has_first_vespers(seq) {
            return false;
        }
        if praec.is_of_same_person(seq) {
            return false;
        }
        if praec.feast_rank() == Some(FeastRank::DoubleFirstClass) {
            return match seq {
                Office::OurLadyOnSaturday | Office::WithinOctave(_) => false,
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::Semidouble,
                _ => true,
            };
        }
        true
    }
    // assuming vespers is of seq, returns true if praec is to be commemorated
    fn seq_admits_commemoration(&self, praec: Office, seq: Office) -> bool {
        assert!(self.has_first_vespers(seq));
        if !self.has_second_vespers(praec) {
            return false;
        }
        if let Office::GreaterFeria {
            commemorated_at_vespers: false,
            ..
        } = praec
        {
            return false;
        }
        if praec.is_of_same_person(seq) {
            return false;
        }
        match seq.feast_rank() {
            Some(FeastRank::DoubleFirstClass) => match praec {
                Office::Sunday { rank, .. } if rank >= SundayRank::SecondClass => true,
                Office::Feast(FeastDetails { rank, .. })
                    if rank >= FeastRank::DoubleSecondClass =>
                {
                    true
                }
                _ => false,
            },
            Some(FeastRank::DoubleSecondClass) => match praec {
                Office::Sunday { rank, .. } if rank >= SundayRank::SecondClass => true,
                Office::OctaveDay(_) => true,
                Office::Feast(FeastDetails { rank, .. }) if rank >= FeastRank::LesserDouble => true,
                _ => false,
            },
            _ => true,
        }
    }
}

impl RubricsSystem for Rubrics1910 {
    fn has_first_vespers(&self, off: Office) -> bool {
        match off.category() {
            OfficeCategory::Feast
            | OfficeCategory::WithinOctave  // days in octaves can have 1V, though it's usually omitted
            | OfficeCategory::OctaveDay
            | OfficeCategory::Sunday
            | OfficeCategory::OurLadyOnSaturday => true,
            _ => false,
        }
    }
    fn has_second_vespers(&self, off: Office) -> bool {
        match off {
            Office::Feast(FeastDetails { rank, .. }) if rank <= FeastRank::Simple => false,
            Office::Vigil(_) | Office::OurLadyOnSaturday => false,
            _ => true,
        }
    }
    fn occurrence_outcome(
        &self,
        occ1: Office,
        occ2: Office,
        at_vespers: bool,
    ) -> OccurrenceOutcome {
        let ord = self.compare_precedence_occ(occ1, occ2);
        let office_to_celebrate = match ord {
            // the rubrics assume there will never be occuring feasts of perfectly equal precedence
            // so how we treat that case is arbitrary
            Ordering::Greater | Ordering::Equal => OfficeIs::DePrimo,
            Ordering::Less => OfficeIs::DeSecundo,
        };
        let (winner, loser) = office_to_celebrate.winner_first(occ1, occ2);
        let loser_is = if self.is_translated(loser) {
            LoserIs::Translated
        } else if loser.is_vigil() && winner.is_sunday() {
            LoserIs::Anticipated
        } else {
            if self.occ_admits_commemoration(winner, loser, at_vespers) {
                LoserIs::Commemorated
            } else {
                LoserIs::Omitted
            }
        };
        OccurrenceOutcome {
            office_to_celebrate,
            loser_is,
        }
    }
    fn concurrence_outcome(&self, praec: Office, seq: Office) -> ConcurrenceOutcome {
        let praec = if self.has_second_vespers(praec) {
            praec
        } else {
            Office::Empty
        };
        let seq = if self.has_first_vespers(seq) {
            seq
        } else {
            Office::Empty
        };
        let ord = self
            .precedence_key_conc(true, praec)
            .cmp(&self.precedence_key_conc(false, seq));
        // ties are allowed at vespers, but not for high-ranked feasts
        let ord = match praec.feast_rank() {
            Some(rank) if rank >= FeastRank::DoubleSecondClass => {
                ord.then(self.compare_feast_precedence(praec, seq))
            }
            _ => ord,
        };
        let office_to_celebrate = match ord {
            Ordering::Greater => VespersIs::DePraec,
            Ordering::Equal => VespersIs::ACapSeq,
            Ordering::Less => VespersIs::DeSeq,
        };
        let has_comm = match office_to_celebrate {
            VespersIs::DePraec => self.praec_admits_commemoration(praec, seq),
            VespersIs::DeSeq | VespersIs::ACapSeq => self.seq_admits_commemoration(praec, seq),
        };
        ConcurrenceOutcome {
            office_to_celebrate,
            has_comm,
        }
    }
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
        // reverse because we want to deal with high-ranked things first
        for &occ in occs.iter().rev() {
            let outcome = self.occurrence_outcome(office_of_day, occ, false);
            assert_eq!(outcome.office_to_celebrate, OfficeIs::DePrimo);
            if outcome.loser_is == LoserIs::Translated && allow_translation {
                to_translate.push(occ)
            } else if (outcome.loser_is == LoserIs::Commemorated
                || outcome.loser_is == LoserIs::Translated)
                && to_commemorate.iter().all(|c| !c.is_of_same_person(occ))
            {
                to_commemorate.push(occ)
            }
        }
        to_commemorate.sort_by(|&c1, &c2| self.compare_commemoration_order(c1, c2));
        OrderedOffice {
            office_of_day,
            to_commemorate,
            to_translate,
        }
    }
}
