use nalgebra::{vector, Vector2, Vector3};
use thiserror::Error;

use crate::{
    cube_n::space::{Axis, Direction, Face},
    generic,
};

/// An edge piece of the cube.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Edge {
    /// The axis of the normal of the face the edge is on (i.e. the axis where the coordinate is 0)
    pub normal_axis: Axis,

    /// The position of the edge on the slice
    pub slice_position: Vector2<Direction>,

    /// Whether the edge is oriented or not.
    ///
    /// See [Edge::oriented()] for more information.
    pub oriented: bool,
}

impl generic::Piece for Edge {
    fn coordinates(&self) -> nalgebra::Vector3<f32> {
        self.normal_axis.map_on_slice(Vector3::zeros(), |_| {
            self.slice_position.map(|dir| dir.scalar() as f32)
        })
    }
}

impl Edge {
    /// Creates a new oriented edge
    pub const fn oriented(normal_axis: Axis, slice_position: Vector2<Direction>) -> Self {
        Self {
            normal_axis,
            slice_position,
            oriented: true,
        }
    }

    /// Gets the faces of the edge
    pub fn faces(&self) -> [Face; 2] {
        let x = self.normal_axis.next();
        let y = x.next();

        [
            Face::new(x, self.slice_position[0]),
            Face::new(y, self.slice_position[1]),
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
        let normal_axis =
            Axis::other(&a.axis, &b.axis).ok_or(EdgeFromFacesError::SameAxes([a.axis, b.axis]))?;

        let x = normal_axis.next();
        let y = x.next();

        let position = if x == a.axis && y == b.axis {
            vector![a.direction, b.direction]
        } else if x == b.axis && y == a.axis {
            vector![b.direction, a.direction]
        } else {
            unreachable!()
        };

        Ok((normal_axis, position))
    }
}

impl std::fmt::Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [f1, f2] = self.faces();
        write!(f, "Edge {{ faces: {f1}{f2}, oriented: {} }}", self.oriented)?;

        Ok(())
    }
}

impl TryFrom<[Face; 2]> for Edge {
    type Error = EdgeFromFacesError;

    fn try_from(value: [Face; 2]) -> Result<Self, Self::Error> {
        let (normal_axis, slice_position) = Self::position_from_faces(value)?;

        Ok(Self {
            normal_axis,
            slice_position,
            oriented: true,
        })
    }
}

/// An error that can occur when creating an edge from faces
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum EdgeFromFacesError {
    #[error("Faces must be on different axes")]
    SameAxes([Axis; 2]),
}

/// A list of all the edges in a solved cube.
///
/// Edges are set up this way so that an X2 rotation increases the index by 6.
/// That is, `SOLVED[n]` and `SOLVED[n + 6]` differ by an X2 rotation. This is
/// useful for indexing into the HalfEdges permutation table.
///
/// See [crate::cube_n::cube3::mus] for more information.
pub const SOLVED: [Edge; 12] = {
    use Axis::*;
    use Direction::*;

    [
        Edge::oriented(X, vector![Positive, Positive]),
        Edge::oriented(X, vector![Positive, Negative]),
        Edge::oriented(Y, vector![Positive, Positive]),
        Edge::oriented(Y, vector![Positive, Negative]),
        Edge::oriented(Z, vector![Positive, Positive]),
        Edge::oriented(Z, vector![Negative, Positive]),
        Edge::oriented(X, vector![Negative, Negative]),
        Edge::oriented(X, vector![Negative, Positive]),
        Edge::oriented(Y, vector![Negative, Positive]),
        Edge::oriented(Y, vector![Negative, Negative]),
        Edge::oriented(Z, vector![Positive, Negative]),
        Edge::oriented(Z, vector![Negative, Negative]),
    ]
};
