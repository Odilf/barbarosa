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

	let rotated = Rotation::new(Axis::X, Amount::Reverse).rotate_face(face.clone());
	assert_eq!(rotated, Face::F);
}

#[test]
fn rotates_d_face() {
	let face = Face::D;

	let rotated = Rotation::new(Axis::X, Amount::Single).rotate_face(face.clone());
	assert_eq!(rotated, Face::F);

	let rotated = Rotation::new(Axis::X, Amount::Double).rotate_face(face.clone());
	assert_eq!(rotated, Face::U);

	let rotated = Rotation::new(Axis::X, Amount::Reverse).rotate_face(face.clone());
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

	Rotation::new(Axis::X, Amount::Reverse).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::R, Face::B]).unwrap());

	let mut edge = Edge::try_from([Face::U, Face::B]).unwrap();

	Rotation::new(Axis::Y, Amount::Single).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::U, Face::R]).unwrap());

	dbg!(Edge::try_from([Face::U, Face::L]).unwrap());

	Rotation::new(Axis::Y, Amount::Double).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::U, Face::L]).unwrap());

	Rotation::new(Axis::Y, Amount::Reverse).rotate_edge(&mut edge);
	assert_eq!(edge, Edge::try_from([Face::U, Face::F]).unwrap());
}

#[test]
fn parses_moves() {
	assert_eq!(Move::parse("R2").unwrap(), Move::new(Face::R, Amount::Double));
	assert_eq!(Move::parse("L'").unwrap(), Move::new(Face::L, Amount::Reverse));
	assert_eq!(Move::parse("D").unwrap(), Move::new(Face::D, Amount::Single));
}

#[test]
fn six_sexy_moves_solves_cube() {
	let mut cube = Cube::solved();

	for _ in 0..6 {
		cube.apply_alg(perm::SEXY_MOVE.iter());
	}

	assert_eq!(cube, Cube::solved());
}

#[test]
fn two_t_perms_solve_cube() {
	let mut cube = Cube::solved();

	for _ in 0..2 {
		cube.apply_alg(perm::T.iter());
	}

	assert_eq!(cube, Cube::solved());
}
