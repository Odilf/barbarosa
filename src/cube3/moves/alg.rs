use once_cell::sync::Lazy;
use thiserror::Error;

use crate::cube3::Cube;

use super::Move;

pub fn parse_alg(input: &str) -> Result<Vec<Move>, ()> {
	input.split_whitespace().map(|s| Move::parse(s).map_err(|_| ())).collect()
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
			.ok_or(FromStatesError::StatesNotConnected { from: from.clone(), to: to.clone() })
	}).collect();

	moves
}

#[derive(Debug, Error)]
pub enum FromStatesError {
	#[error("States not connected (from: {from:?}, to: {to:?})")]
	StatesNotConnected { from: Cube, to: Cube },
}

pub fn reverse(alg: Vec<Move>) -> Vec<Move> {
	alg.into_iter().rev().map(|mov| mov.reversed()).collect()
}
