use std::fmt::Debug;

use nalgebra::{vector, Vector2, Vector3};
use thiserror::Error;

use crate::cube3::{moves::Rotation, space::Face, Axis, Direction};

use super::Piece;

/// An edge piece of the cube.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Edge {
    /// The axis of the normal of the face the edge is on (i.e. the axis where the coordinate is 0)
    pub normal_axis: Axis,

    /// The position of the edge on the slice
    pub position: Vector2<Direction>,

    /// Whether the edge is oriented or not.
    ///
    /// See [Edge::oriented()] for more information.
    pub oriented: bool,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct EdgePosition {
    /// The axis of the normal of the face the edge is on (i.e. the axis where the coordinate is 0)
    pub normal_axis: Axis,

    /// The position of the edge on the slice
    pub position: Vector2<Direction>,
}

impl Piece for Edge {
    fn coordinates(&self) -> Vector3<i8> {
        self.normal_axis
            .map_on_slice(Vector3::zeros(), |_| self.position.map(|dir| dir.scalar()))
    }

    fn rotate(&mut self, rotation: &Rotation) {
        rotation.rotate_edge(self)
    }

    fn in_face(&self, face: &Face) -> bool {
        self.faces().contains(face)
    }
}

impl TryFrom<[Face; 2]> for Edge {
    type Error = EdgeFromFacesError;

    fn try_from([a, b]: [Face; 2]) -> Result<Self, Self::Error> {
        let (slice_axis, position) = Self::position_from_faces([a, b])?;

        Ok(Edge::oriented(slice_axis, position))
    }
}

#[derive(Debug, Error)]
pub enum EdgeFromFacesError {
    #[error("Faces must be on different axes")]
    SameAxes([Axis; 2]),
}

impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [f1, f2] = self.faces();
        write!(f, "Edge {{ faces: {f1}{f2}, oriented: {} }}", self.oriented)?;

        Ok(())
    }
}

impl Edge {
    /// Creates a new oriented edge
    pub const fn oriented(slice_axis: Axis, position: Vector2<Direction>) -> Self {
        Self {
            normal_axis: slice_axis,
            position,
            oriented: true,
        }
    }

    /// Gets the faces of the edge
    pub fn faces(&self) -> [Face; 2] {
        let x = self.normal_axis.next();
        let y = x.next();

        [
            Face::new(x, self.position[0]),
            Face::new(y, self.position[1]),
        ]
    }

    /// Returns the edge with the opposite orientation.
    ///
    /// See also [Edge::flip()] for mutating instead of owning.
    pub fn flipped(mut self) -> Self {
        self.oriented = !self.oriented;
        self
    }

    /// Flips the orientation of the edge.
    ///
    /// See also [Edge::flipped()] for owning instead of mutating.
    pub fn flip(&mut self) {
        self.oriented = !self.oriented;
    }

    /// Calculates the position information of an edge placed in between the given faces.
    ///
    /// Errors if the faces are not perpendicular
    pub fn position_from_faces(
        [a, b]: [Face; 2],
    ) -> Result<(Axis, Vector2<Direction>), EdgeFromFacesError> {
        let slice_axis =
            Axis::other(&a.axis, &b.axis).ok_or(EdgeFromFacesError::SameAxes([a.axis, b.axis]))?;

        let x = slice_axis.next();
        let y = x.next();

        let position = if x == a.axis && y == b.axis {
            vector![a.direction, b.direction]
        } else if x == b.axis && y == a.axis {
            vector![b.direction, a.direction]
        } else {
            unreachable!()
        };

        Ok((slice_axis, position))
    }
}
