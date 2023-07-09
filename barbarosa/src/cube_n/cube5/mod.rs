mod test;

use crate::generic::{self, moves::IntoMove};

use super::{
    moves::{
        rotation::{AxisRotation, Rotatable},
        wide::impl_movable_wide_move_inductively,
    },
    pieces::{
        center::{self, edge::CenterEdge},
        corner, edge, CenterCorner,
    },
    Corner, Edge, WideAxisMove,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube5 {
    corners: [Corner; 8],
    edges: [Edge; 12],
    corner_centers: [CenterCorner; 24],
    corner_edges: [CenterEdge; 24],
}

const SOLVED: Cube5 = Cube5 {
    corners: corner::SOLVED,
    edges: edge::SOLVED,
    corner_centers: center::corner::SOLVED,
    corner_edges: center::edge::SOLVED,
};

impl IntoMove for Cube5 {
    type Move = WideAxisMove<1>;
}

impl generic::Cube for Cube5 {
    fn solved() -> &'static Self {
        &SOLVED
    }
}

impl generic::Movable<WideAxisMove<1>> for Cube5 {
    fn apply(&mut self, m: &WideAxisMove<1>) {
        self.corners.apply(&m.axis_move);
        self.edges.apply(&m.axis_move);

        self.corner_centers
            .iter_mut()
            .filter(|cc| cc.in_wide_move(1, m))
            .for_each(|cc| cc.rotate(&AxisRotation::from(&m.axis_move)));

        self.corner_edges
            .iter_mut()
            .filter(|ce| ce.in_wide_move(1, m))
            .for_each(|ce| ce.rotate(&AxisRotation::from(&m.axis_move)));
    }
}

impl_movable_wide_move_inductively!(Cube5, 1, [0]);
