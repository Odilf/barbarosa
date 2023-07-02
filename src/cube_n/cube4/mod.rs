//! 4x4x4 cube

mod test;

use crate::generic::{self, Alg};

use super::{
    moves::rotation::{AxisRotation, Rotatable},
    pieces::{
        center::{self, Center},
        corner, wing,
    },
    AxisMove, Corner, WideAxisMove, Wing,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cube4 {
    corners: [Corner; 8],
    wings: [Wing; 24],
    centers: [Center<Corner>; 24],
}

impl generic::Cube for Cube4 {
    fn solved() -> &'static Self {
        &SOLVED
    }

    type Move = WideAxisMove<1>;

    type Alg = Alg<Self::Move>;
}

impl generic::Movable<WideAxisMove<1>> for Cube4 {
    fn apply(&mut self, m: &WideAxisMove<1>) {
        self.corners.apply(&m.axis_move);

        for wing in &mut self.wings {
            if wing::in_wide_move(wing, 1, &m) {
                wing.rotate(&AxisRotation::from(&m.axis_move));
            }
        }

        for center in &mut self.centers {
            if center::corner_in_wide_move(center, 1, m) {
                center.rotate(&AxisRotation::from(&m.axis_move));
            }
        }
    }
}

impl generic::Movable<AxisMove> for Cube4 {
    fn apply(&mut self, m: &AxisMove) {
        let mov = m.clone().widen(0).unwrap();
        self.apply(&mov)
    }
}

const SOLVED: Cube4 = Cube4 {
    corners: corner::SOLVED,
    wings: wing::SOLVED,
    centers: center::SOLVED_CORNERS,
};
