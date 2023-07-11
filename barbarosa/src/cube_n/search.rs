use crate::{
    cube3::Cube3,
    generic::{
        search::{ida::IDASearcher, Searcher},
        Alg, Movable,
    },
};

use super::AxisMove;

impl Cube3 {
    pub fn solve_with_heuristic(&self, heuristic: impl Fn(&Self) -> f32) -> Option<Alg<AxisMove>> {
        let searcher = IDASearcher::new(
            heuristic,
            |cube| AxisMove::all().map(|mov| (cube.clone().moved(&mov), mov)),
            20,
        );

        searcher.solve(self)
    }
}
