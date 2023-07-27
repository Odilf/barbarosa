//! The wing pieces for 4x4+. See [Wing] for more info.

mod test;

use nalgebra::{vector, Vector2};

use crate::{
    cube_n::{
        moves::rotation::{AxisRotation, Rotatable},
        space::{Axis, Direction, Face},
        WideAxisMove,
    },
    generic,
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

impl generic::Piece for Wing {}

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
    pub fn slice_position(&self) -> Vector2<Direction> {
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

/// Gets the wing normal direction based on a normal axis, the slice position and the hypothetical orientation.
///
/// This normal direction has properties such that it is consistent with the way that edges flip orientation
/// (when doing a non double move on the X axis)
///
/// Look into the implementation for details.
pub fn wing_normal_direction(
    normal_axis: Axis,
    slice_position: Vector2<Direction>,
    hypothetically_oriented: bool,
) -> Direction {
    let is_x_axis = normal_axis == Axis::X;

    let is_even_position_parity = slice_position.x == slice_position.y;

    if is_x_axis ^ is_even_position_parity ^ hypothetically_oriented {
        Direction::Negative
    } else {
        Direction::Positive
    }
}

impl Wing {
    /// Creates a new [`Wing`]
    pub fn new(
        normal_axis: Axis,
        slice_position: Vector2<Direction>,
        normal_direction: Direction,
    ) -> Self {
        let orientation =
            wing_normal_direction(normal_axis, slice_position, true) == normal_direction;

        Self {
            corresponding_edge: Edge::new(normal_axis, slice_position, orientation),
        }
    }

    const fn new_with_orientation(
        normal_axis: Axis,
        slice_position: Vector2<Direction>,
        oriented: bool,
    ) -> Self {
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

    /// Determines whether the wing is in a wide move
    pub fn in_wide_move<const N: u32>(&self, wing_depth: u32, m: &WideAxisMove<N>) -> bool {
        let wing_edge = &self.corresponding_edge;
        // If just on the same face
        if m.axis_move.face.contains_edge(&self.corresponding_edge) {
            return true;
        }

        // If on parallel slices (so, same normal)
        if wing_edge.normal_axis == m.face().axis {
            // If it's on the right depth
            if wing_depth <= m.depth() && m.face().direction == self.normal_direction() {
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

/// The solved set of [`Wing`]s.
pub const SOLVED: [Wing; 24] = {
    use Axis::*;
    use Direction::*;

    [
        Wing::new_with_orientation(X, vector![Positive, Positive], true),
        Wing::new_with_orientation(X, vector![Positive, Negative], true),
        Wing::new_with_orientation(X, vector![Negative, Negative], true),
        Wing::new_with_orientation(X, vector![Negative, Positive], true),
        Wing::new_with_orientation(Y, vector![Positive, Positive], true),
        Wing::new_with_orientation(Y, vector![Positive, Negative], true),
        Wing::new_with_orientation(Y, vector![Negative, Negative], true),
        Wing::new_with_orientation(Y, vector![Negative, Positive], true),
        Wing::new_with_orientation(Z, vector![Positive, Positive], true),
        Wing::new_with_orientation(Z, vector![Positive, Negative], true),
        Wing::new_with_orientation(Z, vector![Negative, Negative], true),
        Wing::new_with_orientation(Z, vector![Negative, Positive], true),
        Wing::new_with_orientation(X, vector![Positive, Positive], false),
        Wing::new_with_orientation(X, vector![Positive, Negative], false),
        Wing::new_with_orientation(X, vector![Negative, Negative], false),
        Wing::new_with_orientation(X, vector![Negative, Positive], false),
        Wing::new_with_orientation(Y, vector![Positive, Positive], false),
        Wing::new_with_orientation(Y, vector![Positive, Negative], false),
        Wing::new_with_orientation(Y, vector![Negative, Negative], false),
        Wing::new_with_orientation(Y, vector![Negative, Positive], false),
        Wing::new_with_orientation(Z, vector![Positive, Positive], false),
        Wing::new_with_orientation(Z, vector![Positive, Negative], false),
        Wing::new_with_orientation(Z, vector![Negative, Negative], false),
        Wing::new_with_orientation(Z, vector![Negative, Positive], false),
    ]
};
