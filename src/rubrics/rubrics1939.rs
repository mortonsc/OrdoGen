use super::*;

pub struct Rubrics1939;

impl Rubrics1939 {
    // implementing all of these as methods rather than class functions
    // to make it easier later to copy this code for rubrics specified in an external format
    fn precedence_key_occ(&self, off: Office) -> u32 {
        match off {
            Office::Sunday {
                rank: SundayRank::FirstClass,
                ..
            } => 100,
            Office::Feria {
                rank: FeriaRank::Privileged | FeriaRank::Special,
                ..
            } => 100,
            Office::Vigil {
                rank: VigilRank::FirstClass,
                ..
            } => 100,
            Office::WithinOctave {
                rank: OctaveRank::FirstOrder,
                ..
            } => 100,
            Office::OctaveDay {
                rank: OctaveRank::FirstOrder,
                ..
            } => panic!("there are no 1st order octaves with octave days"),
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                is_local: false,
                ..
            }) => 95,
            Office::OctaveDay {
                rank: OctaveRank::SecondOrder,
                ..
            } => 94,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                is_local: true,
                ..
            }) => 93,
            Office::WithinOctave {
                rank: OctaveRank::SecondOrder,
                ..
            } => 92,
            Office::Sunday {
                rank: SundayRank::SecondClass,
                ..
            } => 90,
            Office::Vigil {
                rank: VigilRank::SecondClass,
                ..
            } => 90,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                ..
            }) => 80,
            Office::Sunday {
                rank: SundayRank::Common,
                ..
            } => 50,
            Office::OctaveDay {
                rank: OctaveRank::ThirdOrder,
                ..
            } => 47,
            Office::OctaveDay {
                rank: OctaveRank::Common,
                ..
            } => 46,
            Office::Feast(FeastDetails {
                rank: FeastRank::GreaterDouble,
                ..
            }) => 45,
            Office::Feast(FeastDetails {
                rank: FeastRank::LesserDouble,
                ..
            }) => 40,
            Office::Feast(FeastDetails {
                rank: FeastRank::Semidouble,
                ..
            }) => 30,
            Office::WithinOctave {
                rank: OctaveRank::ThirdOrder,
                ..
            } => 25,
            Office::WithinOctave {
                rank: OctaveRank::Common,
                ..
            } => 24,
            // pre-55 doesn't distinguish between these three classes of feria
            Office::Feria {
                rank: FeriaRank::SecondClass,
                ..
            } => 20,
            Office::Feria {
                rank: FeriaRank::ThirdClass,
                ..
            } => 20,
            Office::Feria {
                rank: FeriaRank::ThirdClassAdvent,
                ..
            } => 20,
            Office::Vigil {
                rank: VigilRank::Common,
                ..
            } => 17,
            Office::OctaveDay {
                rank: OctaveRank::Simple,
                ..
            } => 15,
            Office::OurLadyOnSaturday => 12,
            Office::Feast(FeastDetails {
                rank: FeastRank::Simple,
                ..
            }) => 10,
            Office::Feria {
                rank: FeriaRank::Common,
                ..
            } => 5,
            Office::WithinOctave {
                rank: OctaveRank::Simple,
                ..
            } => panic!("there are no days within simple octaves"),
            Office::Empty => 0,
            _ => panic!("unrecognized office: {:?}", off),
        }
    }
    fn precedence_key_conc(&self, is_second_vespers: bool, off: Office) -> u32 {
        if is_second_vespers {
            assert!(self.has_second_vespers(off));
        } else {
            // it being Sunday or not is irrelevant to 1939 rubrics
            assert!(self.has_first_vespers(off, false));
        }
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
                Office::Feria {
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
            Office::Feria { .. } => 3,
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
            fd1.rank
                .cmp(&fd2.rank)
                .then(fd1.person.cmp(&fd2.person))
                .then(true_is_greater(fd1.is_local, fd2.is_local))
                .then(false_is_greater(fd1.is_moveable, fd2.is_moveable))
        } else {
            Ordering::Equal
        }
    }
    fn is_translated(&self, off: Office) -> bool {
        off.feast_details().map_or(false, |fd| {
            fd.rank >= FeastRank::GreaterDouble
                || (fd.rank == FeastRank::LesserDouble && fd.person == Person::Doctor)
        })
    }
}

impl RubricsSystem for Rubrics1939 {
    fn has_first_vespers(&self, off: Office, _is_sunday: bool) -> bool {
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
            Office::Feria {
                rank: FeriaRank::Special | FeriaRank::Privileged,
                ..
            } => false,
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
    fn concurrence_outcome(
        &self,
        praec: Office,
        seq: Office,
        seq_is_sunday: bool,
    ) -> ConcurrenceOutcome {
        let praec = if self.has_second_vespers(praec) {
            praec
        } else {
            Office::Empty
        };
        let seq = if self.has_first_vespers(seq, seq_is_sunday) {
            seq
        } else {
            Office::Empty
        };
        let office_to_celebrate = self.compare_precedence_conc(praec, seq);
        let has_comm = match office_to_celebrate {
            VespersIs::DePraec => self.praec_admits_commemoration(praec, seq, seq_is_sunday),
            VespersIs::DeSeq | VespersIs::ACapSeq => {
                self.seq_admits_commemoration(praec, seq, seq_is_sunday)
            }
        };
        ConcurrenceOutcome {
            office_to_celebrate,
            has_comm,
        }
    }
    fn compare_precedence_occ(&self, occ1: Office, occ2: Office) -> Ordering {
        match (occ1, occ2) {
            // special case for Sundays in octaves
            (Office::Sunday { .. }, Office::WithinOctave { .. }) => Ordering::Greater,
            (Office::WithinOctave { .. }, Office::Sunday { .. }) => Ordering::Less,
            // TODO potentially: special case that vigil of Epiphany (2 class) cedes to feasts of
            // the Lord; but idk when this would ever occur with a feast that wasn't 1st or 2nd
            // class (and therefore already superseding the vigil by virtue of its rank)
            _ => self
                .precedence_key_occ(occ1)
                .cmp(&self.precedence_key_occ(occ2))
                .then(self.compare_feast_precedence(occ1, occ2)),
        }
    }
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
    // Less = is commemorated first
    // (which generally means higher ranked, so we reverse it at the end)
    fn compare_commemoration_order(&self, comm1: Office, comm2: Office) -> Ordering {
        self.commemoration_ordering_key(comm1)
            .cmp(&self.commemoration_ordering_key(comm2))
            .reverse()
    }
    fn occ_admits_commemoration(&self, winner: Office, loser: Office, at_vespers: bool) -> bool {
        let loser_wants_commemoration = match loser {
            Office::Feria {
                commemorated_at_vespers: false,
                ..
            } if at_vespers => false,
            Office::Feria {
                rank: FeriaRank::Common,
                ..
            } => false,
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
                // Ferias return true now because if it's not a greater feria we returned earlier
                OfficeCategory::Sunday | OfficeCategory::OctaveDay | OfficeCategory::Feria => true,
                OfficeCategory::Feast => loser.feast_rank().unwrap() >= FeastRank::GreaterDouble,
                _ => false,
            };
        }
        true
    }
    fn praec_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool {
        assert!(self.has_second_vespers(praec));
        if !self.has_first_vespers(seq, seq_is_sunday) {
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
    fn seq_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool {
        assert!(self.has_first_vespers(seq, seq_is_sunday));
        if !self.has_second_vespers(praec) {
            return false;
        }
        if let Office::Feria {
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
}
