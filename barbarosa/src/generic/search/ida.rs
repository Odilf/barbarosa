use crate::generic::{Alg, Cube, Movable, Move};

use super::Searcher;

pub struct IDASearcher<C, M, Heuristic, Successors, Iter>
where
    C: Cube + Movable<M>,
    M: Move,
    Heuristic: Fn(&C) -> i8,
    Successors: Fn(&C) -> Iter,
    Iter: IntoIterator<Item = (C, M)>,
{
    heuristic: Heuristic,
    successors: Successors,
    max_depth: i32,
    _cube_marker: std::marker::PhantomData<C>,
}

impl<C, M, Heuristic, Successors, Iter> IDASearcher<C, M, Heuristic, Successors, Iter>
where
    C: Cube + Movable<M>,
    M: Move,
    Heuristic: Fn(&C) -> i8,
    Successors: Fn(&C) -> Iter,
    Iter: IntoIterator<Item = (C, M)>,
{
    pub fn new(heuristic: Heuristic, successors: Successors, max_depth: i32) -> Self {
        Self {
            heuristic,
            successors,
            max_depth,
            _cube_marker: std::marker::PhantomData,
        }
    }

    fn search_impl(
        &self,
        cube: &C,
        is_target: &impl Fn(&C) -> bool,
        current_cost: i8,
        bound: i8,
    ) -> Option<Alg<M>>
    where
        C: Cube + Movable<M>,
        M: Move,
    {
        let f = current_cost + (self.heuristic)(cube);

        if f > bound {
            return None;
        }

        if is_target(cube) {
            return Some(Alg::empty());
        }

        for (successor, mov) in (self.successors)(cube) {
            let new_search = self.search_impl(&successor, is_target, current_cost + 1, bound);

            if let Some(mut solution) = new_search {
                solution.moves.push(mov);
                return Some(solution);
            }
        }

        None
    }
}

impl<C, M, Heuristic, Successors, Iter> Searcher<C, M>
    for IDASearcher<C, M, Heuristic, Successors, Iter>
where
    C: Cube + Movable<M>,
    M: Move,
    Heuristic: Fn(&C) -> i8,
    Successors: Fn(&C) -> Iter,
    Iter: IntoIterator<Item = (C, M)>,
{
    fn search(&self, cube: &C, is_target: impl Fn(&C) -> bool) -> Option<Alg<M>>
    where
        C: Cube + Movable<M>,
        M: Move,
    {
        let mut bound = (self.heuristic)(cube);
        for _ in 0..=self.max_depth {
            let t = self.search_impl(cube, &is_target, 0, bound);

            match t {
                Some(mut solution) => {
                    solution.moves.reverse();
                    return Some(solution);
                }
                None => bound += 1,
            }
        }

        None
    }
}
