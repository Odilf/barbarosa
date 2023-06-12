use std::fmt::Debug;

use enum_dispatch::enum_dispatch;
use nalgebra::{Vector3, Vector2, vector};
use thiserror::Error;

use super::{space::{Direction, Axis, Face}, moves::Rotation};

/// An enum that represents either a corner or a face.
/// 
/// This is useful for when you want to iterate over all pieces without having to use
/// dinamic dispatch or caring about the trait being object safe and what not. 
#[enum_dispatch]
pub enum PieceEnum {
	/// The corner piece variant. Visit [Corner] for more information.
	Corner,
	/// The edge piece variant. Visit [Edge] for more information.
	Edge,
}

/// A trait that represents a piece of the cube.
/// 
/// [Corner] and [Edge] implement this trait.
#[enum_dispatch(PieceEnum)]
pub trait Piece {
	/// The position of the piece, relative to the center of the cube.
	fn position(&self) -> Vector3<i8>;

	/// A piece only rotates. Moves are part of the cube, since
	/// they're just rotations but only to a subset of the pieces.
	fn rotate(&mut self, rotation: &Rotation);

	/// Returns `true` if the piece is on the given face.
	fn in_face(&self, face: &Face) -> bool;
}

/// A corner piece of the cube.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Corner {
	/// The position of the corner piece, relative to the center of the cube.
	pub position: Vector3<Direction>,

	/// The orientation of the corner piece, determined by the axis of the
	/// sticker that is originally on the X (R-L) axis (usually red-orange)
	pub orientation_axis: Axis,
}

impl Piece for Corner {
    fn position(&self) -> Vector3<i8> {
		self.position.map(|direction| direction.scalar())
    }

	fn rotate(&mut self, rotation: &Rotation) {
		rotation.rotate_corner(self)
	}

	fn in_face(&self, face: &Face) -> bool {
		face.contains_corner(self)
	}
}

impl Corner {
	/// Construct a new oriented corner at the specified position
	pub const fn oriented(position: Vector3<Direction>) -> Self {
		Self {
			position,
			orientation_axis: Axis::X,
		}
	}

	fn is_even_position_parity(&self) -> bool {
		self.position.iter().filter(|dir| dir == &&Direction::Negative).count() % 2 == 0
	}

	/// Amount of counter-clockwise rotations needed to orient the corner
	pub fn orientation_index(&self) -> i32 {
		let even_parity = self.is_even_position_parity();
		let axis_index = self.orientation_axis as i32;

		if even_parity {
			axis_index
		} else {
			(3 - axis_index).rem_euclid(3)
		}
	}

	/// Twists a corner counter-clockwise
	pub fn twist(&mut self) {
		self.orientation_axis = if self.is_even_position_parity() {
			self.orientation_axis.next()
		} else {
			self.orientation_axis.prev()
		};
	}
}

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

impl Piece for Edge {
	fn position(&self) -> Vector3<i8> {
		self.normal_axis.map_on_slice(Vector3::zeros(), |_| self.position.map(|dir| dir.scalar()))
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
        f.debug_struct("Edge").field("slice_axis", &self.normal_axis).field("position", &self.position).field("oriented", &self.oriented).finish()?;

		let [f1, f2] = self.faces();
		write!(f, " (Faces: {f1}{f2})")?;

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
	pub fn position_from_faces([a, b]: [Face; 2]) -> Result<(Axis, Vector2<Direction>), EdgeFromFacesError> {
		let slice_axis = Axis::other(&a.axis, &b.axis)
			.ok_or(EdgeFromFacesError::SameAxes([a.axis, b.axis]))?;

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
