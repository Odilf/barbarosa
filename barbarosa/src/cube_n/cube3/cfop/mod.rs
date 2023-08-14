//! The CFOP method

pub mod cross;

mod test;

use crate::{
    cube_n::AxisMove,
    generic::{search::Solver, Alg},
};

use super::Cube3;

struct CfopSolver {}

impl Solver<Cube3, AxisMove> for CfopSolver {
    fn solve(&self, _cube: &Cube3) -> Option<Alg<AxisMove>> {
        todo!()
    }
}
