#![cfg(test)]

use crate::{generic::{Cube, Movable}, cube_n::moves::perms};

use super::Cube2;

#[test]
fn trying_to_permute_edges_doesnt_unsolve() {
	let cube = Cube2::new_solved().moved(&perms::U);
	assert!(cube.is_solved());
}

#[test]
fn two_ts_solves() {
	let cube = Cube2::new_solved()
		.moved(&perms::T)
		.moved(&perms::T);
	
	assert!(cube.is_solved());
}

#[test]
fn four_sexies() {
	let mut cube = Cube2::new_solved();
		
	for _ in 0..6 {
		cube.apply(&perms::SEXY_MOVE);
	}

	assert!(cube.is_solved());
}
