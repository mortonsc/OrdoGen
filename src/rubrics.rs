use std::cmp::Ordering;

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

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum FeriaRank {
    Common,
    Greater {
        privileged: bool,
        includes_vespers: bool,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct FeastDetails<'a> {
    id: &'a str,
    rank: FeastRank,
    sub_rank: FeastSubRank,
    person: Person,
    is_local: bool,
    is_moveable: bool,
    octave: Option<OctaveType>,
}

#[derive(Debug, Clone, Copy)]
pub enum LiturgicalDay<'a> {
    Feast(FeastDetails<'a>),
    Sunday {
        id: &'a str,
        rank: SundayRank,
    },
    Feria {
        // ordinary ferias aren't named
        id: Option<&'a str>,
        rank: FeriaRank,
        is_saturday: bool,
    },
    DayInOctave(FeastDetails<'a>),
    OctaveDay(FeastDetails<'a>),
    Vigil(FeastDetails<'a>),
    OurLadyOnSaturday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiturgicalDayCategory {
    Feast,
    Sunday,
    Feria,
    DayInOctave,
    OctaveDay,
    Vigil,
    OurLadyOnSaturday,
}

impl<'a> LiturgicalDay<'a> {
    pub fn category(self) -> LiturgicalDayCategory {
        match self {
            Self::Feast(_) => LiturgicalDayCategory::Feast,
            Self::Sunday { .. } => LiturgicalDayCategory::Sunday,
            Self::Feria { .. } => LiturgicalDayCategory::Feria,
            Self::DayInOctave(_) => LiturgicalDayCategory::DayInOctave,
            Self::OctaveDay(_) => LiturgicalDayCategory::OctaveDay,
            Self::Vigil { .. } => LiturgicalDayCategory::Vigil,
            Self::OurLadyOnSaturday => LiturgicalDayCategory::OurLadyOnSaturday,
        }
    }
    pub fn is_feast(self) -> bool {
        self.category() == LiturgicalDayCategory::Feast
    }
    pub fn is_sunday(self) -> bool {
        self.category() == LiturgicalDayCategory::Sunday
    }
    pub fn is_double_first_class(self) -> bool {
        if let Self::Feast(FeastDetails {
            rank: FeastRank::DoubleFirstClass,
            ..
        }) = self
        {
            true
        } else {
            false
        }
    }
    pub fn is_feria(self) -> bool {
        self.category() == LiturgicalDayCategory::Feria
    }
    pub fn is_vigil(self) -> bool {
        self.category() == LiturgicalDayCategory::Vigil
    }
    pub fn is_greater_feria(self) -> bool {
        if let Self::Feria {
            rank: FeriaRank::Greater { .. },
            ..
        } = self
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
            Self::DayInOctave(fd) => Some(fd),
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
    // TODO: right now this can't deal with persons besides those explicitly
    // given enum variants (not really a problem in practice though)
    pub fn is_of_same_person(self, other: Self) -> bool {
        let p1 = self.person();
        let p2 = other.person();
        p1.is_some() && p2.is_some() && p1 != Some(Person::Other) && p1 == p2
    }
    pub fn at_vespers(self) -> Self {
        match self {
            Self::Feria {
                id,
                rank:
                    FeriaRank::Greater {
                        includes_vespers: false,
                        ..
                    },
                is_saturday,
            } => Self::Feria {
                id,
                rank: FeriaRank::Common,
                is_saturday,
            },
            _ => self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OccurrenceLoserOutcome {
    Transferred,
    Anticipated, // only applies to vigils that fall on Sunday
    Commemorated,
    Omitted,
}

// note that when Vespers is split at the cap,
// we consider the Vespers of the following day to be the "winner"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VespersIs {
    DePraec,
    DeSeq,
    ACapSeq,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConcurrenceOutcome {
    office: VespersIs,
    has_comm: bool,
}

impl ConcurrenceOutcome {
    pub fn seq_wins(&self) -> bool {
        match self.office {
            VespersIs::DeSeq | VespersIs::ACapSeq => true,
            _ => false,
        }
    }
    pub fn praec_wins(&self) -> bool {
        !self.seq_wins()
    }
}

pub trait RubricsSystem {
    fn compare_precedence_occ(&self, occ1: LiturgicalDay, occ2: LiturgicalDay) -> Ordering;
    fn occurence_outcome_day(
        &self,
        occ1: LiturgicalDay,
        occ2: LiturgicalDay,
    ) -> (Ordering, OccurrenceLoserOutcome);
    // bool output is true if the "loser" is commemorated, false otherwise
    // doesn't check if either occ1 or occ2 actually has vespers
    fn occurence_outcome_eve(&self, occ1: LiturgicalDay, occ2: LiturgicalDay) -> (Ordering, bool) {
        let (ord, outcome) = self.occurence_outcome_day(occ1.at_vespers(), occ2.at_vespers());
        (ord, outcome == OccurrenceLoserOutcome::Commemorated)
    }
    // doesn't check if either praec or seq actually has second/first vespers
    fn concurrence_outcome_eve(
        &self,
        praec: LiturgicalDay,
        seq: LiturgicalDay,
    ) -> ConcurrenceOutcome;
    fn has_first_vespers(&self, day: LiturgicalDay) -> bool;
    fn has_second_vespers(&self, day: LiturgicalDay) -> bool;
}

pub struct Rubrics1910;

impl Rubrics1910 {
    fn precedence_key(day: LiturgicalDay) -> u32 {
        match day {
            LiturgicalDay::Sunday {
                rank: SundayRank::FirstClass,
                ..
            } => 16,
            LiturgicalDay::Feria {
                rank:
                    FeriaRank::Greater {
                        privileged: true, ..
                    },
                ..
            } => 15,
            LiturgicalDay::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                ..
            }) => 14,
            LiturgicalDay::Sunday {
                rank: SundayRank::SecondClass,
                ..
            } => 13,
            LiturgicalDay::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                ..
            }) => 12,
            LiturgicalDay::OctaveDay(_) => 11,
            LiturgicalDay::Feast(FeastDetails {
                rank: FeastRank::GreaterDouble,
                ..
            }) => 10,
            LiturgicalDay::Feast(FeastDetails {
                rank: FeastRank::LesserDouble,
                ..
            }) => 9,
            LiturgicalDay::Sunday {
                rank: SundayRank::Common,
                ..
            } => 8,
            LiturgicalDay::Feast(FeastDetails {
                rank: FeastRank::Semidouble,
                ..
            }) => 7,
            LiturgicalDay::DayInOctave(_) => 6,
            LiturgicalDay::Feria {
                rank: FeriaRank::Greater { .. },
                ..
            } => 5,
            LiturgicalDay::Vigil(_) => 4,
            LiturgicalDay::OurLadyOnSaturday => 3,
            LiturgicalDay::Feast(FeastDetails {
                rank: FeastRank::Simple,
                ..
            }) => 2,
            LiturgicalDay::Feria {
                rank: FeriaRank::Common,
                ..
            } => 1,
            LiturgicalDay::Feast(FeastDetails {
                rank: FeastRank::Commemoration,
                ..
            }) => 0,
        }
    }
    fn compare_feast_precedence(f1: FeastDetails, f2: FeastDetails) -> Ordering {
        f1.rank
            .cmp(&f2.rank)
            .then(f1.person.cmp(&f2.person))
            .then(true_is_greater(f1.is_local, f2.is_local))
            .then(false_is_greater(f1.is_moveable, f2.is_moveable))
    }
    fn is_transferred(fd: FeastDetails) -> bool {
        match fd.rank {
            FeastRank::DoubleFirstClass
            | FeastRank::DoubleSecondClass
            | FeastRank::GreaterDouble => true,
            FeastRank::LesserDouble => fd.person >= Person::Doctor,
            _ => false,
        }
    }
    // assuming winner outranks loser when they occur, and loser would ordinarily be commemorated,
    // this returns true iff winner does not exclude the commemoration
    fn admits_commemoration(&self, winner: LiturgicalDay, loser: LiturgicalDay) -> bool {
        if winner.is_of_same_person(loser) {
            return false;
        }
        // two further cases where commemoration is omitted:
        // 1. winner is d1cl and loser is anything except a greater feria, sunday, octave day, or
        //    feast of at least dm rank
        // 2. winner is a greater feria and loser is a vigil
        if winner.is_double_first_class() {
            match loser.category() {
                LiturgicalDayCategory::Sunday | LiturgicalDayCategory::OctaveDay => true,
                LiturgicalDayCategory::Feria => loser.is_greater_feria(),
                LiturgicalDayCategory::Feast => {
                    loser.feast_details().unwrap().rank >= FeastRank::GreaterDouble
                }
                _ => false,
            }
        } else if winner.is_greater_feria() && loser.is_vigil() {
            false
        } else {
            true
        }
    }
}

impl RubricsSystem for Rubrics1910 {
    fn compare_precedence_occ(&self, occ1: LiturgicalDay, occ2: LiturgicalDay) -> Ordering {
        match (occ1, occ2) {
            (LiturgicalDay::Feast(fd1), LiturgicalDay::Feast(fd2)) => {
                Self::compare_feast_precedence(fd1, fd2)
            }
            _ => Self::precedence_key(occ1).cmp(&Self::precedence_key(occ2)),
        }
    }
    fn occurence_outcome_day(
        &self,
        occ1: LiturgicalDay,
        occ2: LiturgicalDay,
    ) -> (Ordering, OccurrenceLoserOutcome) {
        let ord = self.compare_precedence_occ(occ1, occ2);
        let (winner, loser) = match ord {
            Ordering::Greater | Ordering::Equal => (occ1, occ2),
            Ordering::Less => (occ2, occ1),
        };
        // the procedure for determining what happens to the loser has two parts:
        // first decide what to do based only on looking at the loser
        // then if you think it should be commemorated, check if the winner
        // allows the commemoration
        // (also the special case for anticipating vigils to Saturdays happens in the first part)
        let based_on_loser = match loser {
            LiturgicalDay::Feast(fd) if Self::is_transferred(fd) => {
                OccurrenceLoserOutcome::Transferred
            }
            LiturgicalDay::Vigil(_) => {
                if let LiturgicalDay::Sunday { .. } = winner {
                    OccurrenceLoserOutcome::Anticipated
                } else {
                    OccurrenceLoserOutcome::Commemorated
                }
            }
            LiturgicalDay::OurLadyOnSaturday
            | LiturgicalDay::Feria {
                rank: FeriaRank::Common,
                ..
            } => OccurrenceLoserOutcome::Omitted,
            _ => OccurrenceLoserOutcome::Commemorated,
        };
        let actual_outcome = if based_on_loser != OccurrenceLoserOutcome::Commemorated {
            based_on_loser
        } else if self.admits_commemoration(winner, loser) {
            OccurrenceLoserOutcome::Commemorated
        } else {
            OccurrenceLoserOutcome::Omitted
        };
        (ord, actual_outcome)
    }
    fn concurrence_outcome_eve(
        &self,
        _praec: LiturgicalDay,
        _seq: LiturgicalDay,
    ) -> ConcurrenceOutcome {
        ConcurrenceOutcome {
            office: VespersIs::DePraec,
            has_comm: false,
        }
    }
    fn has_first_vespers(&self, day: LiturgicalDay) -> bool {
        match day {
            LiturgicalDay::Feast(_)
            | LiturgicalDay::OctaveDay(_)
            | LiturgicalDay::Sunday { .. }
            | LiturgicalDay::OurLadyOnSaturday => true,
            _ => false,
        }
    }
    fn has_second_vespers(&self, day: LiturgicalDay) -> bool {
        match day {
            LiturgicalDay::Feast(FeastDetails {
                rank: FeastRank::Simple,
                ..
            })
            | LiturgicalDay::Vigil(_)
            | LiturgicalDay::Feria {
                is_saturday: true, ..
            }
            | LiturgicalDay::OurLadyOnSaturday => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmp_person() {
        assert!(Person::OurLady < Person::OurLord);
        assert!(Person::JohnBaptist > Person::Joseph);
        assert!(Person::Other < Person::Trinity);
    }
}
