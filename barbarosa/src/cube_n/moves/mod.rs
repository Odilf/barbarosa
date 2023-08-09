//! Moves for NxNxN cubes

use std::mem::{self, MaybeUninit};

use itertools::iproduct;
use rand_derive2::RandGen;
use strum::IntoEnumIterator;

use self::wide::WideMoveCreationError;

use super::space::{Axis, Direction, Face};
use crate::generic;

pub mod non_redundant;
pub mod perms;
pub mod rotation;
pub mod wide;

mod amount;
mod extended;
mod quarter;
mod test;

pub use amount::Amount;
pub use extended::ExtendedAxisMove;
pub use non_redundant::NonRedundantAxisMove;
pub use quarter::QuarterAxisMove;
pub use wide::WideAxisMove;

/// A clockwise [move](generic::Move) on an axis. Main type of move for cuboids.
///
/// It can be either single, double or inverse.
///
/// Variants of axis moves are:
/// - [WideAxisMove] for big cubes (4x4 and up)
/// - [QuarterAxisMove]
/// - [NonRedundantAxisMove]
#[derive(Debug, PartialEq, Eq, Clone, RandGen)]
pub struct AxisMove {
    /// The face that is being rotated
    pub face: Face,
    /// The amount of rotation
    pub amount: Amount,
}

impl AxisMove {
    /// Creates a new [`AxisMove`]
    pub fn new(face: Face, amount: Amount) -> Self {
        Self { face, amount }
    }
}

impl generic::Move for AxisMove {
    fn inverse(&self) -> Self {
        AxisMove {
            face: self.face.clone(),
            amount: self.amount * Direction::Negative,
        }
    }
}

impl AxisMove {
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

    /// Returns the wide version of this move at the specified depth
    pub fn widen<const N: u32>(self, depth: u32) -> Result<WideAxisMove<N>, WideMoveCreationError> {
        WideAxisMove::new(self.face, self.amount, depth)
    }
}

impl std::fmt::Display for AxisMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.face, self.amount)
    }
}

impl IntoEnumIterator for AxisMove {
    type Iterator = core::array::IntoIter<Self, { Self::DISTINCT_MOVES }>;

    fn iter() -> Self::Iterator {
        Self::all().into_iter()
    }
}
