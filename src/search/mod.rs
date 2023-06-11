mod test;
pub mod heuristics;

use pathfinding::directed::astar::astar;

use crate::cube3::{Cube, moves::{Move, alg}};

impl Cube {
	fn successors(&self) -> Vec<(Self, i8)> {
		Move::all().into_iter().map(|mov| {
			let cube = self.clone().into_move(&mov);
			(cube, 1i8)
		}).collect()
	}

	pub fn solve(&self, heuristic: impl Fn(&Self) -> i8) -> Vec<Move> {
		let (states, _cost) = astar(
			self,
			|cube| cube.successors(),
			|cube| heuristic(cube),
			|cube| cube.is_solved(),
		).unwrap();

		alg::try_from_states(states).expect("States should be connected")
	}
}
