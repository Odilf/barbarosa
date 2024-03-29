use rand_derive2::RandGen;
use strum::EnumIter;

use crate::cube_n::space::Direction;

/// A move amount (either single, double or reverse)
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, RandGen)]
#[allow(missing_docs)]
pub enum Amount {
    Single = 1,
    Double = 2,
    Inverse = 3,
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amount::Single => write!(f, ""),
            Amount::Double => write!(f, "2"),
            Amount::Inverse => write!(f, "'"),
        }
    }
}

impl std::ops::Mul<Direction> for Amount {
    type Output = Amount;

    /// Multiply an amount by a direction to get the resulting amount.
    fn mul(self, rhs: Direction) -> Self::Output {
        use Amount::*;
        use Direction::*;

        match (self, rhs) {
            (Single, Positive) | (Inverse, Negative) => Single,
            (Double, _) => Double,
            (Inverse, Positive) | (Single, Negative) => Inverse,
        }
    }
}

impl std::ops::Add<Amount> for Amount {
    type Output = Option<Amount>;

    fn add(self, rhs: Amount) -> Self::Output {
        match (self as u32 + rhs as u32).rem_euclid(4) {
            0 => None,
            1 => Some(Amount::Single),
            2 => Some(Amount::Double),
            3 => Some(Amount::Inverse),
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for Amount {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Positive => Amount::Single,
            Direction::Negative => Amount::Inverse,
        }
    }
}
