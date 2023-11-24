use super::*;

#[derive(Clone, Copy)]
struct Rubrics1962;

impl Rubrics1962 {
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
            Office::Feast(FeastDetails {
                id: "conceptio-immaculata-bmv",
                ..
            }) => 0,
            Office::Sunday {
                rank: SundayRank::FirstClass,
                ..
            } => 0,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                ..
            }) => 0,
            Office::Feria {
                rank: FeriaRank::SecondClass,
                ..
            } => 0,
            Office::Feast(FeastDetails {
                person: Person::OurLord,
                rank: FeastRank::DoubleSecondClass,
                ..
            }) => 0,
            Office::Sunday {
                rank: SundayRank::SecondClass,
                ..
            } => 0,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                is_local: false,
                ..
            }) => 0,
            Office::WithinOctave {
                rank: OctaveRank::SecondOrder,
                ..
            } => 0,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                is_local: true,
                ..
            }) => 0,
            Office::Vigil {
                rank: VigilRank::SecondClass,
                ..
            } => 0,
            Office::Feria {
                rank: FeriaRank::ThirdClass,
                ..
            } => 0,
            Office::Feast(FeastDetails {
                rank: FeastRank::Double,
                is_local: true,
                ..
            }) => 0,
            Office::Feast(FeastDetails {
                rank: FeastRank::Double,
                is_local: false,
                ..
            }) => 0,
            Office::Feria {
                rank: FeriaRank::ThirdClassAdvent,
                ..
            } => 0,
            Office::Vigil {
                // "Third class" vigil
                rank: VigilRank::Common,
                ..
            } => 0,
            Office::OurLadyOnSaturday => 0,
            Office::Feria {
                rank: FeriaRank::Common,
                ..
            } => 0,
            Office::Feast(FeastDetails {
                rank: FeastRank::Commemoration,
                ..
            }) => 0,
            Office::Empty => 0,
            _ => panic!("unexpected office in occurence: {:?}", off),
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
    fn is_translated(&self, off: Office) -> bool {
        off.feast_details()
            .map_or(false, |fd| fd.rank == FeastRank::DoubleFirstClass)
    }
    fn wants_commemoration(&self, off: Office, hour: Hour, is_sunday: bool) -> bool {
        if hour == Hour::FirstVespers && !self.has_first_vespers(off, is_sunday)
            || hour == Hour::SecondVespers && !self.has_second_vespers(off)
        {
            return false;
        }
        match off {
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
}

impl RubricsSystem for Rubrics1962 {
    fn has_first_vespers(&self, off: Office, is_sunday: bool) -> bool {
        match off {
            Office::Sunday { .. }
            | Office::Feast(FeastDetails {
                rank: FeastRank::DoubleFirstClass,
                ..
            }) => true,
            Office::Feast(FeastDetails {
                rank: FeastRank::DoubleSecondClass,
                person: Person::OurLord,
                ..
            }) => is_sunday,
            _ => false,
        }
    }
    fn has_second_vespers(&self, off: Office) -> bool {
        match off {
            Office::Feria {
                has_second_vespers, ..
            } => has_second_vespers,
            Office::WithinOctave {
                has_second_vespers, ..
            } => has_second_vespers,
            Office::Vigil {
                rank: VigilRank::SecondClass | VigilRank::FirstClass,
                ..
            } => false,
            _ => true,
        }
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
    fn occurrence_outcome(&self, occ1: Office, occ2: Office) -> OccurrenceOutcome {
        todo!()
    }
    // assumes both praec and seq are the office of their respective days
    fn concurrence_outcome(
        &self,
        praec: Office,
        seq: Office,
        seq_is_sunday: bool,
    ) -> ConcurrenceOutcome {
        todo!()
    }
    fn compare_precedence_occ(&self, occ1: Office, occ2: Office) -> Ordering {
        todo!()
    }
    fn compare_precedence_conc(&self, praec: Office, seq: Office) -> VespersIs {
        todo!()
    }
    fn compare_commemoration_order(&self, comm1: Office, comm2: Office) -> Ordering {
        todo!()
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
        if winner.is_feast_of_the_lord() && loser.is_sunday() {
            return false;
        }
        if matches!(
            winner,
            Office::Feria {
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
        true
    }
    // assuming Vespers is of praec, returns true if seq is to be commemorated
    fn praec_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool {
        todo!()
    }
    // assuming Vespers is of seq, returns true if praec is to be commemorated
    fn seq_admits_commemoration(&self, praec: Office, seq: Office, seq_is_sunday: bool) -> bool {
        todo!()
    }
    // returns true if a vigil of the given rank should be anticipated when it falls on a Sunday
    fn anticipate_vigil(&self, rank: VigilRank) -> bool {
        todo!()
    }
    fn admits_our_lady_on_saturday(&self, off: Office) -> bool {
        todo!()
    }
    fn admits_anticipated_sunday(&self, off: Office) -> bool {
        todo!()
    }
}
