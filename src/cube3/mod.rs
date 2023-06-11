use std::{array, collections::HashSet, hash::Hash};

use nalgebra::vector;
use once_cell::sync::Lazy;

use self::{pieces::{Edge, Corner, PieceEnum, Piece}, space::{Direction, Axis}, moves::Move};

pub mod pieces;
pub mod space;
pub mod moves;
pub mod random;

mod test;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cube {
	pub edges: [Edge; 12],
	pub corners: [Corner; 8],
}

static SOLVED_CUBE: Lazy<Cube> = Lazy::new(Cube::construct_solved);

impl Cube {
	/// Creates a new cube in the solved state.
	// TODO: Make this const
	pub fn solved() -> Self {
		SOLVED_CUBE.clone()
	}

	fn construct_solved() -> Self {
		let edges = array::from_fn(|i| {
			let x = if i / 2 % 2 == 0 { Direction::Positive } else { Direction::Negative };
			let y = if i % 2 == 0 { Direction::Positive } else { Direction::Negative };

			let axis = match i {
				0..=3 => Axis::X,
				4..=7 => Axis::Y,
				8..=11 => Axis::Z,
				_ => unreachable!("Index should be in range 0..12"),
			};

			Edge::oriented(axis, vector![x, y])
		});

		let corners = array::from_fn(|i| {
			let coords = [1, 2, 4].map(|axis_index| {
				if (i / axis_index) % 2 == 0 { Direction::Positive } else { Direction::Negative }
			}).into();

			Corner::oriented(coords)
		});

		Self { edges, corners }
	}

	pub fn is_solved(&self) -> bool {
		let solved = Self::solved();
		solved.edges == self.edges && solved.corners == self.corners
	}

	pub fn ensure_consistent(&self) {
		let unique_edges = self.edges.iter().collect::<HashSet<_>>().len();
		debug_assert_eq!(unique_edges, 12, "Edges should be unique, but there are {} duplicates", 12 - unique_edges);

		let unique_corners = self.corners.iter().collect::<HashSet<_>>().len();
		debug_assert_eq!(unique_corners, 8, "Corners should be unique, but there are {} duplicates", 8 - unique_corners);
	}
}

impl IntoIterator for Cube {
	type Item = PieceEnum;
	type IntoIter = std::iter::Chain<std::array::IntoIter<PieceEnum, 12>, std::array::IntoIter<PieceEnum, 8>>;

	fn into_iter(self) -> Self::IntoIter {
		self.edges.map(|edge| edge.into()).into_iter().chain(
			self.corners.map(|corner| corner.into()).into_iter()
		)
	}
}

impl Cube {
	fn move_piece<T: Piece>(piece: &mut T, mov: &Move) {
		if piece.in_face(&mov.face) {
			piece.rotate(&mov.rotation());
		}
	}

	pub fn do_move(&mut self, mov: &Move) {
		self.edges.iter_mut().for_each(|edge| Self::move_piece(edge, mov));
		self.corners.iter_mut().for_each(|corner| Self::move_piece(corner, mov));
	}

	pub fn into_move(self, mov: &Move) -> Self {
		let mut cube = self;
		cube.do_move(mov);
		cube
	}

	pub fn apply_alg<'a>(&mut self, alg: impl Iterator<Item = &'a Move>) {
		alg.into_iter().for_each(|mov| self.do_move(mov));
	}
}

impl Hash for Cube {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.edges.hash(state);
		self.corners.hash(state);
	}
}

impl From<&Vec<Move>> for Cube {
	fn from(alg: &Vec<Move>) -> Self {
		let mut cube = Self::solved();
		cube.apply_alg(alg.iter());
		cube
	}
}
