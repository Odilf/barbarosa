//! The center-wing piece. See [`CenterWing`] for more info.

use cartesian_array_product::cartesian_array_map;
use nalgebra::vector;

use crate::{
    cube_n::{
        moves::{
            rotation::Rotatable,
            wide::{DepthPiece, DepthPieceSet},
        },
        pieces::wing::wing_normal_direction,
        space::{faces, Axis, Direction, Face},
        WideAxisMove,
    },
    generic::{self, piece::PieceSetDescriptor},
};

use super::edge::CenterEdge;

/// The center-wing piece. It's the center version of the [Wing](crate::cube_n::Wing) piece.
///
/// [CenterEdge]s have a tangent depth and [CenterCorner](super::corner::CenterCorner)s have a normal depth. A [CenterWing] need both to
/// be identified.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CenterWing {
    corresponding_center_edge: CenterEdge,
    pseudo_oriented: bool,
}

impl generic::Piece for CenterWing {
    type Position = Self;

    fn position(&self) -> Self::Position {
        self.clone()
    }

    fn is_solved(&self, original_pos: &Self::Position) -> bool {
        self.main_face() == original_pos.main_face()
    }
}

impl PieceSetDescriptor<48> for CenterWing {
    const REFERENCE_POSITIONS: [Self::Position; 48] = {
        use faces::*;
        use Direction::*;

        cartesian_array_map!(
            [R, U, F, L, D, B],
            [Positive, Negative],
            [Positive, Negative],
            [true, false];
            CenterWing::new_with_orientation
        )
    };

    const SOLVED: [Self; 48] = Self::REFERENCE_POSITIONS;
}

impl Rotatable for CenterWing {
    fn rotate(&mut self, rotation: &crate::cube_n::moves::rotation::AxisRotation) {
        self.corresponding_center_edge.rotate(rotation);

        self.pseudo_oriented ^= rotation.flips_edge_orientation(self.normal_axis());
    }
}

impl CenterWing {
    /// Creates a new [`CenterWing`]
    pub fn new(
        main_face: Face,
        handedness: Direction,
        side_direction: Direction,
        normal_direction: Direction,
    ) -> Self {
        let corresponding_edge_center = CenterEdge::new(main_face, handedness, side_direction);
        let pseudo_oriented = wing_normal_direction(
            corresponding_edge_center.normal_axis(),
            vector![
                corresponding_edge_center.main_face.direction,
                corresponding_edge_center.side_direction
            ],
            true,
        ) == normal_direction;

        Self {
            corresponding_center_edge: corresponding_edge_center,
            pseudo_oriented,
        }
    }

    /// Create a new [`CenterWing`] using orientation instead of side directions and what not
    pub const fn new_with_orientation(
        main_face: Face,
        handedness: Direction,
        side_direction: Direction,
        pseudo_oriented: bool,
    ) -> Self {
        Self {
            corresponding_center_edge: CenterEdge::new(main_face, handedness, side_direction),
            pseudo_oriented,
        }
    }

    /// The main face
    pub fn main_face(&self) -> &Face {
        &self.corresponding_center_edge.main_face
    }

    /// The side face
    pub fn side_face(&self) -> Face {
        self.corresponding_center_edge.side_face()
    }

    /// The normal axis
    pub fn normal_axis(&self) -> Axis {
        self.corresponding_center_edge.normal_axis()
    }

    /// The normal direction
    pub fn normal_direction(&self) -> Direction {
        wing_normal_direction(
            self.normal_axis(),
            vector![
                self.corresponding_center_edge.main_face.direction,
                self.corresponding_center_edge.side_direction
            ],
            self.pseudo_oriented,
        )
    }
}

impl std::fmt::Debug for CenterWing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CenterWing")
            .field("main_face", &self.corresponding_center_edge.main_face)
            .field("side_face", &self.side_face())
            .field("normal_direction", &self.normal_direction())
            .field("hypo_orient", &self.pseudo_oriented)
            .finish()
    }
}

impl DepthPiece for CenterWing {
    fn is_in_wide_move<const M: u32>(
        &self,
        normal_depth: u32,
        tangent_depth: u32,
        m: &WideAxisMove<M>,
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

/// A set of [`CenterWing`]s with normal depth `ND` and tangent depth `TD`
pub type CenterWingSet<const ND: u32, const TD: u32> = DepthPieceSet<CenterWing, 48, ND, TD>;
