use nalgebra::{vector, Vector2};

use crate::cube_n::{
    space::{Axis, Direction, Face},
    Corner, Edge,
};

use super::Amount;

/// A rotation around an axis. This is similar to an [AxisMove](super::AxisMove), but it doesn't
/// specify the face. Mainly, this is used because L and R' are the same rotation,
/// the only difference is the pieces selected in the rotation.
#[derive(Debug)]
pub struct AxisRotation {
    /// The axis that is being rotated around
    pub axis: Axis,
    /// The amount of rotation
    pub amount: Amount,
}

/// Things that can be rotated.
///
/// Any type that implements [Rotatable] automatically implements [`crate::generic::Movable<AxisMove>`]
pub trait Rotatable: Clone {
    /// Rotates a piece according to an [AxisRotation]
    fn rotate(&mut self, rotation: &AxisRotation);

    /// Returns a rotated copy of the piece
    ///
    /// See also [Self::rotate]
    fn rotated(mut self, rotation: &AxisRotation) -> Self {
        self.rotate(rotation);
        self
    }
}

impl Rotatable for Face {
    fn rotate(&mut self, rotation: &AxisRotation) {
        if self.axis == rotation.axis {
            return;
        }

        *self = match rotation.amount {
            Amount::Double => self.opposite(),
            Amount::Single => self.clone().next_around(&rotation.axis),
            Amount::Inverse => self.clone().prev_around(&rotation.axis),
        };
    }
}

impl Rotatable for Corner {
    fn rotate(&mut self, rotation: &AxisRotation) {
        self.position = rotation
            .axis
            .map_on_slice(self.position, |vec| rotate_vec2d(&rotation.amount, vec));
        match (
            rotation.amount,
            Axis::other(&self.orientation_axis, &rotation.axis),
        ) {
            (Amount::Double, _) => (),
            (_, Some(other_axis)) => self.orientation_axis = other_axis,
            _ => (),
        }
    }
}

impl Rotatable for Edge {
    fn rotate(&mut self, rotation: &AxisRotation) {
        // Orientation changes whenever there's a not double move on the X axis
        if rotation.axis == Axis::X && rotation.amount != Amount::Double {
            self.oriented = !self.oriented;
        }

        // Position
        let faces = self.faces().map(|face| face.rotated(rotation));
        let Ok((new_normal, new_slice_position)) = Edge::position_from_faces(faces) else {
			self.slice_position = rotate_vec2d(&rotation.amount, self.slice_position);
			return;
		};

        self.normal_axis = new_normal;
        self.slice_position = new_slice_position;
    }
}

impl AxisRotation {
    /// Creates a new [AxisRotation]
    pub fn new(axis: Axis, amount: Amount) -> Self {
        Self { axis, amount }
    }
}

/// Rotates a [Vector2]
pub fn rotate_vec2d(amount: &Amount, vec: Vector2<Direction>) -> Vector2<Direction> {
    match amount {
        Amount::Single => vector![vec.y, -vec.x],
        Amount::Double => vector![-vec.x, -vec.y],
        Amount::Inverse => vector![-vec.y, vec.x],
    }
}
