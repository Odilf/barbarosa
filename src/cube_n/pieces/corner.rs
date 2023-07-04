//!

use nalgebra::{vector, Vector3};

use crate::{
    cube_n::{
        moves::rotation::{AxisRotation, Rotatable},
        space::{Axis, Direction, Face},
        AxisMove,
    },
    generic,
};

// use super::ContainedInMove;

/// A corner piece of the cube.
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Corner {
    /// The position of the corner piece, relative to the center of the cube.
    pub position: Vector3<Direction>,

    /// The orientation of the corner piece, determined by the axis of the
    /// sticker that is originally on the X (R-L) axis (usually red-orange)
    pub orientation_axis: Axis,
}

impl generic::Piece for Corner {}

impl Rotatable for Corner {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.position.rotate(rotation);
        self.orientation_axis.rotate(rotation);
    }
}

impl generic::Movable<AxisMove> for Corner {
    fn apply(&mut self, m: &AxisMove) {
        if m.face.contains_vector(&self.position) {
            let rotation = AxisRotation::from(m);
            self.rotate(&rotation);
        }
    }
}

// impl ContainedInMove<AxisMove> for Vector3<Direction> {
//     fn is_contained_in(&self, mov: &AxisMove) -> bool {
//         self[mov.face.axis] == mov.face.direction
//     }
// }

// impl ContainedInMove<AxisMove> for Corner {
//     fn is_contained_in(&self, mov: &AxisMove) -> bool {
//         self.position.is_contained_in(mov.face)
//     }
// }

impl Corner {
    pub const fn new(position: Vector3<Direction>, orientation_axis: Axis) -> Self {
        Self {
            position,
            orientation_axis,
        }
    }

    /// Creates a new [Corner] with the given position and correct orientation.
    pub const fn oriented(position: Vector3<Direction>) -> Self {
        Self {
            position,
            orientation_axis: Axis::X,
        }
    }

    /// Returns the faces that the corner is on
    pub fn faces(&self) -> [Face; 3] {
        [
            Face::new(Axis::X, self.position.x),
            Face::new(Axis::Y, self.position.y),
            Face::new(Axis::Z, self.position.z),
        ]
    }

    fn is_even_position_parity(position: &Vector3<Direction>) -> bool {
        position
            .iter()
            .filter(|dir| dir == &&Direction::Negative)
            .count()
            % 2
            == 0
    }

    /// Amount of counter-clockwise rotations needed for an oriented corner to get
    /// to the current orientation. i think
    pub fn orientation_index(&self) -> usize {
        let even_parity = Self::is_even_position_parity(&self.position);
        let axis_index = self.orientation_axis as usize;

        if even_parity {
            axis_index
        } else {
            (3 - axis_index).rem_euclid(3)
        }
    }

    /// Gets the axis that is a counterclockwise rotation from the current axis
    /// around the given position. So the thing used for twisting corners
    pub fn next_axis(position: &Vector3<Direction>, axis: &Axis) -> Axis {
        if Self::is_even_position_parity(position) {
            axis.next()
        } else {
            axis.prev()
        }
    }

    /// Twists a corner counter-clockwise
    pub fn twist(&mut self) {
        self.orientation_axis = if Self::is_even_position_parity(&self.position) {
            self.orientation_axis.next()
        } else {
            self.orientation_axis.prev()
        };
    }

    pub fn coordinates(&self) -> Vector3<f32> {
        self.position.map(|dir| dir.scalar() as f32)
    }
}

impl std::fmt::Debug for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [f1, f2, f3] = self.faces();

        write!(
            f,
            "Corner {{ faces: {f1}{f2}{f3}, orientation_axis: {:?})",
            self.orientation_axis
        )
    }
}

/// Reference for a set of solved corners
pub const SOLVED: [Corner; 8] = {
    use Direction::*;

    [
        Corner::oriented(vector![Positive, Positive, Positive]),
        Corner::oriented(vector![Positive, Positive, Negative]),
        Corner::oriented(vector![Positive, Negative, Positive]),
        Corner::oriented(vector![Positive, Negative, Negative]),
        Corner::oriented(vector![Negative, Positive, Positive]),
        Corner::oriented(vector![Negative, Positive, Negative]),
        Corner::oriented(vector![Negative, Negative, Positive]),
        Corner::oriented(vector![Negative, Negative, Negative]),
    ]
};
