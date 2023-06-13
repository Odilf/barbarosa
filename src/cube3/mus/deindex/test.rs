#![cfg(test)]

use crate::cube3::Cube;

use super::*;

#[test]
fn index_deindex_solved() {
	let cube = Cube::solved();
	let indices = cube.indices();
	let deindexed_cube = Cube::from_indices(indices);

	assert_eq!(cube, &deindexed_cube);
}

#[test]
fn random_indexes_deindexes() {
	let cube = Cube::random();
	let indices = cube.indices();
	println!("Indexed!");
	let deindexed_cube = Cube::from_indices(indices);
	println!("Deindexed!");

	assert_eq!(cube, deindexed_cube);
}

#[test]
fn index_deindex_journey() {
    for _ in 0..10 {
        
    }
}
