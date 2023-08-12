mod test;

use crate::generic::{self, moves::AsMove};

use super::{
    center::corner::CenterCornerSet,
    moves::wide::impl_movable_wide_move_inductively,
    pieces::{corner::CornerSet, wing::WingSet},
    CubeN, WideAxisMove,
};

/// The 4x4x4 cube.
///
/// See [`crate::cube_n`] for more info.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cube4 {
    corners: CornerSet,
    wings: WingSet<1>,
    centers: CenterCornerSet<1>,
}

impl generic::Cube for Cube4 {
    const SOLVED: Self = Self {
        corners: CornerSet::SOLVED,
        wings: WingSet::SOLVED,
        centers: CenterCornerSet::SOLVED,
    };

    fn is_solved(&self) -> bool
    where
        Self: 'static,
    {
        self.corners.is_solved() && self.wings.is_solved() && self.centers.is_solved()
    }
}

impl AsMove for Cube4 {
    type Move = WideAxisMove<1>;
}

impl generic::Movable<WideAxisMove<1>> for Cube4 {
    fn apply(&mut self, m: &WideAxisMove<1>) {
        self.corners.apply(&m.axis_move);
        self.wings.apply(m);
        self.centers.apply(m);
    }
}

impl CubeN for Cube4 {
    const N: u32 = 4;
}

impl_movable_wide_move_inductively!(Cube4, 1, [0]);
