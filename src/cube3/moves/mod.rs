//! Module for handling rotations and moves.

pub mod alg;
mod test;

use std::{
    fmt::Display,
    mem::{self, MaybeUninit},
};

use itertools::iproduct;
use nalgebra::{vector, Vector2};
use strum::{EnumIter, IntoEnumIterator};
use thiserror::Error;

use super::{
    piece::{Corner, Edge},
    space::{Axis, Direction, Face, FaceParseError},
    Piece,
};

/// A move amount (either single, double or reverse)
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
#[allow(missing_docs)]
pub enum Amount {
    Single,
    Double,
    Inverse,
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amount::Single => write!(f, ""),
            Amount::Double => write!(f, "2"),
            Amount::Inverse => write!(f, "'"),
        }
    }
}

impl std::ops::Mul<Direction> for Amount {
    type Output = Amount;

    /// Multiply an amount by a direction to get the resulting amount.
    ///
    /// Used to convert between moves to rotations, encoding the fact that
    /// R and L' is the same rotation (namely, a 90 degree rotation around the X axis)
    fn mul(self, rhs: Direction) -> Self::Output {
        use Amount::*;
        use Direction::*;

        match (self, rhs) {
            (Single, Positive) | (Inverse, Negative) => Single,
            (Double, _) => Double,
            (Inverse, Positive) | (Single, Negative) => Inverse,
        }
    }
}

impl Amount {
    /// Parses an [Amount]
    pub fn parse(value: Option<char>) -> Result<Amount, AmountParseError> {
        match value {
            None => Ok(Amount::Single),
            Some('2') => Ok(Amount::Double),
            Some('\'') => Ok(Amount::Inverse),
            Some(other) => Err(AmountParseError::InvalidAmount(other)),
        }
    }
}

/// An error that can occur when parsing an [Amount]
#[derive(Debug, Error)]
pub enum AmountParseError {
    /// Found a character wasn't `2` or `'`
    #[error("Invalid amount: {0}")]
    InvalidAmount(char),
}

/// A move on the 3x3x3 cube
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Move {
    /// The face that is being rotated
    pub face: Face,
    /// The amount of rotation
    pub amount: Amount,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.face, self.amount)
    }
}

impl Move {
    /// Creates a new [Move]
    pub fn new(face: Face, amount: Amount) -> Self {
        Self { face, amount }
    }

    /// Gets the corresponding rotation for this move. E.g. R and L' both become a 90 degree rotation around the X axis.
    pub fn rotation(&self) -> Rotation {
        Rotation {
            axis: self.face.axis,
            amount: self.amount * self.face.direction,
        }
    }

    /// Parses a [Move] from a string
    pub fn parse(value: &str) -> Result<Move, MoveParseError> {
        let mut chars = value.chars();
        let face = chars.next().ok_or(MoveParseError::UnexpectedEnd)?;
        let amount = chars.next();

        if let Some(next) = chars.next() {
            return Err(MoveParseError::ExpectedEnd(next));
        }

        let face = Face::parse(face).map_err(MoveParseError::InvalidFace)?;
        let amount = Amount::parse(amount).map_err(MoveParseError::InvalidAmount)?;

        Ok(Move { face, amount })
    }

    const DISTINCT_MOVES: usize = 3 * 2 * 3;

    /// Returns an array of all moves
    pub fn all() -> [Move; Self::DISTINCT_MOVES] {
        let mut moves: [MaybeUninit<Move>; Self::DISTINCT_MOVES] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for (i, (axis, direction, amount)) in
            iproduct!(Axis::iter(), Direction::iter(), Amount::iter()).enumerate()
        {
            let mov = Move::new(Face::new(axis, direction), amount);
            moves[i].write(mov);
        }

        unsafe { mem::transmute::<_, [Move; Self::DISTINCT_MOVES]>(moves) }
    }

    /// Returns the inverse of this move
    pub fn reversed(&self) -> Move {
        Move::new(self.face.clone(), self.amount * Direction::Negative)
    }
}

/// A rotation around an axis. This is similar to a [Move], but it doesn't
/// specify the face. Mainly, this is used because L and R' are the same rotation,
/// the only difference is the pieces selected in the rotation.
pub struct Rotation {
    /// The axis that is being rotated around
    pub axis: Axis,
    /// The amount of rotation
    pub amount: Amount,
}

impl Rotation {
    /// Creates a new [Rotation]
    pub fn new(axis: Axis, amount: Amount) -> Self {
        Self { axis, amount }
    }

    /// Rotates a [Vector2]
    pub fn rotate_vec(amount: &Amount, vec: Vector2<Direction>) -> Vector2<Direction> {
        match amount {
            Amount::Single => vector![vec.y, -vec.x],
            Amount::Double => vector![-vec.x, -vec.y],
            Amount::Inverse => vector![-vec.y, vec.x],
        }
    }

    /// Rotates a [Face]
    pub fn rotate_face(&self, face: Face) -> Face {
        if self.axis == face.axis {
            return face;
        }

        match self.amount {
            Amount::Double => face.opposite(),
            Amount::Single => face.next_around(&self.axis),
            Amount::Inverse => face.prev_around(&self.axis),
        }
    }

    /// Rotates a [Corner]
    pub fn rotate_corner(&self, corner: &mut Corner) {
        corner.position = self
            .axis
            .map_on_slice(corner.position, |vec| Self::rotate_vec(&self.amount, vec));
        match (
            self.amount,
            Axis::other(&corner.orientation_axis, &self.axis),
        ) {
            (Amount::Double, _) => (),
            (_, Some(other_axis)) => corner.orientation_axis = other_axis,
            _ => (),
        }
    }

    /// Rotates an [Edge]
    pub fn rotate_edge(&self, edge: &mut Edge) {
        // Orientation changes whenever there's a not double move on the X axis
        if self.axis == Axis::X && self.amount != Amount::Double {
            edge.oriented = !edge.oriented;
        }

        // Position
        let faces = edge.faces().map(|face| self.rotate_face(face));
        let Ok((axis, position)) = Edge::position_from_faces(faces) else {
			edge.position = Self::rotate_vec(&self.amount, edge.position);
			return;
		};

        edge.position = position;
        edge.normal_axis = axis;
    }
}

fn move_piece<T: Piece>(piece: &mut T, mov: &Move) {
    if piece.in_face(&mov.face) {
        piece.rotate(&mov.rotation());
    }
}

pub fn do_move(pieces: &mut [impl Piece], mov: &Move) {
    for piece in pieces {
        move_piece(piece, mov);
    }
}

/// An error that can occur when parsing a [Face]
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum MoveParseError {
    #[error("Unexpected end of string")]
    UnexpectedEnd,
    #[error("Expected end of input, got {0}")]
    ExpectedEnd(char),
    #[error("Invalid face: {0}")]
    InvalidFace(FaceParseError),
    #[error("Invalid amount: {0}")]
    InvalidAmount(AmountParseError),
}
