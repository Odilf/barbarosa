//! Sequences of moves.

use std::{fmt::Debug, marker::PhantomData};

use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use strum::IntoEnumIterator;
use thiserror::Error;

use super::{parse, Movable, Move, Parsable};
use crate::generic;

/// An alg. A sequence of moves.
///
/// The name alg sort of means "algorithm", but I don't think it's super accurate to call it an algorithm. It's just a sequence of moves.
/// However, that's the name it's used in the cubing community so I'm using it here.
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alg<M: Move> {
    pub moves: Vec<M>,
}

impl<M: Move> Alg<M> {
    /// Creates a new alg from a vector of moves
    pub fn new(moves: Vec<M>) -> Self {
        Self { moves }
    }

    /// Reverses the alg
    pub fn reversed(&self) -> Self {
        Self::new(self.moves.iter().rev().map(|m| m.inverse()).collect())
    }
}

impl<M: Move + Parsable> Parsable for Alg<M> {
    fn parse(s: &str) -> parse::Result<Self> {
        let moves: parse::Result<Vec<_>> = s.split_whitespace().map(M::parse).collect();
        Ok(Self::new(moves?))
    }
}

// Alg::random()
impl<M> Alg<M>
where
    M: Move,
    Standard: Distribution<M>,
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

impl<M: Move + Debug + IntoEnumIterator, T: Movable<M> + Eq + Clone> TryFrom<Vec<T>> for Alg<M> {
    type Error = TryFromStatesError<M, T>;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        Self::try_from_states(value)
    }
}

impl<M: Move + Debug + IntoEnumIterator> Alg<M> {
    /// Simple implementation to understand where the trait bounds fail, if that happens.
    /// Otherwise you can just use `TryFrom<Vec<T>>` directly.
    pub fn try_from_states<T: Movable<M> + Eq + Clone>(
        states: Vec<T>,
    ) -> Result<Self, TryFromStatesError<M, T>> {
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
            .collect::<Result<Vec<M>, _>>()?;

        Ok(Self::new(alg))
    }
}

/// Error returned when trying to convert a vector of states to an alg
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum TryFromStatesError<M: Move, T: Movable<M> + Eq + Clone> {
    #[error("There is no move connecting {0:?} and {1:?}")]
    NotConnected(T, T, PhantomData<M>),
}

impl<M: Move + Parsable> TryFrom<&str> for Alg<M> {
    type Error = parse::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<M: Move + std::fmt::Display> std::fmt::Display for Alg<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.moves.iter().map(|m| m.to_string()).join(" "))
    }
}

// impl<M: Move> From<M> for Alg<M> {
//     fn from(value: M) -> Self {
//         Self::new(vec![value])
//     }
// }

impl<M: Move, T: Movable<M>> Movable<[M]> for T {
    fn apply(&mut self, m: &[M]) {
        for m in m {
            self.apply(m);
        }
    }
}

impl<M: Move, T: Movable<M>> Movable<Alg<M>> for T {
    fn apply(&mut self, m: &Alg<M>) {
        <T as Movable<[M]>>::apply(self, &m.moves);
    }
}

impl<M: generic::Move> FromIterator<M> for Alg<M> {
    fn from_iter<T: IntoIterator<Item = M>>(iter: T) -> Self {
        // TODO: Revise if this is how you properly do collecting
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
