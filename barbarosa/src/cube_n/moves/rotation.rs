//! Rotations of NxNxN cubes.
//!
//! This is mainly used because, for example, L and R' are the same rotation and
/// the only difference is the pieces selected in the rotation.
use nalgebra::vector;

use crate::{
    cube_n::{
        space::{Axis, Direction, Face},
        Vec2, Vec3,
    },
    generic::Alg,
};

use super::{Amount, AxisMove};

/// A rotation around an axis. This is similar to an [AxisMove](super::AxisMove), but it doesn't
/// specify the face.
#[derive(Debug, Clone, PartialEq, Eq)]
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
        self.amount != Amount::Double && (self.axis == Axis::X || self.axis == normal_axis)
    }

    /// Retruns the inverse rotation, such that if both get applied the result is no rotation.
    pub fn inverse(&self) -> Self {
        Self {
            axis: self.axis,
            amount: self.amount * Direction::Negative,
        }
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

impl Rotatable for Vec3 {
    fn rotate(&mut self, rotation: &AxisRotation) {
        *self = rotation.axis.map_on_slice(*self, |[x, y]| {
            rotate_vec2(&rotation.amount, &vector![*x, *y])
        });
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
pub fn rotate_vec2(amount: &Amount, vec: &Vec2) -> Vec2 {
    match amount {
        Amount::Single => vector![vec.y, -vec.x],
        Amount::Double => vector![-vec.x, -vec.y],
        Amount::Inverse => vector![-vec.y, vec.x],
    }
}

impl Rotatable for AxisMove {
    fn rotate(&mut self, axis_rotation: &AxisRotation) {
        self.face.rotate(axis_rotation);
    }
}

impl Rotatable for Alg<AxisMove> {
    fn rotate(&mut self, rotation: &AxisRotation) {
        for mov in &mut self.moves {
            mov.rotate(rotation);
        }
    }
}
