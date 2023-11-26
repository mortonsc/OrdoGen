use log::warn;

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
            }) => 96,
            Office::OctaveDay {
                rank: OctaveRank::SecondOrder,
                ..
            } => 95,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                is_local: true,
                ..
            }) => 94,
            // special case since Holy Family is nominally a greater double
            // but it needs to outrank the Sunday in the octave of the Epiphany
            Office::Feast(FeastDetails {
                id: "s-familiae-jmj",
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
                rank: FeastRank::Double,
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
                    FeriaRank::FridayAfterOctAsc
                    | FeriaRank::SecondClass
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
            Office::Feast(FeastDetails {
                rank: FeastRank::Commemoration,
                ..
            }) => 1,
            Office::Empty => 0,
            _ => panic!("unexpected office in occurence: {:?}", off),
        }
    }
    fn precedence_key_conc(&self, is_second_vespers: bool, off: Office) -> u32 {
        if is_second_vespers {
            assert!(self.has_second_vespers(off));
        } else {
            // assume it's a Sunday (it's not worth passing is_sunday as a parameter just for the
            // sake of this assert)
            assert!(self.has_first_vespers(off, true));
        }
        match (is_second_vespers, off) {
            (
                true,
                Office::Feria {
                    rank: FeriaRank::DoubleFirstClass,
                    ..
                },
            ) => 21,
            // Vigil of the Nativity, when it falls on a Sunday
            // this ranking is mostly arbitrary as it doesn't concur with feasts
            (
                false,
                Office::Vigil {
                    rank: VigilRank::FirstClass,
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
            // Vigil of the Epiphany
            (
                false,
                Office::Vigil {
                    rank: VigilRank::SecondClass,
                    ..
                },
            ) => 19,
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
                    rank: FeastRank::Double,
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
            (
                false,
                Office::Feast(FeastDetails {
                    rank: FeastRank::Commemoration,
                    ..
                }),
            ) => 2,
            // All Souls is a special case because its 1V is celebrated in addition to 2V of the
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
            Office::Sunday { .. } => 15,
            Office::Vigil {
                rank: VigilRank::SecondClass,
                ..
            } => 15,
            // the rubrics don't say this explicitly but generally they treats anticipated Sundays
            // the same as other Sundays unless they explicitly distinguish them
            Office::Feria {
                rank: FeriaRank::AnticipatedSunday,
                ..
            } => 15,
            Office::WithinOctave {
                rank: OctaveRank::SecondOrder,
                ..
            } => 14,
            Office::OctaveDay { rank, .. } if rank >= OctaveRank::Common => 13,
            Office::Feast(FeastDetails {
                rank: FeastRank::GreaterDouble,
                ..
            }) => 12,
            Office::Feast(FeastDetails {
                rank: FeastRank::Double,
                ..
            }) => 11,
            Office::Feast(FeastDetails {
                rank: FeastRank::Semidouble,
                ..
            }) => 10,
            Office::WithinOctave {
                rank: OctaveRank::ThirdOrder,
                ..
            } => 9,
            Office::WithinOctave {
                rank: OctaveRank::Common,
                ..
            } => 8,
            // TODO: Friday after oct asc => 7
            Office::Feria {
                rank: FeriaRank::FridayAfterOctAsc,
                ..
            } => 7,
            Office::Feria { rank, .. } if rank > FeriaRank::Common => 6,
            Office::Vigil {
                rank: VigilRank::Common,
                ..
            } => 5,
            Office::OctaveDay {
                rank: OctaveRank::Simple,
                ..
            } => 4,
            // TODO: ordering between OLOS and a simple feast/octave day isn't explicit in the rubrics
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
    // if d1 and d2 are both feasts, returns which feast has precedence
    // this should only be used to compare days of the same rite
    // as it will give incorrect results if you compare a day within an octave to a feast
    // because it will compare the feast to the feast the octave is of
    // if either day is not a feast / day within octave, returns Ordering::Equal
    // (so it can easily be included in a chain of comparisons between arbitrary Offices)
    fn compare_feast_precedence(&self, off1: Office, off2: Office) -> Ordering {
        if let (Some(fd1), Some(fd2)) = (off1.assoc_feast_details(), off2.assoc_feast_details()) {
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
    fn wants_commemoration(&self, off: Office, hour: Hour, is_sunday: bool) -> bool {
        if hour == Hour::FirstVespers && !self.has_first_vespers(off, is_sunday)
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
    fn has_first_vespers(&self, off: Office, is_sunday: bool) -> bool {
        match off {
            Office::Feast(_) => true,
            // days in octaves can have 1V, though it's usually omitted
            Office::WithinOctave { .. } => true,
            Office::OctaveDay { .. } => true,
            Office::Sunday { .. } => true,
            // Christmas vigil
            Office::Vigil {
                rank: VigilRank::FirstClass,
                ..
            } => is_sunday,
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
    fn admits_translated_feast(&self, off: Office) -> bool {
        match off {
            Office::Sunday { .. } => false,
            Office::Feast(FeastDetails { rank, .. }) => rank < FeastRank::DoubleSecondClass,
            // TODO: not sure about this
            Office::WithinOctave { rank, .. } => rank <= OctaveRank::SecondOrder,
            Office::Feria { rank, .. } => rank < FeriaRank::Privileged,
            Office::Vigil { rank, .. } => rank < VigilRank::SecondClass,
            Office::AllSouls => false,
            _ => true,
        }
    }
    fn occurrence_outcome(&self, occ1: Office, occ2: Office) -> OccurrenceOutcome {
        let ord = self.compare_precedence_occ(occ1, occ2);
        let office_to_celebrate = match ord {
            // the rubrics assume there will never be occuring feasts of perfectly equal precedence
            // so how we treat that case is arbitrary
            Ordering::Greater | Ordering::Equal => OfficeIs::OfTheFirst,
            Ordering::Less => OfficeIs::OfTheSecond,
        };
        let (winner, loser) = office_to_celebrate.winner_first(occ1, occ2);
        let loser_is = if self.is_translated(loser) {
            LoserIs::Translated
            //is_sunday doesn't matter here
        } else if self.occ_admits_commemoration(winner, loser, Hour::Lauds, true) {
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
        // special case for feasts in the octave of the nativity
        if let (
            Office::Feast(FeastDetails {
                id: praec_id,
                rank: FeastRank::DoubleSecondClass,
                ..
            }),
            Office::Feast(FeastDetails {
                id: seq_id,
                rank: FeastRank::DoubleSecondClass,
                ..
            }),
        ) = (praec, seq)
        {
            if matches!(
                (praec_id, seq_id),
                ("s-stephani-protomartyris", "s-joannis-ap-ev")
                    | ("s-joannis-ap-ev", "ss-innocentium-mm")
            ) {
                return ConcurrenceOutcome {
                    office_to_celebrate: VespersIs::OfThePreceding,
                    has_comm: true,
                };
            }
            if matches!(
                (praec_id, seq_id),
                ("in-circumcisione-domini", "ss-nominis-jesu")
            ) {
                return ConcurrenceOutcome {
                    office_to_celebrate: VespersIs::OfThePreceding,
                    has_comm: false,
                };
            }
        }
        let office_to_celebrate = self.compare_precedence_conc(praec, seq);
        let has_comm = match office_to_celebrate {
            VespersIs::OfThePreceding => self.praec_admits_commemoration(praec, seq, seq_is_sunday),
            VespersIs::OfTheFollowing | VespersIs::SplitAtCap => {
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
            return VespersIs::OfThePreceding;
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
                return VespersIs::OfThePreceding;
            }
        }
        let ord = self
            .precedence_key_conc(true, praec)
            .cmp(&self.precedence_key_conc(false, seq))
            .then(self.compare_feast_precedence(praec, seq));
        match ord {
            Ordering::Greater => VespersIs::OfThePreceding,
            Ordering::Equal => VespersIs::SplitAtCap,
            Ordering::Less => VespersIs::OfTheFollowing,
        }
    }
    // Less = is commemorated first
    // (which generally means higher ranked, so we reverse it at the end)
    fn compare_commemoration_order(&self, comm1: Office, comm2: Office) -> Ordering {
        self.commemoration_ordering_key(comm1)
            .cmp(&self.commemoration_ordering_key(comm2))
            .reverse()
    }
    fn occ_admits_commemoration(
        &self,
        winner: Office,
        loser: Office,
        hour: Hour,
        is_sunday: bool,
    ) -> bool {
        if !self.wants_commemoration(loser, hour, is_sunday) {
            return false;
        }
        if loser == Office::OurLadyOnSaturday {
            // this needs to be a separate case from wants_commemoration() because OLOS is
            // sometimes commemorated in concurrence
            warn!("Our Lady on Saturday is the loser in concurrence--it shouldn't have been added to the calendar here in the first place");
            return false;
        }
        if winner.is_of_same_subject(loser) {
            return false;
        }
        match winner {
            Office::AllSouls
            | Office::Feria {
                rank: FeriaRank::DoubleFirstClass,
                ..
            } => false,
            Office::Feria { rank, .. } if rank >= FeriaRank::AnticipatedSunday => !loser.is_vigil(),
            // In 1V of the Nativity Vigil, the occurring Sunday is not commemorated
            Office::Vigil {
                rank: VigilRank::FirstClass,
                ..
            } => {
                if loser.is_sunday() {
                    hour == Hour::Lauds
                } else {
                    true
                }
            }
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                sub_rank: FeastSubRank::Primary,
                person: Person::OurLord | Person::Trinity,
                is_local: false,
                ..
            }) => match loser {
                Office::Sunday { .. } => true,
                Office::Feria { rank, .. } => rank >= FeriaRank::AnticipatedSunday,
                Office::Vigil { rank, .. } => rank >= VigilRank::SecondClass,
                Office::WithinOctave { rank, .. } => rank >= OctaveRank::ThirdOrder,
                // TODO not sure about this
                Office::OctaveDay { rank, .. } => rank >= OctaveRank::ThirdOrder,
                _ => false,
            },
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                ..
            }) => match loser {
                Office::Sunday { .. } => true,
                Office::Feria { rank, .. } => rank >= FeriaRank::AnticipatedSunday,
                Office::Vigil { rank, .. } => rank >= VigilRank::SecondClass,
                Office::WithinOctave { rank, .. } => rank >= OctaveRank::ThirdOrder,
                Office::OctaveDay { rank, .. } => rank >= OctaveRank::Common,
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::Semidouble,
                _ => false,
            },
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                ..
            }) => match loser {
                Office::Sunday { .. } => true,
                Office::Feria { rank, .. } => rank >= FeriaRank::AnticipatedSunday,
                Office::Vigil { .. } => true,
                Office::WithinOctave { rank, .. } => rank >= OctaveRank::ThirdOrder,
                Office::OctaveDay { rank, .. } => rank >= OctaveRank::Common || hour == Hour::Lauds,
                Office::Feast(FeastDetails { rank, .. }) => {
                    rank >= FeastRank::Semidouble || hour == Hour::Lauds
                }
                _ => false,
            },
            _ => true,
        }
    }
    fn praec_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool {
        assert!(self.has_second_vespers(praec));
        if !self.wants_commemoration(seq, Hour::FirstVespers, seq_is_sunday) {
            return false;
        }
        if praec.is_of_same_subject(seq) {
            return false;
        }
        if matches!(
            praec,
            Office::Feast(FeastDetails {
                id: "in-circumcisione-dnjc",
                ..
            })
        ) {
            return false;
        }
        // The Tuesdays of Easter and Pentecost admit a commemoration at Vespers of a following
        // feast
        if matches!(
            praec,
            Office::Feria {
                rank: FeriaRank::DoubleFirstClass,
                ..
            }
        ) {
            return match seq {
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::Semidouble,
                _ => false,
            };
        }
        // TODO: 1st class feasts of the Lord admit fewer commemorations (?)
        if matches!(
            praec,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass | FeastRank::DoubleSecondClass,
                ..
            })
        ) {
            // TODO: make the default case false
            return match seq {
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::Semidouble,
                Office::WithinOctave { rank, .. } => rank >= OctaveRank::ThirdOrder,
                Office::OctaveDay { rank, .. } => rank >= OctaveRank::Common,
                Office::OurLadyOnSaturday => false,
                _ => true,
            };
        }
        true
    }
    fn seq_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool {
        assert!(self.has_first_vespers(seq, seq_is_sunday));
        // is_sunday doesn't matter here
        if !self.wants_commemoration(praec, Hour::SecondVespers, false) {
            return false;
        }
        if praec.is_of_same_subject(seq) {
            return false;
        }
        match seq {
            Office::Feast(FeastDetails {
                id: "nativitas-dnjc" | "in-epiphania-dnjc",
                ..
            }) => false,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                ..
            }) => match praec {
                Office::Sunday { .. } => true,
                Office::Feria { rank, .. } => rank >= FeriaRank::ThirdClassAdvent,
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::DoubleSecondClass,
                Office::WithinOctave { rank, .. } => rank >= OctaveRank::ThirdOrder,
                Office::OctaveDay { rank, .. } => rank >= OctaveRank::ThirdOrder,
                _ => false,
            },
            Office::Feast(FeastDetails {
                id: "in-circumcisione-dnjc",
                ..
            }) => match praec {
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::DoubleSecondClass,
                _ => false,
            },
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                ..
            }) => match praec {
                Office::Sunday { .. } => true,
                Office::Feria { rank, .. } => rank >= FeriaRank::ThirdClassAdvent,
                Office::Feast(FeastDetails { rank, .. }) => rank >= FeastRank::Double,
                Office::WithinOctave { rank, .. } => rank >= OctaveRank::ThirdOrder,
                Office::OctaveDay { .. } => true,
                _ => false,
            },
            _ => true,
        }
    }
    fn anticipate_vigil(&self, rank: VigilRank) -> bool {
        rank == VigilRank::Common
    }
    fn admits_our_lady_on_saturday(&self, off: Office) -> bool {
        // TODO: confirm that simple octave days don't admit OLOS
        // the occurrence table has a 0 for that situation, implying they don't
        matches!(
            off,
            Office::Feast(FeastDetails {
                rank: FeastRank::Commemoration | FeastRank::Simple,
                ..
            })
        )
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
