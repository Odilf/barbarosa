//! Cube searching (and solving).

pub mod ida;
mod test;

use super::{Alg, Cube, Movable, Move};

/// A type that can search a cube `C` using a mvoe `M`
pub trait Searcher<C, M>
where
    C: Cube + Movable<M>,
    M: Move,
{
    /// Tries to find a state of `C` such that `is_target` evaluates to `true`.
    fn search(&self, cube: &C, is_target: impl Fn(&C) -> bool) -> Option<Alg<M>>;

    /// Solves `C` (so [`Self::search`] with `is_target = C::is_solved`)
    fn solve(&self, cube: &C) -> Option<Alg<M>>
    where
        C: 'static,
    {
        self.search(cube, C::is_solved)
    }
}
