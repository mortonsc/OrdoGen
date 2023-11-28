use super::*;

#[derive(Clone, Copy)]
pub struct Rubrics1962;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ClassRank {
    FourthClass,
    ThirdClass,
    SecondClass,
    FirstClass,
}

impl Rubrics1962 {
    fn class_rank(&self, off: Office) -> Option<ClassRank> {
        match off {
            Office::Sunday {
                rank: SundayRank::FirstClass,
                ..
            }
            | Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                ..
            })
            | Office::WithinOctave {
                rank: OctaveRank::FirstOrder,
                ..
            }
            | Office::Feria {
                rank: FeriaRank::Privileged,
                ..
            }
            | Office::Vigil {
                rank: VigilRank::FirstClass,
                ..
            }
            | Office::AllSouls => Some(ClassRank::FirstClass),
            Office::Sunday {
                rank: SundayRank::SecondClass,
                ..
            }
            | Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                ..
            })
            | Office::WithinOctave {
                rank: OctaveRank::SecondOrder,
                ..
            }
            | Office::Feria {
                rank: FeriaRank::SecondClass,
                ..
            }
            | Office::Vigil {
                rank: VigilRank::SecondClass,
                ..
            } => Some(ClassRank::SecondClass),
            Office::Feast(FeastDetails {
                rank: FeastRank::Double,
                ..
            })
            | Office::Feria {
                rank: FeriaRank::ThirdClass | FeriaRank::ThirdClassAdvent,
                ..
            }
            | Office::Vigil {
                rank: VigilRank::Common,
                ..
            } => Some(ClassRank::ThirdClass),
            Office::Feast(FeastDetails {
                rank: FeastRank::Commemoration,
                ..
            })
            | Office::Feria {
                rank: FeriaRank::Common,
                ..
            }
            | Office::OurLadyOnSaturday => Some(ClassRank::FourthClass),
            _ => None,
        }
    }
    fn precedence_key_occ(&self, off: Office) -> u32 {
        match off {
            Office::Feria {
                rank: FeriaRank::Privileged,
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
            // There's a special ad hoc rubric making the Immaculate Conception outrank a Sunday of
            // Advent
            Office::Feast(FeastDetails {
                id: "conceptio-immaculata-bmv",
                ..
            }) => 17,
            Office::Sunday {
                rank: SundayRank::FirstClass,
                ..
            } => 16,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                ..
            }) => 15,
            Office::Feria {
                rank: FeriaRank::SecondClass,
                ..
            } => 14,
            Office::Feast(FeastDetails {
                person: Person::OurLord,
                rank: FeastRank::DoubleSecondClass,
                ..
            }) => 13,
            Office::Sunday {
                rank: SundayRank::SecondClass,
                ..
            } => 12,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                is_local: false,
                ..
            }) => 11,
            Office::WithinOctave {
                rank: OctaveRank::SecondOrder,
                ..
            } => 10,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                is_local: true,
                ..
            }) => 9,
            Office::Vigil {
                rank: VigilRank::SecondClass,
                ..
            } => 8,
            Office::Feria {
                rank: FeriaRank::ThirdClass,
                ..
            } => 7,
            Office::Feast(FeastDetails {
                rank: FeastRank::Double,
                is_local: true,
                ..
            }) => 6,
            Office::Feast(FeastDetails {
                rank: FeastRank::Double,
                is_local: false,
                ..
            }) => 5,
            Office::Feria {
                rank: FeriaRank::ThirdClassAdvent,
                ..
            } => 4,
            Office::Vigil {
                // "Third class" vigil
                rank: VigilRank::Common,
                ..
            } => 3,
            Office::OurLadyOnSaturday => 2,
            Office::Feria {
                rank: FeriaRank::Common,
                ..
            } => 1,
            Office::Feast(FeastDetails {
                rank: FeastRank::Commemoration,
                ..
            }) => 0,
            Office::Empty => 0,
            _ => panic!("unexpected office in occurrence: {:?}", off),
        }
    }
    fn compare_feast_precedence(&self, off1: Office, off2: Office) -> Ordering {
        if let (Some(fd1), Some(fd2)) = (off1.assoc_feast_details(), off2.assoc_feast_details()) {
            fd1.rank
                .cmp(&fd2.rank)
                .then(false_is_greater(fd1.is_local, fd2.is_local))
                .then(true_is_greater(fd1.is_moveable(), fd2.is_moveable()))
        } else {
            Ordering::Equal
        }
    }
    fn wants_commemoration(&self, off: Office, hour: Hour, is_sunday: bool) -> bool {
        if hour == Hour::FirstVespers && !self.has_first_vespers(off, is_sunday)
            || hour == Hour::SecondVespers && !self.has_second_vespers(off)
        {
            return false;
        }
        match off {
            Office::Empty
            | Office::OurLadyOnSaturday
            | Office::Feria {
                rank: FeriaRank::Common,
                ..
            } => false,
            _ => true,
        }
    }
    fn is_privileged_commemoration(&self, off: Office) -> bool {
        matches!(
            off,
            Office::Sunday { .. }
                | Office::Feast(FeastDetails {
                    rank: FeastRank::DoubleFirstClass,
                    ..
                })
                | Office::WithinOctave { .. }
                | Office::Feria {
                    rank: FeriaRank::ThirdClassAdvent
                        | FeriaRank::ThirdClass
                        | FeriaRank::SecondClass,
                    ..
                }
        )
    }
    fn compare_precedence_occ(&self, occ1: Office, occ2: Office) -> Ordering {
        self.precedence_key_occ(occ1)
            .cmp(&self.precedence_key_occ(occ2))
            .then(self.compare_feast_precedence(occ1, occ2))
    }
    fn is_commemoration_of_the_season(&self, comm: Office) -> bool {
        // I interpret term "commemoration of the season" to mean a feria of Advent or Lent
        // but the September ember days don't count
        // TODO: not certain of this
        let Office::Feria { id, rank, .. } = comm else {
            return false;
        };
        if rank < FeriaRank::ThirdClassAdvent {
            return false;
        }
        if id == Some("fer-4-qt-sept") || id == Some("fer-6-qt-sept") || id == Some("sab-qt-sept") {
            false
        } else {
            true
        }
    }
}

