use crate::generic;

use super::{pieces, AxisMove, Corner, Cube3};

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

    fn random_with_rng(rng: &mut impl rand::Rng) -> Self {
        Cube2 {
            corners: Cube3::random_with_rng(rng).corners,
        }
    }
}

impl generic::Movable<AxisMove> for Cube2 {
    fn apply(&mut self, m: &AxisMove) {
        self.corners.apply(m);
    }
}
