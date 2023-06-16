//! Moves for NxNxN cubes

use std::mem::{self, MaybeUninit};

use crate::generic::{self, parse, Parsable};

mod amount;
mod rotation;
mod test;

pub mod perms;

pub use amount::Amount;
use rand_derive2::RandGen;
pub use rotation::{AxisRotation, Rotatable};

use itertools::iproduct;
use strum::IntoEnumIterator;

use super::{
    space::{Axis, Direction, Face},
    Corner, Edge,
};

/// A move on the 3x3x3 cube
#[derive(Debug, PartialEq, Eq, Clone, RandGen)]
pub struct AxisMove {
    /// The face that is being rotated
    pub face: Face,
    /// The amount of rotation
    pub amount: Amount,
}

impl generic::Move for AxisMove {
    fn inverse(&self) -> Self {
        AxisMove {
            face: self.face.clone(),
            amount: self.amount * Direction::Negative,
        }
    }

    type Iter = std::array::IntoIter<AxisMove, 18>;

    fn iter() -> Self::Iter {
        Self::all().into_iter()
    }
}

impl AxisMove {
    /// Creates a new [AxisMove]
    pub fn new(face: Face, amount: Amount) -> Self {
        Self { face, amount }
    }

    const DISTINCT_MOVES: usize = 3 * 2 * 3;

    /// Returns an array of all moves
    pub fn all() -> [Self; Self::DISTINCT_MOVES] {
        let mut moves: [MaybeUninit<Self>; Self::DISTINCT_MOVES] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for (i, (axis, direction, amount)) in
            iproduct!(Axis::iter(), Direction::iter(), Amount::iter()).enumerate()
        {
            let mov = Self::new(Face::new(axis, direction), amount);
            moves[i].write(mov);
        }

        unsafe {
            mem::transmute::<[MaybeUninit<Self>; Self::DISTINCT_MOVES], [Self; Self::DISTINCT_MOVES]>(
                moves,
            )
        }
    }
}

impl generic::Movable<AxisMove> for Edge {
    fn apply_move(&mut self, mov: &AxisMove) {
        if !mov.face.contains_edge(self) {
            return;
        }

        let rotation = AxisRotation {
            axis: mov.face.axis,
            amount: mov.amount * mov.face.direction,
        };

        self.rotate(&rotation);
    }
}

impl generic::Movable<AxisMove> for Corner {
    fn apply_move(&mut self, mov: &AxisMove) {
        if !mov.face.contains_corner(self) {
            return;
        }

        let rotation = AxisRotation {
            axis: mov.face.axis,
            amount: mov.amount * mov.face.direction,
        };

        self.rotate(&rotation);
    }
}

impl Parsable for AxisMove {
    fn parse(s: &str) -> parse::Result<Self> {
        let mut chars = s.chars();
        let face = chars.next().ok_or(parse::Error::UnexpectedEnd)?;
        let amount = chars.next();

        if let Some(next) = chars.next() {
            return Err(parse::Error::ExpectedEnd(next));
        }

        let face = Face::parse(face)?;
        let amount = Amount::parse(amount)?;

        Ok(AxisMove { face, amount })
    }
}

impl std::fmt::Display for AxisMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.face, self.amount)
    }
}
