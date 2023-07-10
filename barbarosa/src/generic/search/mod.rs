//! Cube searching (and solving).

pub mod ida;
mod test;

use super::{Alg, Cube, Movable, Move};

pub trait Searcher<C, M>
where
    C: Cube + Movable<M>,
    M: Move,
{
    fn search(&self, cube: &C, is_target: impl Fn(&C) -> bool) -> Option<Alg<M>>;

    fn solve(&self, cube: &C) -> Option<Alg<M>>
    where
        C: 'static,
    {
        self.search(cube, |c| c.is_solved())
    }
}
