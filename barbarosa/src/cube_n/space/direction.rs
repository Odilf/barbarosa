use rand_derive2::RandGen;
use strum::EnumIter;

/// Spacial direction, used to indicate the two different directions along an axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, RandGen)]
#[allow(missing_docs)]
pub enum Direction {
    Positive = 1,
    Negative = -1,
}

impl From<&Direction> for i8 {
    fn from(direction: &Direction) -> i8 {
        match direction {
            Direction::Positive => 1,
            Direction::Negative => -1,
        }
    }
}

impl Direction {
    /// Helper function that just calls `i8::from(self)`
    pub fn scalar(&self) -> i8 {
        i8::from(self)
    }
}

impl std::ops::Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::Positive => Direction::Negative,
            Direction::Negative => Direction::Positive,
        }
    }
}
