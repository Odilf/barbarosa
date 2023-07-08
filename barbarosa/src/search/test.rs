#![cfg(test)]

use rand::{rngs::StdRng, SeedableRng};

use crate::{
    cube3::heuristics,
    cube_n::{AxisMove, Cube3},
    generic::{Cube, Movable},
};

use super::*;

#[test]
fn test_solved() {
    let cube = Cube3::solved();
    let solution: Alg<AxisMove> = cube.solve_with_heuristic(heuristics::manhattan);
    assert_eq!(solution.moves.len(), 0);
}

fn assert_solves_alg(alg: Alg<AxisMove>, heuristic: impl Fn(&Cube3) -> i8) {
    println!("Solving {alg}");
    let cube = Cube3::new_solved().moved(&alg);

    let solution: Alg<AxisMove> = cube.solve_with_heuristic(heuristic);

    assert!(cube.moved(&solution).is_solved());
}

#[test]
fn test_solves_manhattan() {
    let max_length = 5;
    let alg = Alg::<Cube3>::random_with_rng(max_length, &mut StdRng::seed_from_u64(69420));

    for i in 0..max_length {
        assert_solves_alg(Alg::new(alg.moves[0..i].to_vec()), heuristics::manhattan);
    }
}
