mod test;

use crate::generic::{self, moves::AsMove};

use super::{
    moves::wide::impl_movable_wide_move_inductively,
    pieces::{
        center::{corner::CenterCornerSet, edge::CenterEdgeSet},
        corner::CornerSet,
        edge::EdgeSet,
    },
    CubeN, WideAxisMove,
};

/// The 5x5x5 cube.
///
/// See [crate::cube_n] for more info.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube5 {
    corners: CornerSet,
    edges: EdgeSet,
    corner_centers: CenterCornerSet<1>,
    corner_edges: CenterEdgeSet<1>,
}

impl AsMove for Cube5 {
    type Move = WideAxisMove<1>;
}

impl generic::Cube for Cube5 {
    const SOLVED: Self = Self {
        corners: CornerSet::SOLVED,
        edges: EdgeSet::SOLVED,
        corner_centers: CenterCornerSet::SOLVED,
        corner_edges: CenterEdgeSet::SOLVED,
    };
}

impl generic::Movable<WideAxisMove<1>> for Cube5 {
    fn apply(&mut self, m: &WideAxisMove<1>) {
        self.corners.apply(&m.axis_move);
        self.edges.apply(&m.axis_move);
        self.corner_centers.apply(m);
        self.corner_edges.apply(m);
    }
}

impl CubeN for Cube5 {
    const N: u32 = 5;
}

impl_movable_wide_move_inductively!(Cube5, 1, [0]);
