use super::*;

#[derive(Clone, Copy)]
pub struct Rubrics1939;

impl Rubrics1939 {
    fn precedence_key_occ(&self, off: Office) -> u32 {
        match off {
            Office::Sunday {
                rank: SundayRank::FirstClass,
                ..
            } => 100,
            Office::Feria {
                rank: FeriaRank::Privileged | FeriaRank::DoubleFirstClass,
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
            Office::AllSouls => 100,
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
            Office::Sunday {
                rank: SundayRank::WithinOctave(OctaveRank::SecondOrder),
                ..
            } => 92,
            Office::WithinOctave {
                rank: OctaveRank::SecondOrder,
                ..
            } => 91,
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
            Office::Sunday {
                rank: SundayRank::WithinOctave(OctaveRank::ThirdOrder),
                ..
            } => 48,
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
            Office::Feria {
                rank:
                    FeriaRank::SecondClass
                    | FeriaRank::ThirdClass
                    | FeriaRank::ThirdClassAdvent
                    | FeriaRank::AnticipatedSunday,
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
            Office::Empty => 0,
            _ => panic!("unexpected office in occurence: {:?}", off),
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
                true,
                Office::Feria {
                    rank: FeriaRank::DoubleFirstClass,
                    ..
                },
            ) => 21,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::DoubleFirstClass,
                    ..
                }),
            ) => 20,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::DoubleSecondClass,
                    ..
                }),
            ) => 19,
            (true, Office::Sunday { .. }) => 18,
            (true, Office::OctaveDay { rank, .. }) if rank >= OctaveRank::ThirdOrder => 17,
            (false, Office::Sunday { .. }) => 16,
            (
                true,
                Office::OctaveDay {
                    rank: OctaveRank::Common,
                    ..
                },
            ) => 15,
            (false, Office::OctaveDay { rank, .. }) if rank >= OctaveRank::Common => 14,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::GreaterDouble,
                    ..
                }),
            ) => 13,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::LesserDouble,
                    ..
                }),
            ) => 12,
            (
                _,
                Office::Feast(FeastDetails {
                    rank: FeastRank::Semidouble,
                    ..
                }),
            ) => 11,
            (_, Office::WithinOctave { .. }) => 10,
            (
                false,
                Office::OctaveDay {
                    rank: OctaveRank::Simple,
                    ..
                },
            ) => 9,
            (
                false,
                Office::Feast(FeastDetails {
                    rank: FeastRank::Simple,
                    ..
                }),
            ) => 9,
            (false, Office::OurLadyOnSaturday) => 9,
            (true, Office::Feria { .. }) => 8,
            // All Souls is a special case because its 1V is celbrated in addition to 2V of the
            // preceding day; we treat this as a "commemoration" of All Souls
            (false, Office::AllSouls) => 1,
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
            Office::OctaveDay { .. } => 9, // not explicit in the rubrics; maybe should tie with greater double?
            Office::Feast(FeastDetails {
                rank: FeastRank::LesserDouble,
                ..
            }) => 8,
            Office::Sunday { .. } => 7,
            Office::Feast(FeastDetails {
                rank: FeastRank::Semidouble,
                ..
            }) => 6,
            Office::WithinOctave { .. } => 5,
            Office::Feria { .. } => 4,
            Office::OurLadyOnSaturday => 3,
            Office::Feast(FeastDetails {
                rank: FeastRank::Simple | FeastRank::Commemoration,
                ..
            }) => 2,
            Office::AllSouls => 1,
            Office::Empty => 0,
            _ => panic!("unexpected commemorated office: {:?}", off),
        }
    }
    // if d1 and d2 are both feasts, returns an ordering indicating which takes precedence
    // otherwise returns Ordering::Equal (so it can easily be included in a chain of comparisons
    // between arbitrary Offices)
    fn compare_feast_precedence(&self, off1: Office, off2: Office) -> Ordering {
        if let (Some(fd1), Some(fd2)) = (off1.feast_details(), off2.feast_details()) {
            fd1.rank
                .cmp(&fd2.rank)
                .then(true_is_greater(fd1.is_solemn(), fd2.is_solemn()))
                .then(fd1.sub_rank.cmp(&fd2.sub_rank))
                .then(fd1.person.cmp(&fd2.person))
                .then(true_is_greater(fd1.is_local, fd2.is_local))
        } else {
            Ordering::Equal
        }
    }
    fn is_translated(&self, off: Office) -> bool {
        off.feast_details()
            .map_or(false, |fd| fd.rank >= FeastRank::DoubleSecondClass)
    }
    // returns whether off is of the sort to be commemorated
    // within an occuring office at the given hour
    fn wants_commemoration(&self, off: Office, hour: Hour) -> bool {
        if hour == Hour::FirstVespers && !self.has_first_vespers(off, false)
            || hour == Hour::SecondVespers && !self.has_second_vespers(off)
        {
            return false;
        }
        match off {
            Office::Feria {
                commemorated_at_vespers: false,
                ..
            } if hour == Hour::SecondVespers => false,
            // OLOS can be commemorated in concurrence (but not occurence)
            Office::OurLadyOnSaturday => hour == Hour::FirstVespers,
            Office::Feria {
                rank: FeriaRank::Common,
                ..
            }
            | Office::Empty => false,
            _ => true,
        }
    }
}

