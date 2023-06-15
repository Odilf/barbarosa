use std::fmt::Debug;

use nalgebra::Vector3;

use crate::cube3::{moves::Rotation, space::Face, Axis, Direction};

use super::Piece;

/// A corner piece of the cube.
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Corner {
    /// The position of the corner piece, relative to the center of the cube.
    pub position: Vector3<Direction>,

    /// The orientation of the corner piece, determined by the axis of the
    /// sticker that is originally on the X (R-L) axis (usually red-orange)
    pub orientation_axis: Axis,
}

impl Piece for Corner {
    fn coordinates(&self) -> Vector3<i8> {
        self.position.map(|direction| direction.scalar())
    }

    fn rotate(&mut self, rotation: &Rotation) {
        rotation.rotate_corner(self)
    }

    fn in_face(&self, face: &Face) -> bool {
        face.contains_corner(self)
    }
}

impl Corner {
    /// Construct a new oriented corner at the specified position
    pub const fn oriented(position: Vector3<Direction>) -> Self {
        Self {
            position,
            orientation_axis: Axis::X,
        }
    }

    fn is_even_position_parity(position: &Vector3<Direction>) -> bool {
        position
            .iter()
            .filter(|dir| dir == &&Direction::Negative)
            .count()
            % 2
            == 0
    }

    /// Amount of counter-clockwise rotations needed to orient the corner
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

    /// Returns the faces that the corner is on
    pub fn faces(&self) -> [Face; 3] {
        [
            Face::new(Axis::X, self.position.x),
            Face::new(Axis::Y, self.position.y),
            Face::new(Axis::Z, self.position.z),
        ]
    }
}

impl Debug for Corner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [f1, f2, f3] = self.faces();

        write!(
            f,
            "Corner {{ faces: {f1}{f2}{f3}, orientation_axis: {:?})",
            self.orientation_axis
        )
    }
}
