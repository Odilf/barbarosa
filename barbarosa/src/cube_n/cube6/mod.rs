mod test;

use crate::generic::{self, moves::AsMove};

use super::{
    center::{corner::CenterCornerSet, wing::CenterWingSet},
    moves::wide::impl_movable_wide_move_inductively,
    pieces::{corner::CornerSet, wing::WingSet},
    WideAxisMove,
};

/// The 6x6x6 cube.
///
/// It has 8 [`Corner`]s, 24 [`Wing`]s at depth 1 and 24 at depth 2, 24 [`CenterCorner`]s
/// at depth 1 and another 24 at depth 2, and 48 [`CenterWing`]s.
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
    fn solved() -> &'static Self {
        &SOLVED
    }

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

impl_movable_wide_move_inductively!(Cube6, 2, [0, 1]);

const SOLVED: Cube6 = Cube6 {
    corners: CornerSet::SOLVED,

    wings_1: WingSet::SOLVED,
    wings_2: WingSet::SOLVED,

    center_corners_1: CenterCornerSet::SOLVED,
    center_corners_2: CenterCornerSet::SOLVED,
    center_wings: CenterWingSet::SOLVED,
};
