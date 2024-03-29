use rand::{distributions::Standard, prelude::Distribution};

use crate::generic::{self, moves::AsMove, Cube};

use super::{invariants::fix_corner_multiplicity, pieces::corner::CornerSet, AxisMove, CubeN};

mod test;

/// The 2x2x2 cube.
///
/// See [`crate::cube_n`] for more info.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Cube2 {
    /// Corners of the 2x2x2 cube (actually, it's the only piece type it has).
    pub corners: CornerSet,
}

impl generic::Cube for Cube2 {
    const SOLVED: Self = Self {
        corners: CornerSet::SOLVED,
    };
}

impl AsMove for Cube2 {
    type Move = AxisMove;
}

impl CubeN for Cube2 {
    const N: u32 = 2;
}

impl generic::Movable<AxisMove> for Cube2 {
    fn apply(&mut self, m: &AxisMove) {
        self.corners.apply(m);
    }
}

impl Distribution<Cube2> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Cube2 {
        let mut cube = Cube2::SOLVED;

        cube.corners.shuffle(rng);

        fix_corner_multiplicity(&mut cube.corners);

        cube
    }
}
