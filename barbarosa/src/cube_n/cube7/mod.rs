mod test;

use crate::generic::{self, moves::AsMove};

use super::{
    center::{corner::CenterCornerSet, edge::CenterEdgeSet, wing::CenterWingSet},
    moves::wide::impl_movable_wide_move_inductively,
    pieces::{corner::CornerSet, edge::EdgeSet, wing::WingSet},
    CubeN, WideAxisMove,
};

/// The 7x7x7 cube. The biggest [WCA](https://www.worldcubeassociation.org/) NxN.
///
/// See [crate::cube_n] for more info.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube7 {
    corners: CornerSet,
    edges: EdgeSet,

    wings_1: WingSet<1>,
    wings_2: WingSet<2>,

    center_corners_1: CenterCornerSet<1>,
    center_corners_2: CenterCornerSet<2>,
    center_wings: CenterWingSet<1, 2>,

    center_edges_1: CenterEdgeSet<1>,
    center_edges_2: CenterEdgeSet<2>,
}

impl generic::Cube for Cube7 {
    fn solved() -> &'static Self {
        &SOLVED
    }

    fn is_solved(&self) -> bool
    where
        Self: 'static,
    {
        self.corners.is_solved()
            && self.edges.is_solved()
            && self.wings_1.is_solved()
            && self.wings_2.is_solved()
            && self.center_corners_1.is_solved()
            && self.center_corners_2.is_solved()
            && self.center_wings.is_solved()
            && self.center_edges_1.is_solved()
            && self.center_edges_2.is_solved()
    }
}

impl AsMove for Cube7 {
    type Move = WideAxisMove<2>;
}

impl generic::Movable<WideAxisMove<2>> for Cube7 {
    fn apply(&mut self, m: &WideAxisMove<2>) {
        self.corners.apply(&m.axis_move);
        self.edges.apply(&m.axis_move);
        self.wings_1.apply(m);
        self.wings_2.apply(m);
        self.center_corners_1.apply(m);
        self.center_corners_2.apply(m);
        self.center_wings.apply(m);
        self.center_edges_1.apply(m);
        self.center_edges_2.apply(m);
    }
}

impl CubeN for Cube7 {}

impl_movable_wide_move_inductively!(Cube7, 2, [0, 1]);

const SOLVED: Cube7 = Cube7 {
    corners: CornerSet::SOLVED,
    edges: EdgeSet::SOLVED,

    wings_1: WingSet::SOLVED,
    wings_2: WingSet::SOLVED,

    center_corners_1: CenterCornerSet::SOLVED,
    center_corners_2: CenterCornerSet::SOLVED,
    center_wings: CenterWingSet::SOLVED,

    center_edges_1: CenterEdgeSet::SOLVED,
    center_edges_2: CenterEdgeSet::SOLVED,
};
