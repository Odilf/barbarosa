//! 3x3x3 Rubik's cube searching.

use std::hash::Hash;

use crate::generic::{alg::Alg, Cube, Movable, Move};

mod test;

/// Something that can be searched.
///
/// This trait is auto-implemented for all cubes that can be moved and hashed.
pub trait Searchable<M: Move>: Cube + Hash + Movable<M> {
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
                cube.successors()
                    .into_iter()
                    .map(|cube| (cube, 1i8))
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
    M: Move,
    C: Cube + Hash + Movable<M>,
{
}
