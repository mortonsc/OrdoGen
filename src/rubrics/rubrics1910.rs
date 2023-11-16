use super::*;
use chrono::{Datelike, NaiveDate};
use itertools::chain;
use liturgical::western::easter;

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
            //TODO: separate different octave types
            Office::OctaveDay { .. } => 11,
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
            //TODO: separate different octave types
            Office::WithinOctave { .. } => 6,
            Office::GreaterFeria { .. } => 5,
            Office::Vigil { .. } => 4,
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
            )
                // TODO: separate out different octave types
            | (_, Office::OctaveDay { .. })
            | (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::LesserDouble,
                    ..
                }),
            ) => 6,
            (true, Office::Sunday { .. }) => 5,
            (false, Office::Sunday { .. })
                // TODO: separate out different octave types
            | (_, Office::WithinOctave {..})
            | (
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
            )
            | (false, Office::OurLadyOnSaturday)
            | (_, Office::Empty) => 0,
            _ => panic!(
                "unexpected vespers in concurrence: {:?} (is_second_vespers: {})",
                off, is_second_vespers
            ),
        }
    }
    fn commemoration_ordering_key(&self, off: Office) -> u32 {
        match off {
            Office::Feast(FeastDetails { rank, .. }) if rank >= FeastRank::GreaterDouble => 9,
            Office::OctaveDay { .. } => 8, // not explicit in the rubrics; maybe should tie with greater double?
            Office::Feast(FeastDetails {
                rank: FeastRank::LesserDouble,
                ..
            }) => 7,
            Office::Sunday { .. } => 6,
            Office::Feast(FeastDetails {
                rank: FeastRank::Semidouble,
                ..
            }) => 5,
            Office::WithinOctave { .. } => 4,
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
    // returns VespersIs rather than an Ordering because returning an Ordering
    // might imply this is a proper ordering, which it's not
    fn compare_precedence_conc(&self, praec: Office, seq: Office) -> VespersIs {
        if seq.is_empty() {
            return VespersIs::DePraec;
        }
        // hacky special case for successive days in octaves
        if let (
            Office::WithinOctave {
                feast_details: fd1, ..
            },
            Office::WithinOctave {
                feast_details: fd2, ..
            },
        ) = (praec, seq)
        {
            if fd1.id == fd2.id {
                return VespersIs::DePraec;
            }
        }
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
        match ord {
            Ordering::Greater => VespersIs::DePraec,
            Ordering::Equal => VespersIs::ACapSeq,
            Ordering::Less => VespersIs::DeSeq,
        }
    }
    fn is_translated(&self, off: Office) -> bool {
        off.feast_details().map_or(false, |fd| {
            fd.rank >= FeastRank::GreaterDouble
                || (fd.rank == FeastRank::LesserDouble && fd.person == Person::Doctor)
        })
    }

    fn holy_family_date(&self, year: i32) -> usize {
        // Sunday after Epiphany
        let epiphany_date = NaiveDate::from_ymd_opt(year, 1, 6).expect("year out of range");
        (epiphany_date.ordinal0() as usize) + 7
            - (epiphany_date.weekday().number_from_monday() as usize)
    }
}

impl RubricsSystem for Rubrics1910 {
    fn has_first_vespers(&self, off: Office) -> bool {
        matches!(
            off.category(),
            OfficeCategory::Feast
            | OfficeCategory::WithinOctave  // days in octaves can have 1V, though it's usually omitted
            | OfficeCategory::OctaveDay
            | OfficeCategory::Sunday
            | OfficeCategory::OurLadyOnSaturday
            // Empty needs to have 1V so it can be a placeholder for 1V of days that don't have 1V
            | OfficeCategory::Empty
        )
    }
    fn has_second_vespers(&self, off: Office) -> bool {
        match off {
            Office::Feast(FeastDetails { rank, .. }) if rank <= FeastRank::Simple => false,
            Office::Vigil { .. } | Office::OurLadyOnSaturday => false,
            _ => true,
        }
    }
    // TODO: not very sure about this
    fn admits_translated_feast(&self, off: Office) -> bool {
        match off {
            Office::Sunday { rank, .. } => rank < SundayRank::SecondClass, // should this be all Sundays?
            Office::Feast(FeastDetails { rank, .. }) => rank < FeastRank::Semidouble,
            Office::OctaveDay { .. } => false,
            Office::GreaterFeria { is_privileged, .. } => !is_privileged,
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
        } else if self.occ_admits_commemoration(winner, loser, at_vespers) {
            LoserIs::Commemorated
        } else {
            LoserIs::Omitted
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
        let office_to_celebrate = self.compare_precedence_conc(praec, seq);
        let has_comm = match office_to_celebrate {
            VespersIs::DePraec => self.praec_admits_commemoration(praec, seq),
            VespersIs::DeSeq | VespersIs::ACapSeq => self.seq_admits_commemoration(praec, seq),
        };
        ConcurrenceOutcome {
            office_to_celebrate,
            has_comm,
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
            .reverse()
    }
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
                // TODO: is this true for all days within octaves?
                Office::OurLadyOnSaturday | Office::WithinOctave { .. } => false,
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::Semidouble,
                _ => true,
            };
        }
        true
    }
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
                // TODO: is this true for all octave days?
                Office::OctaveDay { .. } => true,
                Office::Feast(FeastDetails { rank, .. }) if rank >= FeastRank::LesserDouble => true,
                _ => false,
            },
            _ => true,
        }
    }
    fn date_of_moveable_feast(&self, id: &str, year: i32) -> Option<usize> {
        match id {
            "s-familiae-jmj" => Some(self.holy_family_date(year)),
            "ss-trinitatis" => {
                Some(easter::date(year).expect("year out of range").ordinal0() as usize + 56)
            }
            &_ => None,
        }
    }
}
