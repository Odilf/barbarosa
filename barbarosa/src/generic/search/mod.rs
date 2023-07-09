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

// use std::{collections::VecDeque, hash::Hash};

// use crate::generic::{Alg, Cube, Movable};

// use super::Move;

// mod test;

// pub fn ida_star<M, C, Iter>(
//     cube: &C,
//     heuristic: impl Fn(&C) -> i8,
//     successors: impl Fn(&C) -> Iter,
//     is_goal: impl Fn(&C) -> bool,
//     max_depth: i32,
// ) -> Option<Alg<M>>
// where
//     M: Move,
//     C: Cube + Movable<M>,
//     Iter: IntoIterator<Item = (C, M)>,
// {
//     let mut bound = heuristic(cube);
//     for _ in 0..=max_depth {
//         let t = search(
//             cube,
//             &heuristic,
//             &successors,
//             &is_goal,
//             0,
//             bound,
//         );

//         match t {
//             Some(mut solution) => {
//                 solution.moves.reverse();
//                 return Some(solution);
//             }
//             None => bound += 1,
//         }
//     }

//     None
// }

// fn search<M, C, Iter>(
//     cube: &C,
//     heuristic: &impl Fn(&C) -> i8,
//     successors: &impl Fn(&C) -> Iter,
//     is_goal: &impl Fn(&C) -> bool,
//     current_cost: i8,
//     bound: i8,
// ) -> Option<Alg<M>>
// where
//     M: Move,
//     C: Cube + Movable<M>,
//     Iter: IntoIterator<Item = (C, M)>,
// {
//     let f = current_cost + heuristic(cube);

//     if f > bound {
//         return None;
//     }

//     if is_goal(cube) {
//         return Some(Alg::empty());
//     }

//     // let mut min = i8::MAX;
//     for (successor, mov) in successors(cube) {
//         let new_search = search(
//             &successor,
//             heuristic,
//             successors,
//             is_goal,
//             current_cost + 1,
//             bound,
//         );

//         if let Some(mut solution) = new_search {
//             solution.moves.push(mov);
//             return Some(solution);
//         }

//         // min = min.min(heuristic(&succ));
//     }

//     None
// }
