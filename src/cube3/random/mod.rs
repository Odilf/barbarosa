use rand::{Rng, seq::SliceRandom};

use super::Cube;

impl Cube {
	pub fn random_with_rng(rng: &mut impl Rng) {
		let mut cube = Cube::solved();

		// Move pieces
		cube.edges.shuffle(rng);
		cube.corners.shuffle(rng); 

		// Fix piece permutations
		let edge_swap_parity = swap_cycles(&cube.edges, &Self::solved().edges) % 2 == 0;
		let corner_swap_parity = swap_cycles(&cube.corners, &Self::solved().corners) % 2 == 0;

		if edge_swap_parity != corner_swap_parity {
			(cube.edges[0], cube.edges[1]) = (cube.edges[1].clone(), cube.edges[0].clone());
		}

		// Flip pieces
		cube.edges.iter_mut().for_each(|edge| edge.oriented = rng.gen());
		cube.corners.iter_mut().for_each(|corner| corner.orientation_axis = rng.gen());

		// Fix edge orientation
		let oriented_edges = cube.edges.iter().filter(|edge| edge.oriented).count();
		if oriented_edges % 2 == 1 {
			cube.edges[0].flip();
		}

		// Fix corner orientation
		let oriented_corners: i32 = cube.corners.iter().map(|corner| corner.orientation_index()).sum();
		let corner_orientation_offset = oriented_corners % 3;
		for _ in 0..corner_orientation_offset {
			cube.corners[0].twist();
		}
	}

	pub fn random() {
		let mut rng = rand::thread_rng();
		Self::random_with_rng(&mut rng);
	}
}

fn swap_cycles<T: Eq, const N: usize>(given: &[T; N], original: &[T; N]) -> i32 {
	let mut permutations: [Option<usize>; N] = [None; N];
	let mut current_index = 0;
	let mut cycles = 0;

	loop {
		if permutations[current_index].is_none() {
			let Some(first_unvisited) = permutations.iter().position(|x| x.is_none()) else {
				return cycles;
			};

			cycles += 1;
			current_index = first_unvisited;
		}

		let next = original.iter().position(|x| x == &given[current_index]).unwrap();
		permutations[current_index] = Some(next);
		current_index = next;
	}
}
