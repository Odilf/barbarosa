use nalgebra::vector;

use crate::{
    cube_n::{
        moves::rotation::Rotatable,
        pieces::wing::wing_normal_direction,
        space::{faces, Axis, Direction, Face},
        WideAxisMove,
    },
    generic,
};

use super::edge::CenterEdge;

/// The center-wing piece. It's the center version of the [Wing](super::Wing) piece.
///
/// [CenterEdge]s have a tangent depth and [CenterCorner]s have a normal depth. A [CenterWing] need both to
/// be identified.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CenterWing {
    corresponding_center_edge: CenterEdge,
    hypothetically_oriented: bool,
}

impl generic::Piece for CenterWing {}

impl Rotatable for CenterWing {
    fn rotate(&mut self, rotation: &crate::cube_n::moves::rotation::AxisRotation) {
        self.corresponding_center_edge.rotate(rotation);

        self.hypothetically_oriented ^= rotation.flips_edge_orientation(self.normal_axis());
    }
}

impl CenterWing {
    pub fn new(
        main_face: Face,
        handedness: Direction,
        side_direction: Direction,
        normal_direction: Direction,
    ) -> Self {
        let corresponding_edge_center = CenterEdge::new(main_face, handedness, side_direction);
        let hypothetically_oriented = wing_normal_direction(
            corresponding_edge_center.normal_axis(),
            vector![
                corresponding_edge_center.main_face.direction,
                corresponding_edge_center.side_direction
            ],
            true,
        ) == normal_direction;

        Self {
            corresponding_center_edge: corresponding_edge_center,
            hypothetically_oriented,
        }
    }

    pub const fn new_with_orientation(
        main_face: Face,
        handedness: Direction,
        side_direction: Direction,
        hypothetically_oriented: bool,
    ) -> Self {
        Self {
            corresponding_center_edge: CenterEdge::new(main_face, handedness, side_direction),
            hypothetically_oriented,
        }
    }

    pub fn main_face(&self) -> &Face {
        &self.corresponding_center_edge.main_face
    }

    pub fn side_face(&self) -> Face {
        self.corresponding_center_edge.side_face()
    }

    pub fn normal_axis(&self) -> Axis {
        self.corresponding_center_edge.normal_axis()
    }

    pub fn normal_direction(&self) -> Direction {
        wing_normal_direction(
            self.normal_axis(),
            vector![
                self.corresponding_center_edge.main_face.direction,
                self.corresponding_center_edge.side_direction
            ],
            self.hypothetically_oriented,
        )
    }

    pub fn is_solved(&self, original: &Self) -> bool {
        self.main_face() == original.main_face()
    }

    pub fn in_wide_move<const N: u32>(
        &self,
        normal_depth: u32,
        tangent_depth: u32,
        m: &WideAxisMove<N>,
    ) -> bool {
        let (main, side, mov) = (self.main_face(), &self.side_face(), m.face());

        // If directly on face
        if main == mov {
            return true;
        }

        // If it's to the side, check the normal depth
        if side == mov {
            return normal_depth <= m.depth();
        }

        // If neither of the two sides is the same as the move, then having the same axis on
        // either of them means it's the opposite face. We can discard those.
        if main.axis == mov.axis || side.axis == mov.axis {
            return false;
        }

        // Here we know that if neither of the two faces are on the axis of the move that said axis is the normal.
        debug_assert_eq!(self.normal_axis(), m.face().axis);

        self.normal_direction() == m.face().direction && tangent_depth <= m.depth()
    }
}

impl std::fmt::Debug for CenterWing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CenterWing")
            .field("main_face", &self.corresponding_center_edge.main_face)
            .field("side_face", &self.side_face())
            .field("normal_direction", &self.normal_direction())
            .field("hypo_orient", &self.hypothetically_oriented)
            .finish()
    }
}

pub const SOLVED: [CenterWing; 48] = {
    use faces::*;
    use Direction::*;

    [
        CenterWing::new_with_orientation(R, Positive, Positive, true),
        CenterWing::new_with_orientation(R, Positive, Negative, true),
        CenterWing::new_with_orientation(R, Negative, Negative, true),
        CenterWing::new_with_orientation(R, Negative, Positive, true),
        CenterWing::new_with_orientation(U, Positive, Positive, true),
        CenterWing::new_with_orientation(U, Positive, Negative, true),
        CenterWing::new_with_orientation(U, Negative, Negative, true),
        CenterWing::new_with_orientation(U, Negative, Positive, true),
        CenterWing::new_with_orientation(F, Positive, Positive, true),
        CenterWing::new_with_orientation(F, Positive, Negative, true),
        CenterWing::new_with_orientation(F, Negative, Negative, true),
        CenterWing::new_with_orientation(F, Negative, Positive, true),
        CenterWing::new_with_orientation(L, Positive, Positive, true),
        CenterWing::new_with_orientation(L, Positive, Negative, true),
        CenterWing::new_with_orientation(L, Negative, Negative, true),
        CenterWing::new_with_orientation(L, Negative, Positive, true),
        CenterWing::new_with_orientation(D, Positive, Positive, true),
        CenterWing::new_with_orientation(D, Positive, Negative, true),
        CenterWing::new_with_orientation(D, Negative, Negative, true),
        CenterWing::new_with_orientation(D, Negative, Positive, true),
        CenterWing::new_with_orientation(B, Positive, Positive, true),
        CenterWing::new_with_orientation(B, Positive, Negative, true),
        CenterWing::new_with_orientation(B, Negative, Negative, true),
        CenterWing::new_with_orientation(B, Negative, Positive, true),
        CenterWing::new_with_orientation(R, Positive, Positive, false),
        CenterWing::new_with_orientation(R, Positive, Negative, false),
        CenterWing::new_with_orientation(R, Negative, Negative, false),
        CenterWing::new_with_orientation(R, Negative, Positive, false),
        CenterWing::new_with_orientation(U, Positive, Positive, false),
        CenterWing::new_with_orientation(U, Positive, Negative, false),
        CenterWing::new_with_orientation(U, Negative, Negative, false),
        CenterWing::new_with_orientation(U, Negative, Positive, false),
        CenterWing::new_with_orientation(F, Positive, Positive, false),
        CenterWing::new_with_orientation(F, Positive, Negative, false),
        CenterWing::new_with_orientation(F, Negative, Negative, false),
        CenterWing::new_with_orientation(F, Negative, Positive, false),
        CenterWing::new_with_orientation(L, Positive, Positive, false),
        CenterWing::new_with_orientation(L, Positive, Negative, false),
        CenterWing::new_with_orientation(L, Negative, Negative, false),
        CenterWing::new_with_orientation(L, Negative, Positive, false),
        CenterWing::new_with_orientation(D, Positive, Positive, false),
        CenterWing::new_with_orientation(D, Positive, Negative, false),
        CenterWing::new_with_orientation(D, Negative, Negative, false),
        CenterWing::new_with_orientation(D, Negative, Positive, false),
        CenterWing::new_with_orientation(B, Positive, Positive, false),
        CenterWing::new_with_orientation(B, Positive, Negative, false),
        CenterWing::new_with_orientation(B, Negative, Negative, false),
        CenterWing::new_with_orientation(B, Negative, Positive, false),
    ]
};
