#![cfg(test)]

use crate::{
    cube3::{heuristics, Cube3},
    cube_n::{moves::wide::Parsable, AxisMove},
    generic::{Alg, Cube, Movable},
};

macro_rules! assert_solves_ida {
    ($cube:ty, $alg:expr) => {
        let alg: Alg<AxisMove> = Alg::parse($alg).unwrap();
        let cube = <$cube>::new_solved().moved(&alg);
        let solution = cube.solve_with_heuristic(heuristics::mus);
        assert_eq!(solution.unwrap(), alg.reversed());
    };
}

#[test]
fn test_solved() {
    assert_solves_ida!(Cube3, "R U R' U'");
}

#[test]
fn lolwut() {
    let alg: Alg<AxisMove> = Alg::parse("R U R2 U2").unwrap();
    let cube = <Cube3>::new_solved().moved(&alg);
    let solution = cube.solve_with_heuristic(heuristics::mus);
    assert_eq!(solution.unwrap(), alg.reversed());
}
