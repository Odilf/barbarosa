use core::fmt;
use std::fmt::Display;

use generic::parse;
use nalgebra::Vector3;
use rand_derive2::RandGen;

use crate::{cube_n::Edge, generic};

use super::{Axis, Direction};

/// One of the 6 faces of a cube
#[derive(Clone, PartialEq, Eq, RandGen, Hash)]
pub struct Face {
    /// The axis of the face
    pub axis: Axis,
    /// The direction along the axis of the face
    pub direction: Direction,
}

impl From<&Face> for Vector3<i8> {
    fn from(face: &Face) -> Vector3<i8> {
        Vector3::from(&face.axis) * i8::from(&face.direction)
    }
}

impl fmt::Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Face {{ {self} }}")?;
        Ok(())
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Face::R => "R",
            Face::L => "L",
            Face::U => "U",
            Face::D => "D",
            Face::F => "F",
            Face::B => "B",
        })
    }
}

/// Module that re-exports all faces in order to be able to do `use faces::*`.
pub mod faces {
    use super::Face;

    /// The right face
    pub const R: Face = Face::R;
    /// The left face
    pub const L: Face = Face::L;
    /// The "up" face
    pub const U: Face = Face::U;
    /// The "down" face
    pub const D: Face = Face::D;
    /// The front face
    pub const F: Face = Face::F;
    /// The back face
    pub const B: Face = Face::B;
}

impl Face {
    /// Creates a new face with the specified axis and direction
    pub const fn new(axis: Axis, direction: Direction) -> Face {
        Face { axis, direction }
    }

    /// The right face
    pub const R: Face = Face::new(Axis::X, Direction::Positive);
    /// The left face
    pub const L: Face = Face::new(Axis::X, Direction::Negative);
    /// The "up" face
    pub const U: Face = Face::new(Axis::Y, Direction::Positive);
    /// The "down" face
    pub const D: Face = Face::new(Axis::Y, Direction::Negative);
    /// The front face
    pub const F: Face = Face::new(Axis::Z, Direction::Positive);
    /// The back face
    pub const B: Face = Face::new(Axis::Z, Direction::Negative);

    /// Iterates over all 6 faces
    pub fn iter() -> impl Iterator<Item = Face> {
        use faces::*;
        use std::iter::once;

        once(R)
            .chain(once(U))
            .chain(once(F))
            .chain(once(L))
            .chain(once(D))
            .chain(once(B))
    }

    /// Gets the opposite face
    pub fn opposite(&self) -> Face {
        Face {
            axis: self.axis,
            direction: -self.direction,
        }
    }

    /// Gets the next face around a 90deg clockwise rotation around the given axis.
    pub fn next_around(self, axis: &Axis) -> Face {
        match axis {
            Axis::X => match self {
                Face::F => Face::U,
                Face::D => Face::F,
                Face::B => Face::D,
                Face::U => Face::B,
                _ => self,
            },

            Axis::Y => match self {
                Face::R => Face::F,
                Face::F => Face::L,
                Face::L => Face::B,
                Face::B => Face::R,
                _ => self,
            },

            Axis::Z => match self {
                Face::R => Face::D,
                Face::D => Face::L,
                Face::L => Face::U,
                Face::U => Face::R,
                _ => self,
            },
        }
    }

    /// Gets the next face around a 90deg counterclockwise rotation around the given axis.
    pub fn prev_around(self, axis: &Axis) -> Face {
        match axis {
            Axis::X => match self {
                Face::F => Face::D,
                Face::D => Face::B,
                Face::B => Face::U,
                Face::U => Face::F,
                _ => self,
            },

            Axis::Y => match self {
                Face::R => Face::B,
                Face::B => Face::L,
                Face::L => Face::F,
                Face::F => Face::R,
                _ => self,
            },

            Axis::Z => match self {
                Face::R => Face::U,
                Face::U => Face::L,
                Face::L => Face::D,
                Face::D => Face::R,
                _ => self,
            },
        }
    }

    /// Parses a face from a character.
    ///
    /// Not implemented with [Parsable](crate::generic::parse::Parsable) because it's easier
    /// to just accept a char instead of a string.
    pub fn parse(value: char) -> parse::Result<Face> {
        match value {
            'R' => Ok(Face::R),
            'L' => Ok(Face::L),
            'U' => Ok(Face::U),
            'D' => Ok(Face::D),
            'F' => Ok(Face::F),
            'B' => Ok(Face::B),
            other => Err(parse::Error::InvalidChar(other)),
        }
    }

    /// Wether a face contains a vector
    pub fn contains_vector(&self, vec: &Vector3<Direction>) -> bool {
        vec[self.axis] == self.direction
    }

    /// Whether a face contains an edge
    pub fn contains_edge(&self, edge: &Edge) -> bool {
        let offset = edge.normal_axis.offset(&self.axis);

        match offset {
            0 => false,
            1 => edge.slice_position[0] == self.direction,
            2 => edge.slice_position[1] == self.direction,
            _ => unreachable!("Offset should be in the range 0..3"),
        }
    }

    /// Computes the "cross product" of two faces. That is, returns the face that is
    /// perpendicular to both faces, according to the right-hand rule.
    ///
    /// Retruns `None` if the faces are parallel.
    pub fn cross(&self, other: &Face) -> Option<Face> {
        let axis = Axis::other(&self.axis, &other.axis)?;

        let direction = match self.clone().next_around(&axis) == *other {
            false => Direction::Positive,
            true => Direction::Negative,
        };

        Some(Face::new(axis, direction))
    }
}