impl RubricsSystem for Rubrics1962 {
    fn has_first_vespers(&self, off: Office, is_sunday: bool) -> bool {
        match off {
            Office::Empty
            | Office::Sunday { .. }
            | Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                ..
            }) => true,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                person: Person::OurLord,
                ..
            })
            | Office::Vigil {
                rank: VigilRank::FirstClass,
                ..
            } => is_sunday,
            _ => false,
        }
    }
    fn has_second_vespers(&self, off: Office) -> bool {
        match off {
            Office::Feast(FeastDetails { rank, .. }) => rank != FeastRank::Commemoration,
            Office::Feria {
                has_second_vespers, ..
            } => has_second_vespers,
            Office::WithinOctave {
                has_second_vespers, ..
            } => has_second_vespers,
            // The "3rd class" vigil of St Lawrence has second vespers
            Office::Vigil { rank, .. } => rank == VigilRank::Common,
            Office::OurLadyOnSaturday => false,
            _ => true,
        }
    }
    fn is_translated(&self, off: Office) -> bool {
        off.feast_details()
            .map_or(false, |fd| fd.rank == FeastRank::DoubleFirstClass)
    }
    fn admits_translated_feast(&self, off: Office) -> bool {
        match off {
            Office::Sunday { .. } | Office::WithinOctave { .. } => false,
            Office::Feast(FeastDetails { rank, .. }) => rank < FeastRank::DoubleSecondClass,
            Office::Feria { rank, .. } => rank < FeriaRank::SecondClass,
            Office::Vigil { rank, .. } => rank < VigilRank::SecondClass,
            Office::AllSouls => false,
            _ => true,
        }
    }
    // assumes both praec and seq are the office of their respective days
    fn concurrence_outcome(
        &self,
        praec: Office,
        seq: Office,
        seq_is_sunday: bool,
    ) -> ConcurrenceOutcome {
        // TODO: confirm whether the octave of the Nativity requires a specical case
        assert!(self.has_second_vespers(praec));
        assert!(self.has_first_vespers(seq, seq_is_sunday));
        let office_to_celebrate = self.compare_precedence_conc(praec, seq);
        let has_comm = match office_to_celebrate {
            VespersIs::OfThePreceding => self.praec_admits_commemoration(praec, seq, seq_is_sunday),
            VespersIs::OfTheFollowing => self.seq_admits_commemoration(praec, seq, seq_is_sunday),
            VespersIs::SplitAtCap => {
                panic!("1962 rubrics don't have splitting at the cap.");
            }
        };
        ConcurrenceOutcome {
            office_to_celebrate,
            has_comm,
        }
    }
    fn compare_precedence_occ(&self, occ1: Office, occ2: Office) -> Ordering {
        self.precedence_key_occ(occ1)
            .cmp(&self.precedence_key_occ(occ2))
            .then(self.compare_feast_precedence(occ1, occ2))
    }
    fn compare_precedence_conc(&self, praec: Office, seq: Office) -> VespersIs {
        match self.compare_precedence_occ(praec, seq) {
            Ordering::Less => VespersIs::OfTheFollowing,
            Ordering::Equal | Ordering::Greater => VespersIs::OfThePreceding,
        }
    }
    fn compare_commemoration_order(&self, comm1: Office, comm2: Office) -> Ordering {
        // Commemoration "of the season" comes first
        if self.is_commemoration_of_the_season(comm1) {
            Ordering::Greater
        } else if self.is_commemoration_of_the_season(comm2) {
            Ordering::Less
        } else {
            self.compare_precedence_occ(comm1, comm2)
        }
    }
    // assuming the office of the day is winner and loser occurs on the same day,
    // returns whether loser should be commemorated at the given hour
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
        if winner.is_of_same_subject(loser) {
            return false;
        }
        if winner.is_feast_of_the_lord() && loser.is_sunday()
            || winner.is_sunday() && loser.is_feast_of_the_lord()
        {
            return false;
        }

        if matches!(
            winner,
            Office::AllSouls
                | Office::Feria {
                    rank: FeriaRank::Privileged,
                    ..
                }
        ) {
            return false;
        }
        if matches!(
            winner,
            Office::Sunday { .. }
                | Office::Feast(FeastDetails {
                    rank: FeastRank::DoubleFirstClass,
                    ..
                })
        ) && matches!(
            loser,
            Office::Vigil {
                rank: VigilRank::SecondClass | VigilRank::Common,
                ..
            }
        ) {
            return false;
        }
        if self.class_rank(winner) == Some(ClassRank::FirstClass) {
            self.is_privileged_commemoration(loser)
        } else if winner.is_sunday() {
            self.is_privileged_commemoration(loser)
                || matches!(
                    loser,
                    Office::Feast(FeastDetails {
                        rank: FeastRank::DoubleSecondClass,
                        ..
                    })
                )
        } else {
            true
        }
    }
    // assuming Vespers is of praec, returns true if seq is to be commemorated
    fn praec_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool {
        assert!(self.has_second_vespers(praec));
        if !self.wants_commemoration(seq, Hour::FirstVespers, seq_is_sunday) {
            return false;
        }
        // TODO: might need a special case saying there's no commemoration of 1V of the Holy name
        // in 2V of the circumcision
        // The rules for whether a commemoration is allowed in concurrence are the same as for
        // occurrence
        self.occ_admits_commemoration(praec, seq, Hour::Lauds, false)
    }
    // assuming Vespers is of seq, returns true if praec is to be commemorated
    fn seq_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool {
        assert!(self.has_first_vespers(seq, seq_is_sunday));
        // is_sunday is irrelevant here
        if !self.wants_commemoration(praec, Hour::SecondVespers, false) {
            return false;
        }
        self.occ_admits_commemoration(seq, praec, Hour::Lauds, false)
    }
    fn anticipate_vigil(&self, _rank: VigilRank) -> bool {
        false
    }
    fn admits_our_lady_on_saturday(&self, off: Office) -> bool {
        matches!(
            off,
            Office::Feast(FeastDetails {
                rank: FeastRank::Commemoration,
                ..
            }) | Office::Feria {
                rank: FeriaRank::Common,
                ..
            } | Office::Empty
        )
    }
    // No anticipated Sundays in the 62 rubrics
    fn admits_anticipated_sunday(&self, _off: Office) -> bool {
        false
    }
    fn n_commemorations_limit(&self, off: Office) -> Option<u32> {
        match self.class_rank(off) {
            Some(ClassRank::FirstClass | ClassRank::SecondClass) => Some(1),
            Some(ClassRank::ThirdClass | ClassRank::FourthClass) => Some(2),
            None => None,
        }
    }
}
