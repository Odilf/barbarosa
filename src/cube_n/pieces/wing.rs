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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Wing {
    corresponding_edge: Edge,
}

impl generic::Piece for Wing {}

impl Rotatable for Wing {
    fn rotate(&mut self, rotation: &AxisRotation) {
        println!("Rotating with rotation {:?} and normal {:?}", rotation, self.normal_axis());
        self.corresponding_edge.rotate(rotation);

        // if rotation.axis == self.normal_axis() && rotation.amount != Amount::Double {
        //     self.corresponding_edge.oriented = !self.corresponding_edge.oriented;
        // }
    }
}

impl Wing {
    pub fn normal_axis(&self) -> Axis {
        self.corresponding_edge.normal_axis
    }

    pub fn slice_position(&self) -> Vector2<Direction> {
        self.corresponding_edge.slice_position
    }

    pub fn normal_direction(&self) -> Direction {
        wing_normal_direction(
            self.normal_axis(),
            self.corresponding_edge.slice_position,
            self.corresponding_edge.oriented,
        )
    }

    pub fn faces(&self) -> [Face; 2] {
        self.corresponding_edge.faces()
    }
}

pub fn wing_normal_direction(
    normal_axis: Axis,
    slice_position: Vector2<Direction>,
    oriented: bool,
) -> Direction {
    let is_x_axis = normal_axis == Axis::X;

    let is_even_position_parity = slice_position.x == slice_position.y;

    if is_x_axis ^ is_even_position_parity ^ oriented {
        Direction::Negative
    } else {
        Direction::Positive
    }
}

impl Wing {
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

    pub fn from_faces(
        faces: [Face; 2],
        normal_direction: Direction,
    ) -> Result<Self, ParallelAxesError> {
        let (normal_axis, slice_position) = Edge::position_from_faces(faces)?;
        Ok(Wing::new(normal_axis, slice_position, normal_direction))
    }

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
