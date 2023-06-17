#![cfg(test)]

use crate::{
    cube3::heuristics,
    cube_n::{AxisMove, Cube3},
    generic::{parse::Parsable, Cube, Movable},
};

use super::*;

#[test]
fn test_solved() {
    let cube = Cube3::solved();
    let solution: Alg<AxisMove> = cube.solve_with_heuristic(heuristics::manhattan);
    assert_eq!(solution.moves.len(), 0);
}

fn assert_solves_alg(alg: Alg<AxisMove>, heuristic: impl Fn(&Cube3) -> i8) {
    let mut cube = Cube3::new_solved().moved(&alg);

    let solution = cube.solve_with_heuristic(heuristic);

    assert_eq!(solution, alg.reversed());
}

// TODO: Make this test not uggo
#[test]
fn test_solves_manhattan() {
    let algs = ["R2", "R", "R'", "R U", "R U R' U'", "R U R' U' F"];

    for alg in algs {
        assert_solves_alg(<Alg<AxisMove>>::parse(alg).unwrap(), heuristics::manhattan);
    }
}
