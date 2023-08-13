//! Edge piece of the cube.

use arr_macro::arr;
use nalgebra::{vector, Vector3};
use static_assertions::const_assert_eq;
use thiserror::Error;

use crate::{
    cube_n::{
        moves::rotation::{rotate_vec2, AxisRotation, Rotatable},
        space::{Axis, Direction, Face},
        AxisMove, Vec2,
    },
    generic::{
        self, moves::impl_movable_array, piece::PieceSetDescriptor, utils::map_array_const, Piece,
        PieceSet,
    },
};

/// An edge piece of the cube.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Edge {
    /// The axis of the normal of the face the edge is on (i.e. the axis where the coordinate is 0)
    pub normal_axis: Axis,

    /// The position of the edge on the slice. It uses as basis the positive diriection of the other two axes from the normal,
    /// in contiguous order (e.g.: if normal is Y, then basis is Z, X).
    pub slice_position: Vec2,

    /// Whether the edge is oriented or not.
    ///
    /// An edge need to be oriented to be solved.
    ///
    /// The orientation of the edges only changes when doing a non double move on the Z axis. In other words: F, F', B, B'.
    ///
    /// The geometric interpretation is a bit more involved:
    ///
    /// Each edge has two stickers. One of this stickers is the "orientation sticker". For edges on the U and D faces it's the sticker on
    /// the U and D face, for the other ones it's the sticker on the F or B face (whichever is applicable). Then, an edge is oriented if
    /// it has the orientation sticker in the place where the original piece would have it's orientation sticker.
    ///
    /// Even though this definition seems convoluted, it is very useful for two reasons:
    /// 1. The sum of the orientations of the edges in a cube has to always be even.
    /// 2. It is very easy and cheap to implement.
    /// 3. It is standard in the cubing community.
    pub oriented: bool,
}

impl generic::Piece for Edge {
    type Position = (Axis, Vec2);

    fn position(&self) -> Self::Position {
        (self.normal_axis, self.slice_position)
    }

    fn is_solved(&self, original_pos: &Self::Position) -> bool {
        self.position() == *original_pos && self.oriented
    }
}

impl PieceSetDescriptor<12> for Edge {
    // TODO: Try to use cartesian product to make this nicer
    const REFERENCE_POSITIONS: [(Axis, Vec2); 12] = {
        use Axis::*;
        use Direction::*;

        [
            (X, vector![Positive, Positive]),
            (X, vector![Positive, Negative]),
            (Y, vector![Positive, Positive]),
            (Y, vector![Positive, Negative]),
            (Z, vector![Positive, Positive]),
            (Z, vector![Negative, Positive]),
            (X, vector![Negative, Negative]),
            (X, vector![Negative, Positive]),
            (Y, vector![Negative, Positive]),
            (Y, vector![Negative, Negative]),
            (Z, vector![Positive, Negative]),
            (Z, vector![Negative, Negative]),
        ]
    };

    /// A list of all the edges in a solved cube.
    ///
    /// Edges are set up this way so that an X2 rotation increases the index by 6.
    /// That is, `SOLVED[n]` and `SOLVED[n + 6]` differ by an X2 rotation. This is
    /// useful for indexing into the `HalfEdges` permutation table with the second half
    /// of the edges.
    ///
    /// See [`crate::cube_n::cube3::mus`] for more information.
    const SOLVED: [Edge; 12] = {
        const fn from_tuple((axis, pos): <Edge as Piece>::Position) -> Edge {
            Edge::oriented(axis, pos)
        }

        map_array_const!(Edge::REFERENCE_POSITIONS, 12, from_tuple)
    };
}

impl Rotatable for Edge {
    fn rotate(&mut self, rotation: &AxisRotation) {
        // Orientation changes whenever there's a not double move on the X axis
        self.oriented ^= rotation.flips_edge_orientation(self.normal_axis);

        // Position
        if rotation.axis == self.normal_axis {
            self.slice_position = rotate_vec2(&rotation.amount, &self.slice_position);
            return;
        }

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

impl_movable_array!(Edge, AxisMove);

impl generic::piece::Coordinates for Edge {
    fn coordinates_pos((normal_axis, slice_position): Self::Position) -> Vector3<f32> {
        normal_axis.map_on_slice(Vector3::zeros(), |_| {
            slice_position.map(|dir| dir.scalar() as f32)
        })
    }
}

impl Edge {
    /// Creates a new [`Edge`]
    pub const fn new(normal_axis: Axis, slice_position: Vec2, oriented: bool) -> Self {
        Self {
            normal_axis,
            slice_position,
            oriented,
        }
    }

    /// Creates a new oriented edge
    pub const fn oriented(normal_axis: Axis, slice_position: Vec2) -> Self {
        Self::new(normal_axis, slice_position, true)
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
    /// See also [`Edge::flip`()] for mutating instead of owning.
    pub const fn flipped(mut self) -> Self {
        self.oriented = !self.oriented;
        self
    }

    /// Flips the orientation of the edge.
    ///
    /// See also [`Edge::flipped`()] for owning instead of mutating.
    pub fn flip(&mut self) {
        self.oriented = !self.oriented;
    }

    /// Calculates the position information of an edge placed in between the given faces.
    ///
    /// Errors if the faces are not perpendicular
    pub fn position_from_faces([a, b]: [Face; 2]) -> Result<(Axis, Vec2), ParallelAxesError> {
        let normal_axis =
            Axis::other(&a.axis, &b.axis).ok_or(ParallelAxesError::SameAxis(a.axis))?;

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

    /// Returns the face that has the orientation sticker. That is, the U/F face for edges on the U/D faces, and the F/B face for the other ones.
    pub fn orientation_face((normal, slice_pos): <Edge as Piece>::Position) -> Face {
        match normal {
            Axis::X => Face::new(Axis::Y, slice_pos[0]),
            Axis::Y => Face::new(Axis::Z, slice_pos[0]),
            Axis::Z => Face::new(Axis::Y, slice_pos[1]),
        }
    }

    /// Returns the face that is not the [`orientation_face`](Self::orientation_face)
    pub fn non_orientation_face((normal, slice_pos): <Edge as Piece>::Position) -> Face {
        match normal {
            Axis::X => Face::new(Axis::Z, slice_pos[1]),
            Axis::Y => Face::new(Axis::X, slice_pos[1]),
            Axis::Z => Face::new(Axis::X, slice_pos[0]),
        }
    }

    /// Gets the direction of the edge position on the given axis.
    ///
    /// Returns `None` if the axis is the normal axis of the edge.
    pub fn direction_on_axis(
        &(normal, slice_pos): &<Edge as Piece>::Position,
        axis: Axis,
    ) -> Option<Direction> {
        if axis == normal {
            return None;
        }

        let x = normal.next();
        let y = x.next();

        if axis == x {
            Some(slice_pos[0])
        } else if axis == y {
            Some(slice_pos[1])
        } else {
            unreachable!()
        }
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
    SameAxis(Axis),
}

/// The piece set of 12 [`Edge`]s
pub type EdgeSet = PieceSet<Edge, 12>;