impl RubricsSystem for Rubrics1939 {
    fn has_first_vespers(&self, off: Office, _is_sunday: bool) -> bool {
        match off {
            Office::Feast(_) => true,
            // days in octaves can have 1V, though it's usually omitted
            Office::WithinOctave { .. } => true,
            Office::OctaveDay { .. } => true,
            Office::Sunday { .. } => true,
            // Epiphany vigil
            Office::Vigil {
                rank: VigilRank::SecondClass,
                ..
            } => true,
            Office::AllSouls => true,
            Office::OurLadyOnSaturday => true,
            // Empty needs to have 1V so it can be a placeholder for 1V of days that don't have 1V
            Office::Empty => true,
            _ => false,
        }
    }
    fn has_second_vespers(&self, off: Office) -> bool {
        match off {
            Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::Semidouble,
            Office::Feria {
                has_second_vespers, ..
            } => has_second_vespers,
            Office::WithinOctave {
                has_second_vespers, ..
            } => has_second_vespers,
            Office::OctaveDay { rank, .. } => rank > OctaveRank::Simple,
            Office::Vigil { .. } | Office::AllSouls | Office::OurLadyOnSaturday => false,
            _ => true,
        }
    }
    // TODO: not very sure about this
    // probably has to account for the rank of the translated feast
    fn admits_translated_feast(&self, off: Office) -> bool {
        match off {
            Office::Sunday { .. } => false,
            Office::Feast(FeastDetails { rank, .. }) => rank < FeastRank::DoubleSecondClass,
            Office::WithinOctave { rank, .. } => rank < OctaveRank::SecondOrder,
            Office::Feria { rank, .. } => rank < FeriaRank::Privileged,
            Office::Vigil { rank, .. } => rank < VigilRank::FirstClass,
            Office::AllSouls => false,
            _ => true,
        }
    }
    fn occurrence_outcome(&self, occ1: Office, occ2: Office) -> OccurrenceOutcome {
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
        } else if self.occ_admits_commemoration(winner, loser, Hour::Lauds) {
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
        assert!(self.has_second_vespers(praec));
        assert!(self.has_first_vespers(seq, seq_is_sunday));
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
            // special case for feasts of the Lord on Sunday
            (
                Office::Sunday { .. },
                Office::Feast(FeastDetails {
                    rank,
                    person: Person::OurLord,
                    ..
                }),
            ) if rank >= FeastRank::Semidouble => Ordering::Less,
            (
                Office::Feast(FeastDetails {
                    rank,
                    person: Person::OurLord,
                    ..
                }),
                Office::Sunday { .. },
            ) if rank >= FeastRank::Semidouble => Ordering::Greater,
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
            .cmp(&self.precedence_key_conc(false, seq))
            .then(self.compare_feast_precedence(praec, seq));
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
    fn occ_admits_commemoration(&self, winner: Office, loser: Office, hour: Hour) -> bool {
        if !self.wants_commemoration(loser, hour) {
            return false;
        }
        if loser == Office::OurLadyOnSaturday {
            // this needs to be a separate case from wants_commemoration() because OLOS is
            // sometimes commemorated in concurrence
            return false;
        }
        if winner.is_of_same_subject(loser) {
            return false;
        }
        if matches!(
            winner,
            Office::Feria {
                rank: FeriaRank::DoubleFirstClass,
                ..
            }
        ) {
            return false;
        }
        if winner == Office::AllSouls {
            return false;
        }
        if winner.is_greater_feria() && loser.is_vigil() {
            return false;
        }
        if matches!(
            winner,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                sub_rank: FeastSubRank::Primary,
                person: Person::OurLord | Person::Trinity,
                is_local: false,
                ..
            })
        ) && matches!(loser, Office::Feast(_))
        {
            return false;
        }
        match winner.feast_rank() {
            Some(FeastRank::DoubleFirstClass) => match loser {
                Office::OctaveDay {
                    rank: OctaveRank::Simple,
                    ..
                } => false,
                Office::WithinOctave {
                    rank: OctaveRank::Common,
                    ..
                } => false,
                Office::Vigil { .. } => false,
                Office::Feast(FeastDetails { rank, .. }) => rank > FeastRank::Simple,
                _ => true,
            },
            Some(FeastRank::DoubleSecondClass) => match loser {
                Office::WithinOctave {
                    rank: OctaveRank::Common,
                    ..
                } => false,
                _ => true,
            },
            _ => true,
        }
    }
    fn praec_admits_commemoration(&self, praec: Office, seq: Office, _seq_is_sunday: bool) -> bool {
        assert!(self.has_second_vespers(praec));
        if !self.wants_commemoration(seq, Hour::FirstVespers) {
            return false;
        }
        if praec.is_of_same_subject(seq) {
            return false;
        }
        if matches!(
            praec,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass | FeastRank::DoubleSecondClass,
                ..
            })
        ) {
            return match seq {
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::Semidouble,
                Office::WithinOctave { rank, .. } => rank >= OctaveRank::ThirdOrder,
                Office::OurLadyOnSaturday => false,
                _ => true,
            };
        }
        true
    }
    fn seq_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool {
        assert!(self.has_first_vespers(seq, seq_is_sunday));
        if !self.wants_commemoration(praec, Hour::SecondVespers) {
            return false;
        }
        if praec.is_of_same_subject(seq) {
            return false;
        }
        match seq.feast_rank() {
            Some(FeastRank::DoubleFirstClass) => match praec {
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::DoubleSecondClass,
                Office::WithinOctave { rank, .. } => rank >= OctaveRank::ThirdOrder,
                Office::OctaveDay { rank, .. } => rank >= OctaveRank::ThirdOrder,
                _ => true,
            },
            Some(FeastRank::DoubleSecondClass) => match praec {
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::LesserDouble,
                Office::WithinOctave { rank, .. } => rank >= OctaveRank::ThirdOrder,
                _ => true,
            },
            _ => true,
        }
    }
    fn anticipate_vigils(&self) -> bool {
        true
    }
    fn admits_anticipated_sunday(&self, off: Office) -> bool {
        // Sundays cannot be anticipated to "days of 9 lessons"
        match off {
            Office::Feast(FeastDetails { rank, .. }) => rank <= FeastRank::Simple,
            Office::WithinOctave { .. } => false,
            Office::OctaveDay { rank, .. } => rank > OctaveRank::Simple,
            Office::Feria {
                rank: FeriaRank::Common,
                ..
            }
            | Office::OurLadyOnSaturday
            | Office::Empty => true,
            _ => panic!("trying to anticipate a Sunday to unexpected day: {:?}", off),
        }
    }
}
