//! Cube searching

use crate::{
    cube3::Cube3,
    generic::{
        search::{ida::IDASearcher, Solver},
        Alg, Movable,
    },
};

use super::AxisMove;

impl Cube3 {
    /// Solves the cube using the specified heuristic
    pub fn solve_with_heuristic(&self, heuristic: impl Fn(&Self) -> f32) -> Option<Alg<AxisMove>> {
        let searcher = IDASearcher::new(heuristic, Cube3::successors, 200);

        searcher.solve(self)
    }
}
