mod test;

use crate::generic::{self, moves::AsMove};

use super::{
    center::{corner::CenterCornerSet, wing::CenterWingSet},
    moves::wide::impl_movable_wide_move_inductively,
    pieces::{corner::CornerSet, wing::WingSet},
    CubeN, WideAxisMove,
};

/// The 6x6x6 cube.
///
/// See [crate::cube_n] for more info.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube6 {
    corners: CornerSet,

    wings_1: WingSet<1>,
    wings_2: WingSet<2>,

    center_corners_1: CenterCornerSet<1>,
    center_corners_2: CenterCornerSet<2>,
    center_wings: CenterWingSet<1, 2>,
}

impl generic::Cube for Cube6 {
    const SOLVED: Self = Self {
        corners: CornerSet::SOLVED,

        wings_1: WingSet::SOLVED,
        wings_2: WingSet::SOLVED,

        center_corners_1: CenterCornerSet::SOLVED,
        center_corners_2: CenterCornerSet::SOLVED,
        center_wings: CenterWingSet::SOLVED,
    };

    fn is_solved(&self) -> bool
    where
        Self: 'static,
    {
        self.corners.is_solved()
            && self.wings_1.is_solved()
            && self.wings_2.is_solved()
            && self.center_corners_1.is_solved()
            && self.center_corners_2.is_solved()
            && self.center_wings.is_solved()
    }
}

impl AsMove for Cube6 {
    type Move = WideAxisMove<2>;
}

impl generic::Movable<WideAxisMove<2>> for Cube6 {
    fn apply(&mut self, m: &WideAxisMove<2>) {
        self.corners.apply(&m.axis_move);
        self.wings_1.apply(m);
        self.wings_2.apply(m);
        self.center_corners_1.apply(m);
        self.center_corners_2.apply(m);
        self.center_wings.apply(m);
    }
}

impl CubeN for Cube6 {
    const N: u32 = 6;
}

impl_movable_wide_move_inductively!(Cube6, 2, [0, 1]);
