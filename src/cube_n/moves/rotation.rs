use nalgebra::{vector, Vector2, Vector3};

use crate::cube_n::space::{Axis, Direction, Face};

use super::{Amount, AxisMove};

/// A rotation around an axis. This is similar to an [AxisMove](super::AxisMove), but it doesn't
/// specify the face. Mainly, this is used because, for example, L and R' are the same rotation and
/// the only difference is the pieces selected in the rotation.
#[derive(Debug, Clone)]
pub struct AxisRotation {
    /// The axis that is being rotated around
    pub axis: Axis,
    /// The amount of rotation
    pub amount: Amount,
}

impl AxisRotation {
    /// Creates a new [AxisRotation]
    pub fn new(axis: Axis, amount: Amount) -> Self {
        Self { axis, amount }
    }

    /// Whether a rotation flips the orientation of an edge.
    /// 
    /// todo!()
    pub fn flips_edge_orientation(&self, normal_axis: Axis) -> bool {
        self.amount != Amount::Double && (
            (self.axis == Axis::X) || (self.axis == normal_axis)
        )
    }
}

/// Things that can be rotated.
pub trait Rotatable: Sized {
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

impl Rotatable for Vector3<Direction> {
    fn rotate(&mut self, rotation: &AxisRotation) {
        *self = rotation
            .axis
            .map_on_slice(*self, |vec| rotate_vec2(&rotation.amount, vec));
    }
}

impl Rotatable for Axis {
    fn rotate(&mut self, rotation: &AxisRotation) {
        if rotation.amount != Amount::Double {
            if let Some(other_axis) = Axis::other(self, &rotation.axis) {
                *self = other_axis;
            }
        }
    }
}

impl From<&AxisMove> for AxisRotation {
    fn from(mov: &AxisMove) -> Self {
        AxisRotation {
            axis: mov.face.axis,
            amount: mov.amount,
        }
    }
}

/// Rotates a [Vector2] clockwise
pub fn rotate_vec2(amount: &Amount, vec: Vector2<Direction>) -> Vector2<Direction> {
    match amount {
        Amount::Single => vector![vec.y, -vec.x],
        Amount::Double => vector![-vec.x, -vec.y],
        Amount::Inverse => vector![-vec.y, vec.x],
    }
}
