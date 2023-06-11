mod test;
pub mod alg;

use std::{fmt::Display, mem::{MaybeUninit, self}};

use itertools::iproduct;
use nalgebra::{Vector2, vector};
use strum::{IntoEnumIterator, EnumIter};
use thiserror::Error;

use super::{space::{Face, Axis, Direction, FaceParseError}, pieces::{Corner, Edge}};

 #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum Amount {
	Single,
	Double,
	Reverse,
}

impl Display for Amount {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Amount::Single => write!(f, ""),
			Amount::Double => write!(f, "2"),
			Amount::Reverse => write!(f, "'"),
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
			(Single, Positive) | (Reverse, Negative) => Single,
			(Double, _) => Double,
			(Reverse, Positive) | (Single, Negative) => Reverse,
		}
	}
}

impl Amount {
    pub fn parse(value: Option<char>) -> Result<Amount, AmountParseError> {
		match value {
			None => Ok(Amount::Single),
			Some('2') => Ok(Amount::Double),
			Some('\'') => Ok(Amount::Reverse),
			Some(other) => Err(AmountParseError::InvalidAmount(other)),
		}
    }
}

#[derive(Debug, Error)]
pub enum AmountParseError {
	#[error("Invalid amount: {0}")]
	InvalidAmount(char),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
	pub face: Face,
	pub amount: Amount,
}

impl Display for Move {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.face, self.amount)
	}
}

impl Move {
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

	pub fn parse(value: &str) -> Result<Move, MoveParseError> {
		let face = value.chars().next().ok_or(MoveParseError::UnexpectedEnd)?;
		let amount = value.chars().nth(1);

		let face = Face::parse(face).map_err(MoveParseError::InvalidFace)?;
		let amount = Amount::parse(amount).map_err(MoveParseError::InvalidAmount)?;

		Ok(Move { face, amount })
    }

	pub const ALL_MOVES_SIZE: usize = 3 * 2 * 3;

	pub fn all() -> [Move; Self::ALL_MOVES_SIZE] {
		let mut moves: [MaybeUninit<Move>; Self::ALL_MOVES_SIZE] = unsafe { 
			MaybeUninit::uninit().assume_init()
		};

		for (i, (axis, direction, amount)) in iproduct!(Axis::iter(), Direction::iter(), Amount::iter()).enumerate() {
			let mov = Move::new(Face::new(axis, direction), amount);
			moves[i].write(mov);
		}

		unsafe { mem::transmute::<_, [Move; Self::ALL_MOVES_SIZE]>(moves) }
	}

	pub fn reversed(&self) -> Move {
		Move::new(self.face.clone(), self.amount * Direction::Negative)
	}
}

pub struct Rotation {
	pub axis: Axis,
	pub amount: Amount,
}

impl Rotation {
	pub fn new(axis: Axis, amount: Amount) -> Self {
		Self { axis, amount }
	}

	pub fn rotate_vec(amount: &Amount, vec: Vector2<Direction>) -> Vector2<Direction> {
		match amount {
			Amount::Single => vector![vec.y, -vec.x],
			Amount::Double => vector![-vec.x, -vec.y],
			Amount::Reverse => vector![-vec.y, vec.x],
		}
	}

	pub fn rotate_face(&self, face: Face) -> Face {
		if self.axis == face.axis {
			return face;
		}

		match self.amount {
			Amount::Double => face.opposite(),
			Amount::Single => face.next_around(self.axis),
			Amount::Reverse => face.prev_around(self.axis),
		}
	}

	pub fn rotate_corner(&self, corner: &mut Corner) {
		self.axis.map_on_slice(&mut corner.position, |vec| Self::rotate_vec(&self.amount, vec));
		match (self.amount, Axis::other(corner.orientation_axis, self.axis)) {
			(Amount::Double, _) => (),
			(_, Some(other_axis)) => corner.orientation_axis = other_axis,
			_ => (),
		}
	}

	pub fn rotate_edge(&self, edge: &mut Edge) {
		// Orientation changes whenever there's a not double move on the X axis
		if self.axis == Axis::X && self.amount != Amount::Double {
			edge.oriented = !edge.oriented;
		}
		
		// Position
		if self.axis == edge.normal_axis {
			edge.position = Self::rotate_vec(&self.amount, edge.position);
		}

		let faces = edge.faces().map(|face| self.rotate_face(face));

		// dbg!(&faces);
		// let slice_axis = Axis::other(faces[0].axis, faces[1].axis).expect("Faces should be perpendicular");

		// TODO: Make this not bad lol
		let correct_question_mark = Edge::try_from(faces).expect("Faces should be valid");
		edge.position = correct_question_mark.position;
		edge.normal_axis = correct_question_mark.normal_axis;

		// edge.position = Vector2::new(
		// 	faces[0].direction,
		// 	faces[1].direction,
		// );

		// edge.slice_axis = slice_axis;
	}
}

#[derive(Debug, Error)]
pub enum MoveParseError {
	#[error("Unexpected end of string")]
	UnexpectedEnd,
	#[error("Invalid face: {0}")]
	InvalidFace(FaceParseError),
	#[error("Invalid amount: {0}")]
	InvalidAmount(AmountParseError),
}
