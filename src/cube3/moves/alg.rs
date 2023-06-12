use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use thiserror::Error;

use crate::cube3::Cube;

use super::{Move, MoveParseError};

pub fn parse_alg(input: &str) -> Result<Vec<Move>, AlgParseError> {
	input.split_whitespace()
		.map(|s| Move::parse(s)
			.map_err(AlgParseError::MoveParseError))
		.collect()
}

#[derive(Debug, Error)]
pub enum AlgParseError {
	#[error("Move parse error: {0}")]
	MoveParseError(MoveParseError),
}

pub mod perm {
    use super::*;

	pub static SEXY_MOVE: Lazy<Vec<Move>> = Lazy::new(|| parse_alg("R U R' U'").unwrap());
	pub static T: Lazy<Vec<Move>> = Lazy::new(|| parse_alg("R U R' U' R' F R2 U' R' U' R U R' F'").unwrap());
	pub static U: Lazy<Vec<Move>> = Lazy::new(|| parse_alg("R2 U' R' U' R U R U R U' R").unwrap());
}

pub fn try_from_states(states: Vec<Cube>) -> Result<Vec<Move>, FromStatesError> {
	let moves = states.windows(2).map(|window| {
		let from = &window[0];
		let to = &window[1];

		Move::all().into_iter()
			.find(|mov| &from.clone().into_move(mov) == to)
			.ok_or(FromStatesError::StatesNotConnected { from: Box::new(from.clone()), to: Box::new(to.clone()) })
	}).collect();

	moves
}

#[derive(Debug, Error)]
pub enum FromStatesError {
	#[error("States not connected (from: {from:?}, to: {to:?})")]
	StatesNotConnected { from: Box<Cube>, to: Box<Cube> },
}

pub fn reverse(alg: Vec<Move>) -> Vec<Move> {
	alg.into_iter().rev().map(|mov| mov.reversed()).collect()
}

/// Creates `Vec` of random `Move`s of the given size.
pub fn random(size: usize) -> Vec<Move> {
	let mut rng = rand::thread_rng();

	(0..size)
		.map(|_| Move::all()
			.choose(&mut rng)
			.expect("`Moves::all()` has more than zero moves")
			.clone()
		)
		.collect()
}
