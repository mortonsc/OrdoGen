use super::*;

use std::fmt;

impl fmt::Display for FeastRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Commemoration => "comm",
            Self::Simple => "sp",
            Self::Semidouble => "sd",
            Self::Double => "d",
            Self::GreaterDouble => "dm",
            Self::DoubleSecondClass => "d2cl",
            Self::DoubleFirstClass => "d1cl",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for SundayRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Common => "common",
            Self::WithinOctave(_) => "inf-oct",
            Self::SecondClass => "2cl",
            Self::FirstClass => "1cl",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for FeriaRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Common => "common",
            Self::AnticipatedSunday => "dom-antic",
            Self::ThirdClass => "3cl",
            Self::ThirdClassAdvent => "3cl",
            Self::SecondClass => "2cl",
            Self::Privileged => "priv",
            Self::DoubleFirstClass => "d1cl",
        };
        write!(f, "{}", s)
    }
}

impl<'a> fmt::Display for Office<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Feast(feast_details) => {
                write!(f, "{}[{}]", feast_details.id, feast_details.rank)
            }
            Self::Sunday {
                id,
                matins_id: None,
                rank,
            } => {
                write!(f, "{}[{}]", id, rank)
            }
            Self::Sunday {
                id,
                matins_id: Some(matins_id),
                rank,
            } => {
                write!(f, "{}({})[{}]", id, matins_id, rank)
            }
            Self::Feria {
                id: Some(id), rank, ..
            } => {
                write!(f, "{}[{}]", id, rank)
            }
            Self::Feria { id: None, rank, .. } => {
                write!(f, "_feria_[{}]", rank)
            }
            Self::WithinOctave { feast_details, .. } => {
                write!(f, "inf-oct[{}]", feast_details.id)
            }
            Self::OctaveDay { feast_details, .. } => {
                write!(f, "in-oct[{}]", feast_details.id)
            }
            Self::Vigil { feast_details, .. } => {
                write!(f, "in-vig[{}]", feast_details.id)
            }
            Self::AllSouls => {
                write!(f, "omnium-defunctorum")
            }
            Self::OurLadyOnSaturday => {
                write!(f, "bmv-in-sabbato")
            }
            Self::Empty => {
                write!(f, "_vacat_")
            }
        }
    }
}

impl<'a> fmt::Display for OrderedOffice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.to_commemorate.is_empty() {
            write!(f, "{}", self.office_of_day)
        } else {
            let comm_list = self
                .to_commemorate
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            write!(f, "{}; comm {}", self.office_of_day, comm_list)
        }
    }
}

impl<'a> fmt::Display for Vespers<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FirstVespers(off) => write!(f, "1V {}", off),
            Self::SecondVespers(off) => write!(f, "2V {}", off),
            Self::SplitAtCap(off1, off2) => write!(f, "2V {}, a cap 1V {}", off1, off2),
        }
    }
}

impl<'a> fmt::Display for VespersComm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FirstVespers(off) => write!(f, "1V {}", off),
            Self::SecondVespers(off) => write!(f, "2V {}", off),
        }
    }
}

impl<'a> fmt::Display for OrderedVespers<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.to_commemorate.is_empty() {
            write!(f, "{}", self.vespers)
        } else {
            let comm_list = self
                .to_commemorate
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            write!(f, "{}; comm {}", self.vespers, comm_list)
        }
    }
}
