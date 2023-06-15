//! 3x3x3 Rubik's cube searching.

mod test;

use pathfinding::directed::astar::astar;

use crate::cube3::{
    moves::{alg, do_move, Move},
    Cube, Piece,
};

pub fn successors<P: Piece + Clone, const N: usize>(pieces: &[P; N]) -> Vec<([P; N], i8)> {
    Move::all()
        .into_iter()
        .map(|mov| {
            let mut pieces = pieces.clone();
            do_move(&mut pieces, &mov);
            (pieces, 1i8)
        })
        .collect()
}

impl Cube {
    fn successors(&self) -> Vec<(Self, i8)> {
        Move::all()
            .into_iter()
            .map(|mov| {
                let cube = self.clone().moved(&mov);
                (cube, 1i8)
            })
            .collect()
    }

    /// Solves the cube using A* with the given heuristic.
    ///
    /// Currently it can solve 5 moves in ~2.5s.
    ///
    /// To get an optimal solution, the heuristic must be admissible. That is,
    /// it must never overestimate the number of moves required to solve the cube.
    ///
    /// See [`heuristics`] for some available heuristics.
    pub fn solve_with_heuristic(&self, heuristic: impl Fn(&Self) -> i8) -> Vec<Move> {
        let (states, _cost) = astar(
            self,
            |cube| cube.successors(),
            |cube| heuristic(cube),
            |cube| cube.is_solved(),
        )
        .unwrap();

        alg::try_from_states(states).expect("States should be connected")
    }
}
