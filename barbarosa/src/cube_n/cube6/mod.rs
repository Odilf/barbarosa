mod test;

use crate::generic::{self, moves::IntoMove};

use super::{
    moves::{rotation::{AxisRotation, Rotatable}, wide::impl_movable_wide_move_inductively},
    pieces::{
        center::{self, wing::CenterWing},
        corner, wing, CenterCorner,
    },
    Corner, WideAxisMove, Wing,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube6 {
    corners: [Corner; 8],

    wings_1: [Wing; 24],
    wings_2: [Wing; 24],

    center_corners_1: [CenterCorner; 24],
    center_corners_2: [CenterCorner; 24],
    center_wings: [CenterWing; 48],
}

impl Cube6 {
    pub fn wing_iter(&self) -> impl Iterator<Item = (&Wing, u32)> {
        self.wings_1
            .iter()
            .map(|wing| (wing, 1))
            .chain(self.wings_2.iter().map(|wing| (wing, 2)))
    }

    pub fn wing_iter_mut(&mut self) -> impl Iterator<Item = (&mut Wing, u32)> {
        self.wings_1
            .iter_mut()
            .map(|wing| (wing, 1))
            .chain(self.wings_2.iter_mut().map(|wing| (wing, 2)))
    }

    pub fn center_corner_iter(&self) -> impl Iterator<Item = (&CenterCorner, u32)> {
        self.center_corners_1
            .iter()
            .map(|wing| (wing, 1))
            .chain(self.center_corners_2.iter().map(|wing| (wing, 2)))
    }

    pub fn center_corner_iter_mut(&mut self) -> impl Iterator<Item = (&mut CenterCorner, u32)> {
        self.center_corners_1
            .iter_mut()
            .map(|wing| (wing, 1))
            .chain(self.center_corners_2.iter_mut().map(|wing| (wing, 2)))
    }
}

impl generic::Cube for Cube6 {
    fn solved() -> &'static Self {
        &SOLVED
    }

    fn is_solved(&self) -> bool
    where
        Self: 'static,
    {
        let corners = self.corners == Self::solved().corners;
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

        corners && wings_1 && wings_2 && center_corners && center_wings
    }
}

impl IntoMove for Cube6 {
    type Move = WideAxisMove<2>;
}

impl generic::Movable<WideAxisMove<2>> for Cube6 {
    fn apply(&mut self, m: &WideAxisMove<2>) {
        self.corners.apply(&m.axis_move);

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
    }
}

impl_movable_wide_move_inductively!(Cube6, 2, [0, 1]);

const SOLVED: Cube6 = Cube6 {
    corners: corner::SOLVED,

    wings_1: wing::SOLVED,
    wings_2: wing::SOLVED,

    center_corners_1: center::corner::SOLVED,
    center_corners_2: center::corner::SOLVED,
    center_wings: center::wing::SOLVED,
};
