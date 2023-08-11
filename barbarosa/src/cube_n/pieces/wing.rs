//! The wing pieces for 4x4+. See [Wing] for more info.

mod test;

use cartesian_array_product::cartesian_array_map;
use nalgebra::vector;

use crate::{
    cube_n::{
        moves::{
            rotation::{AxisRotation, Rotatable},
            wide::{DepthPiece, DepthPieceSet},
        },
        space::{Axis, Direction, Face},
        Vec2, WideAxisMove,
    },
    generic::{self, piece::PieceSetDescriptor},
};

use super::{edge::ParallelAxesError, Edge};

/// The wing pieces. These exist on 4x4x4 and up.
///
/// They are pieces that look like edges, but there are 24 of those and they are not interchangeable. They are also not orientable.
/// This means that there are 24 states of wings, and they are analogous to the 24 states of the edges (12 edges that can be either oriented
/// or not).
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Wing {
    corresponding_edge: Edge,
}

impl generic::Piece for Wing {
    type Position = Self;

    fn position(&self) -> Self::Position {
        self.clone()
    }

    fn is_solved(&self, original_pos: &Self::Position) -> bool {
        self == original_pos
    }
}

impl PieceSetDescriptor<24> for Wing {
    const SOLVED: [Wing; 24] = {
        use Axis::*;
        use Direction::*;

        const fn from_tuple(
            axis: Axis,
            sp1: Direction,
            sp2: Direction,
            pseudo_oriented: bool,
        ) -> Wing {
            Wing::new_with_orientation(axis, vector![sp1, sp2], pseudo_oriented)
        }

        cartesian_array_map!(
            [X, Y, Z],
            [Positive, Negative],
            [Positive, Negative],
            [true, false];
            from_tuple
        )
    };

    const REFERENCE_POSITIONS: [Self::Position; 24] = Self::SOLVED;
}

impl Rotatable for Wing {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.corresponding_edge.rotate(rotation);
    }
}

impl Wing {
    /// Get the normal axis
    pub fn normal_axis(&self) -> Axis {
        self.corresponding_edge.normal_axis
    }

    /// The position of the slice of the wing.
    pub fn slice_position(&self) -> Vec2 {
        self.corresponding_edge.slice_position
    }

    /// The direction along the normal of the wing
    pub fn normal_direction(&self) -> Direction {
        wing_normal_direction(
            self.normal_axis(),
            self.corresponding_edge.slice_position,
            self.corresponding_edge.oriented,
        )
    }

    /// Returns the two faces that the wing is on
    pub fn faces(&self) -> [Face; 2] {
        self.corresponding_edge.faces()
    }
}

/// Gets the wing normal direction based on a normal axis, the slice position and the pseudo orientation.
///
/// This normal direction has properties such that it is consistent with the way that edges flip orientation
/// (when doing a non double move on the X axis)
///
/// Look into the implementation for details.
pub fn wing_normal_direction(
    normal_axis: Axis,
    slice_position: Vec2,
    pseudo_oriented: bool,
) -> Direction {
    let is_z_axis = normal_axis == Axis::Z;

    let is_even_position_parity = slice_position.x == slice_position.y;

    if is_z_axis ^ is_even_position_parity ^ pseudo_oriented {
        Direction::Negative
    } else {
        Direction::Positive
    }
}

impl Wing {
    /// Creates a new [`Wing`]
    pub fn new(normal_axis: Axis, slice_position: Vec2, normal_direction: Direction) -> Self {
        let orientation =
            wing_normal_direction(normal_axis, slice_position, true) == normal_direction;

        Self {
            corresponding_edge: Edge::new(normal_axis, slice_position, orientation),
        }
    }

    const fn new_with_orientation(normal_axis: Axis, slice_position: Vec2, oriented: bool) -> Self {
        Self {
            corresponding_edge: Edge::new(normal_axis, slice_position, oriented),
        }
    }

    /// Tries to create a wing from the two faces it's on and the normal direction. Errors if the faces are parallel
    pub fn try_from_faces(
        faces: [Face; 2],
        normal_direction: Direction,
    ) -> Result<Self, ParallelAxesError> {
        let (normal_axis, slice_position) = Edge::position_from_faces(faces)?;
        Ok(Wing::new(normal_axis, slice_position, normal_direction))
    }
}

impl DepthPiece for Wing {
    fn is_in_wide_move<const M: u32>(
        &self,
        normal_depth: u32,
        _tangent_depth: u32,
        m: &WideAxisMove<M>,
    ) -> bool {
        let wing_edge = &self.corresponding_edge;
        // If just on the same face
        if m.axis_move.face.contains_edge(&self.corresponding_edge) {
            return true;
        }

        // If on parallel slices (so, same normal)
        if wing_edge.normal_axis == m.face().axis {
            // If it's on the right depth
            if normal_depth <= m.depth() && m.face().direction == self.normal_direction() {
                return true;
            }
        }

        false
    }
}

impl std::fmt::Debug for Wing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [f1, f2] = self.corresponding_edge.faces();

        write!(
            f,
            "Wing {{ faces: {f1}{f2}, normal_direction: {:?} }}",
            self.normal_direction()
        )
    }
}

/// A set of [`Wing`]s with depth `ND`
pub type WingSet<const ND: u32> = DepthPieceSet<Wing, 24, ND>;
