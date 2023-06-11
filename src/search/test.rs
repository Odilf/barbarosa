#![cfg(test)]

use crate::cube3::moves::alg::{parse_alg, reverse};

use super::*;

#[test]
fn test_solved() {
	let cube = Cube::solved();
	let solution = cube.solve(heuristics::manhattan);
	assert_eq!(solution.len(), 0);
}

fn assert_solves_alg(alg: Vec<Move>, heuristic: impl Fn(&Cube) -> i8) {
	let mut cube = Cube::solved();
	cube.apply_alg(alg.iter());

	let solution = cube.solve(heuristic);

	assert_eq!(solution, reverse(alg));
}

#[test]
fn test_solves_manhattan() {
	let algs = [
		"R2",
		"R",
		"R'",
		"R U",
		"R U R' U'",
		"R U R' U' F",
	];

	for alg in algs {
		assert_solves_alg(
			parse_alg(alg).unwrap(),
			heuristics::manhattan,
		);
	}
}