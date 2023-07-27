mod test;

use crate::generic::{self, moves::AsMove};

use super::{
    moves::{
        rotation::{AxisRotation, Rotatable},
        wide::impl_movable_wide_move_inductively,
    },
    pieces::{
        center::{self, edge::CenterEdge, wing::CenterWing},
        corner, edge, wing, CenterCorner,
    },
    Corner, Edge, WideAxisMove, Wing,
};

/// The 7x7x7 cube. The biggest [WCA](https://www.worldcubeassociation.org/) NxN.
///
/// It has:
/// - 8 [`Corner`]s
/// - 12 [`Edge`]s
/// - 24 [`Wing`]s at depth 1 and 24 at depth 2
/// - 24 [`CenterCorner`]s at depth 1 and another 24 at depth 2
/// - 48 [`CenterWing`]s
/// - 24 [`CenterEdge`]s at depth 1 and another 24 at depth 2
///
/// See [crate::cube_n] for more info.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube7 {
    corners: [Corner; 8],
    edges: [Edge; 12],

    wings_1: [Wing; 24],
    wings_2: [Wing; 24],

    center_corners_1: [CenterCorner; 24],
    center_corners_2: [CenterCorner; 24],
    center_wings: [CenterWing; 48],

    center_edges_1: [CenterEdge; 24],
    center_edges_2: [CenterEdge; 24],
}

impl Cube7 {
    /// Iterates through the wings of the cube with the depth
    pub fn wing_iter(&self) -> impl Iterator<Item = (&Wing, u32)> {
        self.wings_1
            .iter()
            .map(|wing| (wing, 1))
            .chain(self.wings_2.iter().map(|wing| (wing, 2)))
    }

    /// Mutable version of [`Cube7::wing_iter`]
    pub fn wing_iter_mut(&mut self) -> impl Iterator<Item = (&mut Wing, u32)> {
        self.wings_1
            .iter_mut()
            .map(|wing| (wing, 1))
            .chain(self.wings_2.iter_mut().map(|wing| (wing, 2)))
    }

    /// Iterates through the center-corners of the cube with the depth
    pub fn center_corner_iter(&self) -> impl Iterator<Item = (&CenterCorner, u32)> {
        self.center_corners_1
            .iter()
            .map(|wing| (wing, 1))
            .chain(self.center_corners_2.iter().map(|wing| (wing, 2)))
    }

    /// Mutable version of [`Cube7::center_corner_iter`]
    pub fn center_corner_iter_mut(&mut self) -> impl Iterator<Item = (&mut CenterCorner, u32)> {
        self.center_corners_1
            .iter_mut()
            .map(|wing| (wing, 1))
            .chain(self.center_corners_2.iter_mut().map(|wing| (wing, 2)))
    }

    /// Iterates through the center-edges of the cube with the depth
    pub fn center_edge_iter(&self) -> impl Iterator<Item = (&CenterEdge, u32)> {
        self.center_edges_1
            .iter()
            .map(|wing| (wing, 1))
            .chain(self.center_edges_2.iter().map(|wing| (wing, 2)))
    }

    /// Mutable version of [`Cube7::center_edge_iter`]
    pub fn center_edge_iter_mut(&mut self) -> impl Iterator<Item = (&mut CenterEdge, u32)> {
        self.center_edges_1
            .iter_mut()
            .map(|wing| (wing, 1))
            .chain(self.center_edges_2.iter_mut().map(|wing| (wing, 2)))
    }
}

impl generic::Cube for Cube7 {
    fn solved() -> &'static Self {
        &SOLVED
    }

    fn is_solved(&self) -> bool
    where
        Self: 'static,
    {
        let corners = self.corners == Self::solved().corners;
        let edges = self.edges == Self::solved().edges;
        let wings_1 = self.wings_1 == Self::solved().wings_1;
        let wings_2 = self.wings_2 == Self::solved().wings_2;

        let center_corners = self
            .center_corner_iter()
            .zip(Self::solved().center_corner_iter())
            .all(|((c, _), (o, _))| c.is_solved(o));
        let center_wings = self
            .center_wings
            .iter()
            .zip(Self::solved().center_wings.iter())
            .all(|(c, o)| c.is_solved(o));
        let center_edges = self
            .center_edge_iter()
            .zip(Self::solved().center_edge_iter())
            .all(|((c, _), (o, _))| c.is_solved(o));

        corners && edges && wings_1 && wings_2 && center_corners && center_wings && center_edges
    }
}

impl AsMove for Cube7 {
    type Move = WideAxisMove<2>;
}

impl generic::Movable<WideAxisMove<2>> for Cube7 {
    fn apply(&mut self, m: &WideAxisMove<2>) {
        self.corners.apply(&m.axis_move);
        self.edges.apply(&m.axis_move);

        self.wing_iter_mut()
            .filter(|(wing, depth)| wing.in_wide_move(*depth, m))
            .for_each(|(wing, _depth)| wing.rotate(&AxisRotation::from(&m.axis_move)));

        self.center_corner_iter_mut()
            .filter(|(cc, depth)| cc.in_wide_move(*depth, m))
            .for_each(|(cc, _)| cc.rotate(&AxisRotation::from(&m.axis_move)));

        self.center_wings
            .iter_mut()
            .filter(|cw| cw.in_wide_move(1, 2, m))
            .for_each(|cw| cw.rotate(&AxisRotation::from(&m.axis_move)));

        self.center_edge_iter_mut()
            .filter(|(ce, depth)| ce.in_wide_move(*depth, m))
            .for_each(|(ce, _)| ce.rotate(&AxisRotation::from(&m.axis_move)));
    }
}

impl_movable_wide_move_inductively!(Cube7, 2, [0, 1]);

const SOLVED: Cube7 = Cube7 {
    corners: corner::SOLVED,
    edges: edge::SOLVED,

    wings_1: wing::SOLVED,
    wings_2: wing::SOLVED,

    center_corners_1: center::corner::SOLVED,
    center_corners_2: center::corner::SOLVED,
    center_wings: center::wing::SOLVED,
    center_edges_1: center::edge::SOLVED,
    center_edges_2: center::edge::SOLVED,
};
