use rand_derive2::RandGen;

use crate::{
    cube_n::space::{Direction, Face},
    generic::{self, Alg, Cube, Movable},
};

use super::{Amount, AxisMove};

/// Axis move for quarter turn metric. Same as [`AxisMove`], but without double moves.
///
/// You can always convert from `QuarterAxisMove` into `AxisMove` but not vice versa.
/// However, you can convert from `Alg<AxisMove>` into `Alg<QuarterAxisMove>` (and of cource vice versa._
#[derive(Debug, Clone, PartialEq, Eq, RandGen)]
pub struct QuarterAxisMove {
    /// The face that is being rotated
    pub face: Face,

    /// The direction (either clockwise if positive or negative if counterclockwise)
    pub direction: Direction,
}

impl generic::Move for QuarterAxisMove {
    fn inverse(&self) -> Self {
        QuarterAxisMove {
            face: self.face.clone(),
            direction: -self.direction,
        }
    }
}

impl QuarterAxisMove {
    /// Creates a new [`QuarterAxisMove`]
    pub const fn new(face: Face, direction: Direction) -> Self {
        Self { face, direction }
    }
}

impl From<&QuarterAxisMove> for AxisMove {
    fn from(value: &QuarterAxisMove) -> Self {
        AxisMove {
            face: value.face.clone(),
            amount: match value.direction {
                Direction::Positive => Amount::Single,
                Direction::Negative => Amount::Inverse,
            },
        }
    }
}

impl<C: Cube + Movable<AxisMove>> generic::Movable<QuarterAxisMove> for C {
    fn apply(&mut self, m: &QuarterAxisMove) {
        self.apply(&AxisMove::from(m));
    }
}

impl From<Alg<AxisMove>> for Alg<QuarterAxisMove> {
    fn from(value: Alg<AxisMove>) -> Self {
        let mut output_moves = Vec::new();

        for mov in value.moves {
            match mov.amount {
                Amount::Single => {
                    output_moves.push(QuarterAxisMove::new(mov.face, Direction::Positive))
                }
                Amount::Inverse => {
                    output_moves.push(QuarterAxisMove::new(mov.face, Direction::Negative))
                }
                Amount::Double => {
                    output_moves.push(QuarterAxisMove::new(mov.face.clone(), Direction::Positive));
                    output_moves.push(QuarterAxisMove::new(mov.face, Direction::Positive));
                }
            }
        }

        Alg::new(output_moves)
    }
}
