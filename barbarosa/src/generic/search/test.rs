#![cfg(test)]

use crate::{
    cube3::{heuristics, Cube3},
    cube_n::{moves::wide::Parsable, AxisMove},
    generic::{Alg, Cube, Movable},
};

macro_rules! assert_solves_ida {
    ($cube:ty, $heuristic:expr, $alg:expr) => {
        let alg: Alg<AxisMove> = Alg::parse($alg).unwrap();
        let cube = <$cube>::new_solved().moved(&alg);
        let solution = cube
            .solve_with_heuristic($heuristic)
            .expect("Cube should be solvable");
        assert_eq!(solution, alg.reversed(), "Solution: {solution}, alg: {alg}");
    };
}

#[test]
fn test_solves() {
    assert_solves_ida!(Cube3, heuristics::mus, "R U R' U'");
    assert_solves_ida!(Cube3, heuristics::mus, "R U R F D2 L");
    // assert_solves_ida!(Cube3, heuristics::manhattan, "R' L2 B' U2 D");
}
