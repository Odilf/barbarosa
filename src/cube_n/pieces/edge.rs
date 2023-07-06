//! Edge piece of the cube.

use nalgebra::{vector, Vector2, Vector3};
use thiserror::Error;

use crate::{
    cube_n::{
        moves::{
            rotation::{rotate_vec2, AxisRotation, Rotatable},
            Amount,
        },
        space::{Axis, Direction, Face},
        AxisMove,
    },
    generic,
};

// use super::{ContainedInMove, Corner};

/// An edge piece of the cube.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Edge {
    /// The axis of the normal of the face the edge is on (i.e. the axis where the coordinate is 0)
    pub normal_axis: Axis,

    /// The position of the edge on the slice. It uses as basis the positive diriection of the other two axes from the normal,
    /// in contiguous order (e.g.: if normal is Y, then basis is Z, X).
    pub slice_position: Vector2<Direction>,

    /// Whether the edge is oriented or not.
    ///
    /// An edge need to be oriented to be solved.
    ///
    /// The orientation of the edges only changes when doing a non double move on the X axis. In other words: R, R', L, L'.
    ///
    /// The geometric interpretation is a bit more involved:
    ///
    /// Each edge has two stickers. One of this stickers is the "orientation sticker". For edges on the B and F faces it's the sticker on
    /// the B and F face, for the other ones it's the sticker on the R or L face (whichever is applicable). Then, an edge is oriented if
    /// it has the orientation sticker in the place where the original piece would have it's orientation sticker.
    ///
    /// Even though this definition seems convoluted, it is very useful for two reasons:
    /// 1. The sum of the orientations of the edges in a cube has to always be even.
    /// 2. It is very easy and cheap to implement.
    pub oriented: bool,
}

impl generic::Piece for Edge {}

impl Rotatable for Edge {
    fn rotate(&mut self, rotation: &AxisRotation) {
        if rotation.axis == self.normal_axis {
            self.slice_position = rotate_vec2(&rotation.amount, self.slice_position);
            return;
        }

        // Orientation changes whenever there's a not double move on the X axis
        if rotation.axis == Axis::X && rotation.amount != Amount::Double {
            self.oriented = !self.oriented;
        }

        // Position
        let faces = self.faces().map(|face| face.rotated(rotation));
        match Edge::position_from_faces(faces) {
            Ok((new_normal, new_slice_position)) => {
                self.normal_axis = new_normal;
                self.slice_position = new_slice_position;
            }

            Err(_) => unreachable!(),
        }
    }
}

impl generic::Movable<AxisMove> for Edge {
    fn apply(&mut self, m: &AxisMove) {
        if m.face.contains_edge(self) {
            let rotation = AxisRotation::from(m);
            self.rotate(&rotation);
        }
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
    pub const fn flipped(mut self) -> Self {
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
    ) -> Result<(Axis, Vector2<Direction>), ParallelAxesError> {
        let normal_axis =
            Axis::other(&a.axis, &b.axis).ok_or(ParallelAxesError::SameAxes([a.axis, b.axis]))?;

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

    pub fn coordinates(&self) -> nalgebra::Vector3<f32> {
        self.normal_axis.map_on_slice(Vector3::zeros(), |_| {
            self.slice_position.map(|dir| dir.scalar() as f32)
        })
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
    type Error = ParallelAxesError;

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
pub enum ParallelAxesError {
    #[error("Faces must be on different axes")]
    SameAxes([Axis; 2]),
}

/// A list of all the edges in a solved cube.
///
/// Edges are set up this way so that an X2 rotation increases the index by 6.
/// That is, `SOLVED[n]` and `SOLVED[n + 6]` differ by an X2 rotation. This is
/// useful for indexing into the HalfEdges permutation table with the second half
/// of the edges.
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
