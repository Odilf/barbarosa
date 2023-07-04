//! 4x4x4 cube

mod test;

use crate::generic::{self, Alg, Cube};

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

        let mut moved_corner_count = 0;

        for center in &mut self.centers {
            if center::corner_in_wide_move(center, 1, m) {
                moved_corner_count += 1;
                center.rotate(&AxisRotation::from(&m.axis_move));
            }
        }

        debug_assert_eq!(moved_corner_count, if m.depth() == 0 { 4 } else { 12 });

        let mut moved_wing_count = 0;

        for wing in &mut self.wings {
            if wing::in_wide_move(wing, 1, m) {
                wing.rotate(&AxisRotation::from(&m.axis_move));
                moved_wing_count += 1;
            }
        }

        debug_assert_eq!(moved_wing_count, if m.depth() == 0 { 8 } else { 12 });
    }
}

impl Cube4 {
    /// Assert that the 4x4 cube is in a consistent state. Right now it only checks that it contains all wings
    pub fn assert_consistent(&self) {
        for solved_wing in &Cube4::solved().wings {
            assert!(
                self.wings.contains(solved_wing),
                "{:#?} is missing",
                solved_wing
            )
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
