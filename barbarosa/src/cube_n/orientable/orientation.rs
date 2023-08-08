use std::ops::Neg;

use itertools::Either;

use crate::cube_n::{
    moves::{
        rotation::{AxisRotation, Rotatable},
        Amount,
    },
    pieces::edge::ParallelAxesError,
    space::{Direction, Face},
};

/// An orientation in an axis.
///
/// The orientation is determined by where the R and U faces are (even though it's implemented in a slightly different way)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Orientation {
    r_face: Face,
    handedness: Direction,
    side_direction: Direction,
}

fn connecting_rotation(from: &Face, to: &Face) -> Option<AxisRotation> {
    let output = match from.cross(to) {
        Some(cross_result) => {
            AxisRotation::new(cross_result.axis, cross_result.direction.neg().into())
        }
        None => {
            if from.direction == to.direction {
                return None;
            }

            AxisRotation::new(from.axis.next(), Amount::Double)
        }
    };

    Some(output)
}

/// Gives incorrect results if the faces cannot be reached by a rotation around the axis of `face`.
fn connecting_rotation_with_face(from: &Face, to: &Face, face: &Face) -> Option<AxisRotation> {
    debug_assert_ne!(from.axis, face.axis);
    debug_assert_ne!(to.axis, face.axis);

    if from == to {
        return None;
    }

    let amount = match from.axis.get_handedness(&to.axis) {
        Ok(Direction::Positive) => Amount::Single,
        Ok(Direction::Negative) => Amount::Inverse,
        Err(ParallelAxesError::SameAxis(_)) => Amount::Double,
    };

    Some(AxisRotation::new(face.axis, amount))
}

impl Orientation {
    /// The current position of the original R face, as a face
    pub fn r_face(&self) -> &Face {
        &self.r_face
    }

    /// The current position of the original U face, as a face
    pub fn u_face(&self) -> Face {
        let axis = self.r_face.axis.next_with_handedness(self.handedness);
        Face::new(axis, self.side_direction)
    }

    /// Iterator of rotations that get you from the default orientation to the current one
    pub fn rotations(&self) -> impl Iterator<Item = AxisRotation> {
        let r_rot = connecting_rotation(&Face::R, &self.r_face);
        let new_u = r_rot
            .as_ref()
            .map(|rot| Face::U.rotated(&rot))
            .unwrap_or(Face::U);

        let r_face = r_rot.as_ref().map(|rot| Face::R.rotated(rot));

        let u_rot = match r_face {
            Some(r_face) => connecting_rotation_with_face(&new_u, &self.u_face(), &r_face),
            None => connecting_rotation(&new_u, &self.u_face()),
        };

        use std::iter::{empty, once};
        use Either::*;

        match (r_rot, u_rot) {
            (None, None) => Left(Left(empty())),
            (Some(rot), None) | (None, Some(rot)) => Left(Right(once(rot))),
            (Some(r_rot), Some(u_rot)) => Right(once(r_rot).chain(once(u_rot))),
        }
    }
}

impl TryFrom<[Face; 2]> for Orientation {
    type Error = ParallelAxesError;

    fn try_from(value: [Face; 2]) -> Result<Self, Self::Error> {
        let [r_face, u_face] = value;

        let handedness = r_face.axis.get_handedness(&u_face.axis)?;
        let side_direction = u_face.direction;

        Ok(Self {
            r_face,
            handedness,
            side_direction,
        })
    }
}

impl Rotatable for Orientation {
    fn rotate(&mut self, rotation: &AxisRotation) {
        let u_face = self.u_face().rotated(rotation);
        self.r_face.rotate(rotation); // Rotating the r face needs to happen after the u face, otherwise the u face gets computed incorrectly

        self.handedness = self
            .r_face
            .axis
            .get_handedness(&u_face.axis)
            .expect("Faces should be perpendicular after rotation");

        self.side_direction = u_face.direction;
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            r_face: Face::R,
            handedness: Direction::Positive,
            side_direction: Direction::Positive,
        }
    }
}

#[test]
fn test_all_connects() {
    for from in Face::iter() {
        for to in Face::iter() {
            let rotation = connecting_rotation(&from, &to);

            if from == to {
                assert!(rotation.is_none());
            } else {
                assert_eq!(from.clone().rotated(&rotation.unwrap()), to);
            }
        }
    }
}
