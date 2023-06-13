//! "Algorithms" (collections of moves)
//!
//! The standard term in the cubing for a collection of moves is "algorithm".
//! Some people have said that this is not really a very accurate use of the
//! word "algorithm". However, it is the standard term, and it is what I will
//! use in this crate.

use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use thiserror::Error;

use crate::cube3::Cube;

use super::{Move, MoveParseError};

/// Parses an algorithm from a string.
pub fn parse_alg(input: &str) -> Result<Vec<Move>, AlgParseError> {
    input
        .split_whitespace()
        .map(|s| Move::parse(s).map_err(AlgParseError::MoveParseError))
        .collect()
}

/// An error that can occur when parsing an algorithm.
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum AlgParseError {
    #[error("Move parse error: {0}")]
    MoveParseError(MoveParseError),
}

/// Collection of "permutations". Actually, these are not permutations, but
/// algorithms that permute the pieces in a certain way.
///
/// As an aside, by that logic any algorithm is a permutation, since it
/// permutes the pieces in some way. However, the term "permutation" is
/// usually used to refer to algorithms that permute only a few pieces
/// in a easily describable way.
// TODO: Make all this const
pub mod perm {
    use super::*;

    /// R U R' U'. Yeee
    pub static SEXY_MOVE: Lazy<Vec<Move>> = Lazy::new(|| parse_alg("R U R' U'").unwrap());

    /// The [T perm](http://algdb.net/puzzle/333/pll/t). Swaps RUF-RUB and RU-LU
    pub static T: Lazy<Vec<Move>> =
        Lazy::new(|| parse_alg("R U R' U' R' F R2 U' R' U' R U R' F'").unwrap());

    /// The [U perm](http://algdb.net/puzzle/333/pll/ua) (specifically Ua). Cycles RUF->RUB->RUL
    pub static U: Lazy<Vec<Move>> = Lazy::new(|| parse_alg("R2 U' R' U' R U R U R U' R").unwrap());
}

/// Creates a `Vec` of `Move`s from a `Vec` of `Cube`s.
pub fn try_from_states(states: Vec<Cube>) -> Result<Vec<Move>, FromStatesError> {
    let moves = states
        .windows(2)
        .map(|window| {
            let from = &window[0];
            let to = &window[1];

            Move::all()
                .into_iter()
                .find(|mov| &from.clone().moved(mov) == to)
                .ok_or(FromStatesError::StatesNotConnected(
                    Box::new(from.clone()),
                    Box::new(to.clone()),
                ))
        })
        .collect();

    moves
}

/// An error that can occur when trying to create a [Vec] of [Move]s from a
/// [Vec] of [Cube] states.
#[derive(Debug, Error)]
pub enum FromStatesError {
    /// There isn't a move that goes from `from` to `to`.
    #[error("States not connected (from: {0:?}, to: {1:?})")]
    StatesNotConnected(Box<Cube>, Box<Cube>),
}

/// Reverses an algorithm.
pub fn reverse(alg: Vec<Move>) -> Vec<Move> {
    alg.into_iter().rev().map(|mov| mov.reversed()).collect()
}

/// Creates `Vec` of random `Move`s of the given size.
pub fn random(size: usize) -> Vec<Move> {
    let mut rng = rand::thread_rng();

    (0..size)
        .map(|_| {
            Move::all()
                .choose(&mut rng)
                .expect("`Moves::all()` has more than zero moves")
                .clone()
        })
        .collect()
}
