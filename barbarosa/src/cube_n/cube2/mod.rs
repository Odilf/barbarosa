//! 2x2x2 cube

use rand::{distributions::Standard, prelude::Distribution, seq::SliceRandom};

use crate::generic::{self, moves::IntoMove, Cube};

use super::{invariants::fix_corner_multiplicity, pieces, AxisMove, Corner};

mod test;

/// The 2x2x2 cube
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Cube2 {
    /// Corners of the 2x2x2 cube. Actually, it's the only piece type it has.
    pub corners: [Corner; 8],
}

const SOLVED_CUBE: Cube2 = Cube2 {
    corners: pieces::corner::SOLVED,
};

impl generic::Cube for Cube2 {
    fn solved() -> &'static Self {
        &SOLVED_CUBE
    }
}

impl IntoMove for Cube2 {
    type Move = AxisMove;
}

impl generic::Movable<AxisMove> for Cube2 {
    fn apply(&mut self, m: &AxisMove) {
        self.corners.apply(m);
    }
}

impl Distribution<Cube2> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Cube2 {
        let mut cube = Cube2::new_solved();

        cube.corners.shuffle(rng);

        fix_corner_multiplicity(&mut cube.corners);

        cube
    }
}
