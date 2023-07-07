//! Rubik's cube searching (and solving).

use std::{fmt::Debug, hash::Hash};

use strum::IntoEnumIterator;

use crate::generic::{alg::Alg, Cube, Movable, Move};

mod test;

/// Something that can be searched.
///
/// This trait is auto-implemented for all cubes that can be moved and hashed.
pub trait Searchable<M: Move + Debug + IntoEnumIterator>: Cube + Hash + Movable<M> {
    /// Solves the cube using A* with the given heuristic.
    ///
    /// Currently it can solve 5 moves in ~2.5s.
    ///
    /// To get an optimal solution, the heuristic must be admissible. That is,
    /// it must never overestimate the number of moves required to solve the cube.
    ///
    /// See [crate::cube3::heuristics] for some available heuristics.
    fn solve_with_heuristic(&self, heuristic: impl Fn(&Self) -> i8) -> Alg<M>
    where
        Self: 'static,
    {
        let (states, _cost) = pathfinding::directed::astar::astar(
            self,
            |cube| {
                successors::<M, Self>(cube)
                    .into_iter()
                    .map(|cube| (cube, 1))
                    .collect::<Vec<_>>()
            },
            |cube| heuristic(cube),
            |cube| cube.is_solved(),
        )
        .unwrap();

        Alg::try_from_states(states).expect("States should be connected")
    }
}

impl<M, C> Searchable<M> for C
where
    M: Move + Debug + IntoEnumIterator,
    C: Cube + Hash + Movable<M>,
{
}

/// Returns every possible state reached by making a move on the given cube.
pub fn successors<M, T>(cube: &T) -> Vec<T>
where
    M: Move + IntoEnumIterator,
    T: Movable<M> + Clone,
{
    M::iter().map(|m| cube.clone().moved(&m)).collect()
}
