use std::ops::Index;

use nalgebra::{Vector2, Vector3};
use rand_derive2::RandGen;
use strum::EnumIter;

use crate::cube_n::pieces::edge::ParallelAxesError;

use super::Direction;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, RandGen)]
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
    /// Returns the basis that you use with a given normal (and positive handedness)
    pub fn basis(normal: &Axis) -> [Axis; 2] {
        match *normal {
            Axis::X => [Axis::Y, Axis::Z],
            Axis::Y => [Axis::Z, Axis::X],
            Axis::Z => [Axis::X, Axis::Y],
        }
    }

    /// Maps vector on slice in the specified axis. That is, you look at the
    /// axis head on and just assign `x` and `y` accordingly.
    pub fn map_on_slice<T: Clone>(
        &self,
        mut vec: Vector3<T>,
        f: impl FnOnce([&T; 2]) -> Vector2<T>,
    ) -> Vector3<T> {
        let [x, y] = Axis::basis(self);

        let result = f([&vec[x], &vec[y]]);
        vec[x as usize] = result[0].clone();
        vec[y as usize] = result[1].clone();

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

    /// Returns the next vector given a handedness
    pub fn next_with_handedness(&self, handedness: Direction) -> Axis {
        match handedness {
            Direction::Positive => self.next(),
            Direction::Negative => self.prev(),
        }
    }

    /// Returns the handedness of `self` followed by `other`. If they're in order, it's positive; if they're
    /// not, it's negative; and if they're parallel it errors.
    pub fn get_handedness(&self, other: &Axis) -> Result<Direction, ParallelAxesError> {
        match self.offset(other) {
            0 => Err(ParallelAxesError::SameAxes([*self, *other])),
            1 => Ok(Direction::Positive),
            2 => Ok(Direction::Negative),
            _ => unreachable!(),
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
