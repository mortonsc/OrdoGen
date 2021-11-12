use super::*;

use std::fmt;

impl fmt::Display for FeastRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Commemoration => "comm",
            Self::Simple => "sp",
            Self::Semidouble => "sd",
            Self::LesserDouble => "d",
            Self::GreaterDouble => "dm",
            Self::DoubleSecondClass => "d2cl",
            Self::DoubleFirstClass => "d1cl",
        };
        write!(f, "{}", s)
    }
}
