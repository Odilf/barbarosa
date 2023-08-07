//! Center corner piece. See [CenterCorner] for more info.

use cartesian_array_product::cartesian_array_map;
use nalgebra::{vector, Vector3};

use crate::{
    cube_n::{
        moves::{
            rotation::{AxisRotation, Rotatable},
            wide::{DepthPiece, DepthPieceSet},
        },
        space::{Axis, Direction},
        WideAxisMove,
    },
    generic,
};

/// A center corner piece of the cube. There are 4 of these in each face of a cube.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CenterCorner {
    position: Vector3<Direction>,
    axis: Axis,
}

impl generic::Piece<24> for CenterCorner {
    type Position = Self;

    const SOLVED: [Self; 24] = {
        use Direction::*;

        const fn from_tuple(
            sp1: Direction,
            sp2: Direction,
            sp3: Direction,
            axis: Axis,
        ) -> CenterCorner {
            CenterCorner::new(vector![sp1, sp2, sp3], axis)
        }

        cartesian_array_map!(
            [Positive, Negative],
            [Positive, Negative],
            [Positive, Negative],
            [Axis::X, Axis::Y, Axis::Z];
            from_tuple
        )
    };

    const REFERENCE_POSITIONS: [Self::Position; 24] = Self::SOLVED;

    fn position(&self) -> Self::Position {
        self.clone()
    }

    fn is_solved(&self, original_pos: &Self::Position) -> bool {
        self.position[self.axis] == original_pos.position[original_pos.axis]
    }
}

impl Rotatable for CenterCorner {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.position.rotate(rotation);
        self.axis.rotate(rotation);
    }
}

impl DepthPiece<24> for CenterCorner {
    fn is_in_wide_move<const M: u32>(
        &self,
        normal_depth: u32,
        _tangent_depth: u32,
        m: &WideAxisMove<M>,
    ) -> bool {
        // If on the same direction
        if self.position[m.face().axis] == m.face().direction {
            if normal_depth <= m.depth() {
                return true;
            }

            if self.axis == m.face().axis {
                return true;
            }
        }

        false
    }
}

impl CenterCorner {
    const fn new(position: Vector3<Direction>, axis: Axis) -> Self {
        Self { position, axis }
    }

    /// Determines whether the [`CenterCorner`] is solved.
    pub fn is_solved(&self, original: &CenterCorner) -> bool {
        self.position[self.axis] == original.position[original.axis]
    }
}

/// A set of [`CenterCorner`]s with depth `ND`
pub type CenterCornerSet<const ND: u32> = DepthPieceSet<CenterCorner, 24, ND>;
