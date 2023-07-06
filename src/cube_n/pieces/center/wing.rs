use nalgebra::vector;

use crate::{
    cube_n::{
        moves::{rotation::Rotatable, Amount},
        pieces::wing::wing_normal_direction,
        space::{faces, Axis, Direction, Face},
        WideAxisMove,
    },
    generic,
};

use super::edge::CenterEdge;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CenterWing {
    corresponding_edge_center: CenterEdge,
    hypothetically_oriented: bool,
}

impl generic::Piece for CenterWing {}

impl Rotatable for CenterWing {
    fn rotate(&mut self, rotation: &crate::cube_n::moves::rotation::AxisRotation) {
        self.corresponding_edge_center.rotate(rotation);

        if rotation.axis == Axis::X && rotation.amount != Amount::Double {
            self.hypothetically_oriented = !self.hypothetically_oriented;
        }
    }
}

impl CenterWing {
    pub fn new(
        main_face: Face,
        handedness: Direction,
        side_direction: Direction,
        normal_direction: Direction,
    ) -> Self {
        Self {
            corresponding_edge_center: CenterEdge::new(main_face, handedness, side_direction),
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
            corresponding_edge_center: CenterEdge::new(main_face, handedness, side_direction),
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

    pub fn normal_direction(&self) -> Direction {
        wing_normal_direction(
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

pub const SOLVED: [CenterWing; 48] = {
    use faces::*;
    use Direction::*;

    [
        CenterWing::new_with_orientation(R, Positive, Positive, true),
        CenterWing::new_with_orientation(R, Positive, Positive, true),
        CenterWing::new_with_orientation(R, Positive, Negative, true),
        CenterWing::new_with_orientation(R, Negative, Negative, true),
        CenterWing::new_with_orientation(U, Positive, Positive, true),
        CenterWing::new_with_orientation(U, Positive, Positive, true),
        CenterWing::new_with_orientation(U, Positive, Negative, true),
        CenterWing::new_with_orientation(U, Positive, Negative, true),
        CenterWing::new_with_orientation(F, Negative, Negative, true),
        CenterWing::new_with_orientation(F, Negative, Negative, true),
        CenterWing::new_with_orientation(F, Negative, Positive, true),
        CenterWing::new_with_orientation(F, Negative, Positive, true),
        CenterWing::new_with_orientation(L, Positive, Positive, true),
        CenterWing::new_with_orientation(L, Positive, Positive, true),
        CenterWing::new_with_orientation(L, Positive, Negative, true),
        CenterWing::new_with_orientation(L, Negative, Negative, true),
        CenterWing::new_with_orientation(D, Positive, Positive, true),
        CenterWing::new_with_orientation(D, Positive, Positive, true),
        CenterWing::new_with_orientation(D, Positive, Negative, true),
        CenterWing::new_with_orientation(D, Positive, Negative, true),
        CenterWing::new_with_orientation(B, Negative, Negative, true),
        CenterWing::new_with_orientation(B, Negative, Negative, true),
        CenterWing::new_with_orientation(B, Negative, Positive, true),
        CenterWing::new_with_orientation(B, Negative, Positive, true),
        CenterWing::new_with_orientation(R, Positive, Positive, false),
        CenterWing::new_with_orientation(R, Positive, Negative, false),
        CenterWing::new_with_orientation(R, Negative, Negative, false),
        CenterWing::new_with_orientation(R, Negative, Positive, false),
        CenterWing::new_with_orientation(U, Positive, Positive, false),
        CenterWing::new_with_orientation(U, Positive, Positive, false),
        CenterWing::new_with_orientation(U, Positive, Negative, false),
        CenterWing::new_with_orientation(U, Positive, Negative, false),
        CenterWing::new_with_orientation(F, Negative, Negative, false),
        CenterWing::new_with_orientation(F, Negative, Negative, false),
        CenterWing::new_with_orientation(F, Negative, Positive, false),
        CenterWing::new_with_orientation(F, Negative, Positive, false),
        CenterWing::new_with_orientation(L, Positive, Positive, false),
        CenterWing::new_with_orientation(L, Positive, Positive, false),
        CenterWing::new_with_orientation(L, Positive, Negative, false),
        CenterWing::new_with_orientation(L, Negative, Negative, false),
        CenterWing::new_with_orientation(D, Positive, Positive, false),
        CenterWing::new_with_orientation(D, Positive, Positive, false),
        CenterWing::new_with_orientation(D, Positive, Negative, false),
        CenterWing::new_with_orientation(D, Positive, Negative, false),
        CenterWing::new_with_orientation(B, Negative, Negative, false),
        CenterWing::new_with_orientation(B, Negative, Negative, false),
        CenterWing::new_with_orientation(B, Negative, Positive, false),
        CenterWing::new_with_orientation(B, Negative, Positive, false),
    ]
};