#![cfg(test)]

use crate::cube3::{Cube, moves::alg::perm};

use super::*;

#[test]
fn rotates_u_face() {
	let face = Face::U;

	let rotated = Rotation::new(Axis::X, Amount::Single).rotate_face(face.clone());
	assert_eq!(rotated, Face::B);

	let rotated = Rotation::new(Axis::X, Amount::Double).rotate_face(face.clone());
	assert_eq!(rotated, Face::D);

	let rotated = Rotation::new(Axis::X, Amount::Inverse).rotate_face(face.clone());
	assert_eq!(rotated, Face::F);
}

#[test]
fn rotates_d_face() {
	let face = Face::D;

	let rotated = Rotation::new(Axis::X, Amount::Single).rotate_face(face.clone());
	assert_eq!(rotated, Face::F);

	let rotated = Rotation::new(Axis::X, Amount::Double).rotate_face(face.clone());
	assert_eq!(rotated, Face::U);

	let rotated = Rotation::new(Axis::X, Amount::Inverse).rotate_face(face.clone());
	assert_eq!(rotated, Face::B);
}

#[test]
// #[ignore = ":("]
fn rotates_edges() {
	let mut edge = Edge::try_from([Face::R, Face::F]).unwrap();

	Rotation::new(Axis::X, Amount::Single).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::R, Face::U]).unwrap().flipped());

	Rotation::new(Axis::X, Amount::Double).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::R, Face::D]).unwrap().flipped());

	Rotation::new(Axis::X, Amount::Inverse).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::R, Face::B]).unwrap());

	let mut edge = Edge::try_from([Face::U, Face::B]).unwrap();

	Rotation::new(Axis::Y, Amount::Single).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::U, Face::R]).unwrap());

	dbg!(Edge::try_from([Face::U, Face::L]).unwrap());

	Rotation::new(Axis::Y, Amount::Double).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::U, Face::L]).unwrap());

	Rotation::new(Axis::Y, Amount::Inverse).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::U, Face::F]).unwrap());
}

#[test]
fn parses_moves() {
	assert_eq!(Move::parse("R2").unwrap(), Move::new(Face::R, Amount::Double));
	assert_eq!(Move::parse("L'").unwrap(), Move::new(Face::L, Amount::Inverse));
	assert_eq!(Move::parse("D").unwrap(), Move::new(Face::D, Amount::Single));
}

#[test]
fn errors_on_invalid_algs() {
	let inputs = [
		"P",
		"µ",
		"R2'",
	];
	
	for input in inputs {
		assert!(Move::parse(input).is_err());
	}
}

#[test]
fn errors_completely_when_parsing_invalid_move_in_alg() {
	let inputs = [
		"R2 P",
		"R2 µ",
		"R2 R2'",
	];
	
	for input in inputs {
		assert!(alg::parse_alg(input).is_err());
	}
}

#[test]
fn six_sexy_moves_solves_cube() {
	let mut cube = Cube::new_solved();

	for _ in 0..6 {
		cube.apply_alg(perm::SEXY_MOVE.iter());
	}

	assert_eq!(cube, *Cube::solved());
}

#[test]
fn two_t_perms_solve_cube() {
	let mut cube = Cube::new_solved();

	for _ in 0..2 {
		cube.apply_alg(perm::T.iter());
	}

	assert_eq!(cube, *Cube::solved());
}

#[test]
fn alg_and_inverse_solve_cube() {
	let alg = alg::random(30);

	let mut cube = Cube::new_solved();

	cube.apply_alg(alg.iter());
	cube.apply_alg(alg::reverse(alg).iter());

	assert!(cube.is_solved());
}

