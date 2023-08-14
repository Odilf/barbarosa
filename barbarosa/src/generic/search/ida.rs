//! Iterative deepening A*

use std::hash::Hash;

use crate::generic::{Alg, Cube, Movable, Move};

use super::Searcher;

/// An IDA* searcher
pub struct IDASearcher<C, M, Heuristic, Successors, Iter>
where
    C: Cube + Movable<M>,
    M: Move,
    Heuristic: Fn(&C) -> f32,
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
    C: Cube + Movable<M> + Hash,
    M: Move,
    Heuristic: Fn(&C) -> f32,
    Successors: Fn(&C) -> Iter,
    Iter: IntoIterator<Item = (C, M)>,
{
    /// Creates a new IDA* seracher
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
        path: &mut Vec<C>,
        is_target: &impl Fn(&C) -> bool,
        current_cost: f32,
        bound: f32,
        min_exceeded: &mut f32,
    ) -> Option<Alg<M>> {
        let cube = path
            .last()
            .expect("Path should be populated before calling this function");
        let new_cost = current_cost + (self.heuristic)(cube);

        if new_cost > bound {
            if new_cost < *min_exceeded {
                *min_exceeded = new_cost;
            }

            return None;
        }

        if is_target(cube) {
            return Some(Alg::empty());
        }

        for (successor, mov) in (self.successors)(cube) {
            if path.contains(&successor) {
                continue;
            }

            path.push(successor);

            let new_cost = current_cost + 1.0;
            let new_search = self.search_impl(path, is_target, new_cost, bound, min_exceeded);

            if let Some(mut solution) = new_search {
                solution.moves.push(mov);
                return Some(solution);
            }

            path.pop();
        }

        None
    }
}

impl<C, M, Heuristic, Successors, Iter> Searcher<C, M>
    for IDASearcher<C, M, Heuristic, Successors, Iter>
where
    C: Cube + Movable<M> + Hash,
    M: Move,
    Heuristic: Fn(&C) -> f32,
    Successors: Fn(&C) -> Iter,
    Iter: IntoIterator<Item = (C, M)>,
{
    fn search(&self, cube: &C, is_target: impl Fn(&C) -> bool) -> Option<(Alg<M>, C)> {
        let mut bound = (self.heuristic)(cube);

        for _ in 0..=self.max_depth {
            let mut min_exceeded = bound + 1.0;
            let mut path = Vec::with_capacity(20);

            path.push(cube.clone());

            let t = self.search_impl(&mut path, &is_target, 0.0, bound, &mut min_exceeded);

            match t {
                Some(mut solution) => {
                    solution.moves.reverse();
                    return Some((solution, path.into_iter().last()?));
                }
                None => bound = min_exceeded,
            }
        }

        None
    }
}
