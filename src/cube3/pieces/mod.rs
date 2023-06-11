use std::fmt::Debug;

use enum_dispatch::enum_dispatch;
use nalgebra::{Vector3, Vector2, vector};
use thiserror::Error;

use super::{space::{Direction, Axis, Vec3, Face}, moves::Rotation};

#[enum_dispatch]
pub enum PieceEnum {
	Corner,
	Edge,
}

#[enum_dispatch(PieceEnum)]
pub trait Piece {
	/// The position of the piece, relative to the center of the cube.
	fn position(&self) -> Vec3;

	/// A piece only rotates. Moves are part of the cube, since
	/// they're just rotations but only to a subset of the pieces.
	fn rotate(&mut self, rotation: &Rotation);

	/// Returns `true` if the piece is on the given face.
	fn in_face(&self, face: &Face) -> bool;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Corner {
	pub position: Vector3<Direction>,

	/// The orientation of the corner piece, determined by the axis of the
	/// sticker that is originally on the X (R-L) axis (usually red-orange)
	pub orientation_axis: Axis,
}

impl Piece for Corner {
    fn position(&self) -> Vec3 {
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
	pub fn oriented(position: Vector3<Direction>) -> Self {
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

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Edge {
	/// The axis of the normal of the face the edge is on (i.e. the axis where the coordinate is 0)
	pub normal_axis: Axis,

	/// The position of the edge on the slice
	pub position: Vector2<Direction>,

	/// TODO: Expalin orientation
	pub oriented: bool,
}

impl Piece for Edge {
	fn position(&self) -> Vec3 {
		// TODO: Uggo
		let mut output = Vec3::zeros();
		self.normal_axis.map_on_slice(&mut output, |_| self.position.map(|dir| dir.scalar()));
		output
	}

	fn rotate(&mut self, rotation: &Rotation) {
		rotation.rotate_edge(self)
	}

	fn in_face(&self, face: &Face) -> bool {
		face.contains_edge(self)

		// TODO: Check alternative implementation
		// self.faces().contains(face)
	}
}

impl TryFrom<[Face; 2]> for Edge {
    type Error = EdgeFromFacesError;

    fn try_from([a, b]: [Face; 2]) -> Result<Self, Self::Error> {
        let slice_axis = Axis::other(a.axis, b.axis)
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
	pub fn oriented(slice_axis: Axis, position: Vector2<Direction>) -> Self {
		Self {
			normal_axis: slice_axis,
			position,
			oriented: true,
		}
	}

	pub fn faces(&self) -> [Face; 2] {
		let x = self.normal_axis.next();
		let y = x.next();

		[
			Face::new(x, self.position[0]),
			Face::new(y, self.position[1]),
		]
	}

	pub fn flipped(mut self) -> Self {
		self.oriented = !self.oriented;
		self
	}

	pub fn flip(&mut self) {
		self.oriented = !self.oriented;
	}
}