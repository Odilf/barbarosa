//! Cube searching (and solving).

pub mod ida;
mod test;

use super::{Alg, Cube, Movable, Move};

/// A type that can solve a cube `C` using a move `M`.
///
/// [`Solver`] is automatically implemented for all [`Searcher`]s
pub trait Solver<C: Cube + Movable<M>, M: Move> {
    /// Tries to find a solution to the given cube.
    fn solve(&self, cube: &C) -> Option<Alg<M>>;
}

/// A type that can search a cube `C` using a move `M`.
///
/// [`Searcher`]s implement automatically [`Solver`
pub trait Searcher<C: Cube + Movable<M>, M: Move> {
    /// Tries to find a state of `C` such that `is_target` evaluates to `true`.
    fn search(&self, cube: &C, is_target: impl Fn(&C) -> bool) -> Option<(Alg<M>, C)>;
}

// All searchers implement solver
impl<C: Cube + Movable<M> + 'static, M: Move, S: Searcher<C, M>> Solver<C, M> for S {
    fn solve(&self, cube: &C) -> Option<Alg<M>> {
        let (solution, _solved) = self.search(cube, C::is_solved)?;

        debug_assert!(_solved.is_solved());

        Some(solution)
    }
}
