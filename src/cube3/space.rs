//! Module for operations in space.

use std::{
    fmt::{Debug, Display},
    ops::{Index, Neg},
};

use nalgebra::{vector, Vector2, Vector3};
use rand::{distributions::Standard, prelude::Distribution};
use strum::EnumIter;
use thiserror::Error;

use super::piece::{Corner, Edge};

/// The three axes in space.
///
/// We use Y-up Z-in coordinates, so the axes are:
/// - X for the R-L axis
/// - Y for the U-D axis
/// - Z for the F-B axis
///
/// This means that it is right-hand oriented, such that `X x Y = Z`.
///
/// Another way to think about it is that the axes are ordered based on the frequency of their respective face moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Axis {
    /// The R-L axis
    X = 0,
    /// The U-D axis
    Y = 1,
    /// The F-B axis
    Z = 2,
}

impl<T> Index<Axis> for Vector3<T> {
    type Output = T;

    fn index(&self, index: Axis) -> &Self::Output {
        &self[index as usize]
    }
}

impl Distribution<Axis> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match rng.gen_range(0..3) {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => unreachable!(),
        }
    }
}

impl From<&Axis> for Vector3<i8> {
    fn from(axis: &Axis) -> Vector3<i8> {
        match axis {
            Axis::X => Vector3::new(1, 0, 0),
            Axis::Y => Vector3::new(0, 1, 0),
            Axis::Z => Vector3::new(0, 0, 1),
        }
    }
}

impl Axis {
    /// Maps vector on slice in the specified axis. That is, you look at the
    /// axis head on and just assign `x` and `y` accordingly.
    pub fn map_on_slice<T: Clone>(
        &self,
        mut vec: Vector3<T>,
        f: impl FnOnce(Vector2<T>) -> Vector2<T>,
    ) -> Vector3<T> {
        // Why tf doesn't dot syntax work on generic vectors??
        // This should be a lot simpler :(
        // TODO: Look into this
        let (x, y) = match self {
            Axis::X => (1, 2),
            Axis::Y => (2, 0),
            Axis::Z => (0, 1),
        };

        let mapped = f(vector![vec[x].clone(), vec[y].clone()]);
        vec[x] = mapped[0].clone();
        vec[y] = mapped[1].clone();

        vec
    }

    /// Calculates the offset between two axes. Basically:
    /// - 0 if the axes are the same
    /// - 1 if the axes are `(X, Y)`, `(Y, Z)`, or `(Z, X)`
    /// - 2 if the axes are `(X, Z)`, `(Y, X)`, or `(Z, Y)`
    pub fn offset(&self, other: &Axis) -> i32 {
        (*other as i32 - *self as i32).rem_euclid(3)
    }

    /// Returns the next axis in the loop X -> Y -> Z -> X
    pub fn next(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }

    /// Returns the previous axis in the loop X -> Z -> Y -> X
    pub fn prev(&self) -> Axis {
        match self {
            Axis::X => Axis::Z,
            Axis::Y => Axis::X,
            Axis::Z => Axis::Y,
        }
    }

    /// Returns the axis that is not `a` or `b`, or `None` if they are the same.
    pub fn other(a: &Axis, b: &Axis) -> Option<Axis> {
        use Axis::*;

        match (a, b) {
            (X, X) | (Y, Y) | (Z, Z) => None,
            (X, Y) | (Y, X) => Some(Z),
            (X, Z) | (Z, X) => Some(Y),
            (Y, Z) | (Z, Y) => Some(X),
        }
    }
}

/// Spacial direction, used to indicate the two different directions along an axis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
#[allow(missing_docs)]
pub enum Direction {
    Positive = 1,
    Negative = -1,
}

impl From<&Direction> for i8 {
    fn from(direction: &Direction) -> i8 {
        match direction {
            Direction::Positive => 1,
            Direction::Negative => -1,
        }
    }
}

impl Direction {
    /// Helper function that just calls `i8::from(self)`
    pub fn scalar(&self) -> i8 {
        i8::from(self)
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::Positive => Direction::Negative,
            Direction::Negative => Direction::Positive,
        }
    }
}

/// One of the 6 faces of a cube
#[derive(Clone, PartialEq, Eq)]
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

impl Debug for Face {
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

    /// Whether the face contains the given corner
    pub fn contains_corner(&self, corner: &Corner) -> bool {
        corner.position[self.axis] == self.direction
    }

    /// Whether the face contains the given edge
    pub fn contains_edge(&self, edge: &Edge) -> bool {
        let offset = edge.normal_axis.offset(&self.axis);

        match offset {
            0 => false,
            1 => edge.position[0] == self.direction,
            2 => edge.position[1] == self.direction,
            _ => unreachable!("Offset should be in the range 0..3"),
        }
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
                Face::R => Face::U,
                Face::U => Face::L,
                Face::L => Face::D,
                Face::D => Face::R,
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
                Face::R => Face::D,
                Face::D => Face::L,
                Face::L => Face::U,
                Face::U => Face::R,
                _ => self,
            },
        }
    }

    /// Parses a face from a character
    pub fn parse(value: char) -> Result<Face, FaceParseError> {
        match value {
            'R' => Ok(Face::R),
            'L' => Ok(Face::L),
            'U' => Ok(Face::U),
            'D' => Ok(Face::D),
            'F' => Ok(Face::F),
            'B' => Ok(Face::B),
            _ => Err(FaceParseError::InvalidFaceName(value)),
        }
    }
}

#[test]
fn text_next_around() {
    assert_eq!(Face::F.next_around(&Axis::X), Face::U);
    assert_eq!(Face::U.next_around(&Axis::X), Face::B);
    assert_eq!(Face::B.next_around(&Axis::X), Face::D);
    assert_eq!(Face::D.next_around(&Axis::X), Face::F);
}

/// An error that can occur when parsing a face
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum FaceParseError {
    #[error("Invalid face name: {0}")]
    InvalidFaceName(char),
}
