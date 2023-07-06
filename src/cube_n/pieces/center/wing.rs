use nalgebra::vector;

use crate::{
    cube_n::{
        moves::{rotation::Rotatable, Amount},
        pieces::wing::wing_direction_along_normal,
        space::{faces, Axis, Direction, Face},
        WideAxisMove,
    },
    generic,
};

use super::edge::EdgeCenter;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WingCenter {
    corresponding_edge_center: EdgeCenter,
    hypothetically_oriented: bool,
}

impl generic::Piece for WingCenter {}

impl Rotatable for WingCenter {
    fn rotate(&mut self, rotation: &crate::cube_n::moves::rotation::AxisRotation) {
        self.corresponding_edge_center.rotate(rotation);

        if rotation.axis == Axis::X && rotation.amount != Amount::Double {
            self.hypothetically_oriented = !self.hypothetically_oriented;
        }
    }
}

impl WingCenter {
    pub fn new(
        main_face: Face,
        handedness: Direction,
        side_direction: Direction,
        normal_direction: Direction,
    ) -> Self {
        Self {
            corresponding_edge_center: EdgeCenter::new(main_face, handedness, side_direction),
            hypothetically_oriented: normal_direction == Direction::Positive,
        }
    }

    pub const fn new_with_orientation(
        main_face: Face,
        handedness: Direction,
        side_direction: Direction,
        hypothetically_oriented: bool,
    ) -> Self {
        Self {
            corresponding_edge_center: EdgeCenter::new(main_face, handedness, side_direction),
            hypothetically_oriented,
        }
    }

    pub fn main_face(&self) -> &Face {
        &self.corresponding_edge_center.main_face
    }

    pub fn side_face(&self) -> Face {
        self.corresponding_edge_center.side_face()
    }

    pub fn normal_axis(&self) -> Axis {
        self.main_face()
            .axis
            .next_with_handedness(-self.corresponding_edge_center.handedness)
    }

    pub fn direction_along_normal(&self) -> Direction {
        wing_direction_along_normal(
            self.normal_axis(),
            vector![
                self.corresponding_edge_center.side_direction,
                self.corresponding_edge_center.handedness
            ],
            self.hypothetically_oriented,
        )
    }

    pub fn is_solved(&self, original: &Self) -> bool {
        self.main_face() == original.main_face()
    }

    pub fn in_wide_move<const N: u32>(&self, piece_depth: u32, m: &WideAxisMove<N>) -> bool {
        if self.main_face() == m.face() {
            return true;
        };

        if &self.side_face() == m.face() && piece_depth <= N {
            return true;
        };

        false
    }
}

pub const SOLVED: [WingCenter; 48] = {
    use faces::*;
    use Direction::*;

    [
        WingCenter::new_with_orientation(R, Positive, Positive, true),
        WingCenter::new_with_orientation(R, Positive, Positive, true),
        WingCenter::new_with_orientation(R, Positive, Negative, true),
        WingCenter::new_with_orientation(R, Negative, Negative, true),
        WingCenter::new_with_orientation(U, Positive, Positive, true),
        WingCenter::new_with_orientation(U, Positive, Positive, true),
        WingCenter::new_with_orientation(U, Positive, Negative, true),
        WingCenter::new_with_orientation(U, Positive, Negative, true),
        WingCenter::new_with_orientation(F, Negative, Negative, true),
        WingCenter::new_with_orientation(F, Negative, Negative, true),
        WingCenter::new_with_orientation(F, Negative, Positive, true),
        WingCenter::new_with_orientation(F, Negative, Positive, true),
        WingCenter::new_with_orientation(L, Positive, Positive, true),
        WingCenter::new_with_orientation(L, Positive, Positive, true),
        WingCenter::new_with_orientation(L, Positive, Negative, true),
        WingCenter::new_with_orientation(L, Negative, Negative, true),
        WingCenter::new_with_orientation(D, Positive, Positive, true),
        WingCenter::new_with_orientation(D, Positive, Positive, true),
        WingCenter::new_with_orientation(D, Positive, Negative, true),
        WingCenter::new_with_orientation(D, Positive, Negative, true),
        WingCenter::new_with_orientation(B, Negative, Negative, true),
        WingCenter::new_with_orientation(B, Negative, Negative, true),
        WingCenter::new_with_orientation(B, Negative, Positive, true),
        WingCenter::new_with_orientation(B, Negative, Positive, true),
        WingCenter::new_with_orientation(R, Positive, Positive, false),
        WingCenter::new_with_orientation(R, Positive, Negative, false),
        WingCenter::new_with_orientation(R, Negative, Negative, false),
        WingCenter::new_with_orientation(R, Negative, Positive, false),
        WingCenter::new_with_orientation(U, Positive, Positive, false),
        WingCenter::new_with_orientation(U, Positive, Positive, false),
        WingCenter::new_with_orientation(U, Positive, Negative, false),
        WingCenter::new_with_orientation(U, Positive, Negative, false),
        WingCenter::new_with_orientation(F, Negative, Negative, false),
        WingCenter::new_with_orientation(F, Negative, Negative, false),
        WingCenter::new_with_orientation(F, Negative, Positive, false),
        WingCenter::new_with_orientation(F, Negative, Positive, false),
        WingCenter::new_with_orientation(L, Positive, Positive, false),
        WingCenter::new_with_orientation(L, Positive, Positive, false),
        WingCenter::new_with_orientation(L, Positive, Negative, false),
        WingCenter::new_with_orientation(L, Negative, Negative, false),
        WingCenter::new_with_orientation(D, Positive, Positive, false),
        WingCenter::new_with_orientation(D, Positive, Positive, false),
        WingCenter::new_with_orientation(D, Positive, Negative, false),
        WingCenter::new_with_orientation(D, Positive, Negative, false),
        WingCenter::new_with_orientation(B, Negative, Negative, false),
        WingCenter::new_with_orientation(B, Negative, Negative, false),
        WingCenter::new_with_orientation(B, Negative, Positive, false),
        WingCenter::new_with_orientation(B, Negative, Positive, false),
    ]
};
