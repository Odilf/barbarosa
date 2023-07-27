//! Iterative deepening A*

use std::{collections::HashSet, hash::Hash};

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
        cube: &C,
        is_target: &impl Fn(&C) -> bool,
        current_cost: f32,
        bound: &f32,
        min_exceeded: &mut f32,
        visited: &mut HashSet<C>,
    ) -> Option<Alg<M>> {
        if visited.contains(cube) {
            return None;
        }

        visited.insert(cube.clone());

        let new_cost = current_cost + (self.heuristic)(cube);

        if new_cost > *bound {
            if new_cost < *min_exceeded {
                *min_exceeded = new_cost;
            }

            return None;
        }

        if is_target(cube) {
            return Some(Alg::empty());
        }

        for (successor, mov) in (self.successors)(cube) {
            let new_search = self.search_impl(
                &successor,
                is_target,
                current_cost + 1.0,
                bound,
                min_exceeded,
                visited,
            );

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
    C: Cube + Movable<M> + Hash,
    M: Move,
    Heuristic: Fn(&C) -> f32,
    Successors: Fn(&C) -> Iter,
    Iter: IntoIterator<Item = (C, M)>,
{
    fn search(&self, cube: &C, is_target: impl Fn(&C) -> bool) -> Option<Alg<M>> {
        let mut bound = (self.heuristic)(cube);
        for _ in 0..=self.max_depth {
            let mut min_exceeded = bound + 1.0;
            let mut visited = HashSet::new();

            let t = self.search_impl(
                cube,
                &is_target,
                0.0,
                &bound,
                &mut min_exceeded,
                &mut visited,
            );

            match t {
                Some(mut solution) => {
                    solution.moves.reverse();
                    return Some(solution);
                }
                None => bound = min_exceeded,
            }
        }

        None
    }
}
