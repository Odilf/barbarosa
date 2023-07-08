//! Sequences of moves.

use std::{fmt::Debug, marker::PhantomData};

use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use strum::IntoEnumIterator;
use thiserror::Error;

use super::{moves::IntoMove, parse, Movable, Move, Parsable};
use crate::generic;

/// An alg. A sequence of moves.
///
/// The name alg sort of means "algorithm", but I don't think it's super accurate to call it an algorithm. It's just a sequence of moves.
/// However, that's the name it's used in the cubing community so I'm using it here.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alg<T: IntoMove> {
    pub moves: Vec<T::Move>,
}

impl<T: IntoMove> Alg<T> {
    /// Creates a new alg from a vector of moves
    pub fn new(moves: Vec<T::Move>) -> Self {
        Self { moves }
    }

    /// Reverses the alg
    pub fn reversed(&self) -> Self {
        Self::new(self.moves.iter().rev().map(|m| m.inverse()).collect())
    }
}

impl<T: IntoMove> Parsable for Alg<T>
where
    T::Move: Parsable,
{
    fn parse(s: &str) -> parse::Result<Self> {
        let moves: parse::Result<Vec<_>> = s.split_whitespace().map(T::Move::parse).collect();
        Ok(Self::new(moves?))
    }
}

// Alg::random()
impl<T> Alg<T>
where
    T: IntoMove,
    Standard: Distribution<T::Move>,
{
    /// Creates a random algorithm of the given length
    pub fn random(length: usize) -> Self {
        Self::random_with_rng(length, &mut rand::thread_rng())
    }

    /// Same as [Self::random()], but specifying RNG
    pub fn random_with_rng(length: usize, rng: &mut impl Rng) -> Self {
        Self::new((0..length).map(|_| rng.gen()).collect())
    }
}

impl<T: IntoMove> Alg<T>
where
    T::Move: Debug + IntoEnumIterator,
{
    /// Simple implementation to understand where the trait bounds fail, if that happens.
    /// Otherwise you can just use `TryFrom<Vec<T>>` directly.
    pub fn try_from_states<C: Movable<T::Move> + Eq + Clone>(
        states: Vec<C>,
    ) -> Result<Self, TryFromStatesError<T::Move, C>> {
        let alg = states
            .windows(2)
            .map(|window| {
                let [from, to] = window else {
                    unreachable!("windows(2) always returns slices of length 2")
                };

                generic::moves::connect(from, to).ok_or_else(|| {
                    TryFromStatesError::NotConnected(from.to_owned(), to.to_owned(), PhantomData)
                })
            })
            .collect::<Result<Vec<T::Move>, _>>()?;

        Ok(Self::new(alg))
    }
}

// TODO: Change this implementation
impl<M: Move + Debug + IntoEnumIterator, T: Movable<M> + Eq + Clone> TryFrom<Vec<T>> for Alg<M> {
    type Error = TryFromStatesError<M, T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::try_from_states(value)
    }
}

/// Error returned when trying to convert a vector of states to an alg
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum TryFromStatesError<M: Move, T: Movable<M> + Eq + Clone> {
    #[error("There is no move connecting {0:?} and {1:?}")]
    NotConnected(T, T, PhantomData<M>),
}

// Parsing using `TryFrom`
impl<T: IntoMove> TryFrom<&str> for Alg<T>
where
    T::Move: Parsable,
{
    type Error = parse::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

// Display
impl<T: IntoMove> std::fmt::Display for Alg<T>
where
    T::Move: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.moves.iter().map(|m| m.to_string()).join(" "))
    }
}

// Make movable types able to be moved by slices of moves
impl<M: Move, C: Movable<M>> Movable<[M]> for C {
    fn apply(&mut self, m: &[M]) {
        for m in m {
            self.apply(m);
        }
    }
}

impl<T: IntoMove, C: Movable<[T::Move]>> Movable<Alg<T>> for C {
    fn apply(&mut self, m: &Alg<T>) {
        self.apply(&m.moves);
    }
}

impl<T: IntoMove> FromIterator<T::Move> for Alg<T> {
    fn from_iter<I: IntoIterator<Item = T::Move>>(iter: I) -> Self {
        let moves: Vec<_> = iter.into_iter().collect();
        Self::new(moves)
    }
}

// For once_cell.
// Kinda ugly, shouldn't be necessary.
impl<M: Move, T: Movable<M>> Movable<Lazy<Alg<M>>> for T {
    fn apply(&mut self, m: &Lazy<Alg<M>>) {
        <T as Movable<Alg<M>>>::apply(self, m);
    }
}
