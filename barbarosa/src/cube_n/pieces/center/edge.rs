//! Center edge piece.
//!
//! See [`CenterEdge`] for more info.

use cartesian_array_product::cartesian_array_map;

use crate::{
    cube_n::{
        moves::{
            rotation::{AxisRotation, Rotatable},
            wide::{DepthPiece, DepthPieceSet},
        },
        pieces::edge::ParallelAxesError,
        space::{faces, Axis, Direction, Face},
        WideAxisMove,
    },
    generic::{self, piece::PieceSetDescriptor},
};

/// A center edge piece of the cube. There are 4 of these in each face of a cube.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CenterEdge {
    /// The main face of the center edge piece.
    pub main_face: Face,

    /// The handedness of the basis of the edge piece. If the handedness is positive
    /// then the side axis is the next one (so X -> Y -> Z -> X). If the handedness
    /// is negative it's the other way around.
    ///
    /// You can get the side axis using [`Axis::next_with_handedness`]
    pub handedness: Direction,

    /// The direction of the side axis
    pub side_direction: Direction,
}

impl generic::Piece for CenterEdge {
    type Position = Self;

    fn position(&self) -> Self::Position {
        self.clone()
    }

    fn is_solved(&self, original_pos: &Self::Position) -> bool {
        self.main_face == original_pos.main_face
    }
}

impl PieceSetDescriptor<24> for CenterEdge {
    const REFERENCE_POSITIONS: [Self::Position; 24] = {
        use faces::*;
        use Direction::*;

        cartesian_array_map!(
            [R, U, F, L, D, B],
            [Positive, Negative],
            [Positive, Negative];
            CenterEdge::new
        )
    };

    const SOLVED: [Self; 24] = Self::REFERENCE_POSITIONS;
}

impl Rotatable for CenterEdge {
    fn rotate(&mut self, rotation: &AxisRotation) {
        let side_face = self.side_face().rotated(rotation);

        // Very important to rotate face only *after* getting the side face.
        self.main_face.rotate(rotation);

        self.handedness = self
            .main_face
            .axis
            .get_handedness(&side_face.axis)
            .expect("Side face should be perpendicular to main face");

        self.side_direction = side_face.direction;
    }
}

impl CenterEdge {
    /// Creates a new [`CenterEdge`] piece.
    pub const fn new(main_face: Face, handedness: Direction, side_direction: Direction) -> Self {
        Self {
            main_face,
            handedness,
            side_direction,
        }
    }

    /// Tries to create a new [`CenterEdge`] piece from two faces. Fails if the axes of the faces are parallel.
    pub fn try_from_faces(main_face: Face, side_face: Face) -> Result<Self, ParallelAxesError> {
        let handedness = main_face.axis.get_handedness(&side_face.axis)?;

        Ok(CenterEdge {
            main_face,
            handedness,
            side_direction: side_face.direction,
        })
    }

    /// Gets the side face of the piece.
    pub fn side_face(&self) -> Face {
        let side_axis = self.main_face.axis.next_with_handedness(self.handedness);

        debug_assert_ne!(side_axis, self.main_face.axis);

        Face::new(side_axis, self.side_direction)
    }

    /// Gets the normal axis of the [`CenterEdge`] (so the axis that isn't the main or the side axis).
    pub fn normal_axis(&self) -> Axis {
        let output = self.main_face.axis.next_with_handedness(-self.handedness);

        assert_eq!(
            Axis::other(&self.main_face.axis, &self.side_face().axis).unwrap(),
            output
        );

        output
    }
}

impl DepthPiece for CenterEdge {
    fn is_in_wide_move<const M: u32>(
        &self,
        normal_depth: u32,
        _tangent_depth: u32,
        m: &WideAxisMove<M>,
    ) -> bool {
        if m.face() == &self.main_face {
            return true;
        }

        if m.face() == &self.side_face() && normal_depth <= M {
            return true;
        }

        false
    }
}

/// A set of [`CenterEdge`] pieces with depth `ND`
pub type CenterEdgeSet<const ND: u32> = DepthPieceSet<CenterEdge, 24, ND>;
