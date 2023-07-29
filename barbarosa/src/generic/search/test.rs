#![cfg(test)]

use rand::{rngs::StdRng, SeedableRng};

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

        println!("Solution: {solution}, reversed alg: {}", alg.reversed());
        assert_eq!(solution.moves.len(), alg.reversed().moves.len());
    };
}

#[test]
#[ignore = "MUS takes too long to build"]
fn solves_correctly_and_optimally() {
    assert_solves_ida!(Cube3, heuristics::mus, "R U R' U'");
    assert_solves_ida!(Cube3, heuristics::mus, "R U R F D2 L");
    assert_solves_ida!(Cube3, heuristics::mus, "R' L2 B' U2 D");
    // assert_solves_ida!(Cube3, heuristics::mus, "R' L2 B U2 D2 F B U2");
}

#[test]
#[ignore = "MUS takes too long to build"]
fn solves_lengths() {
    let mut rng = StdRng::seed_from_u64(69420);

    for move_amount in 13..=20 {
        let alg = Alg::random_with_rng(move_amount, &mut rng);
        let cube = Cube3::new_solved().moved(&alg);

        let solution = cube
            .solve_with_heuristic(heuristics::mus)
            .expect("Cube should be solvable");

        assert!(solution.moves.len() <= alg.moves.len());

        println!("Solved length {move_amount}");
    }
}

// #[test]
// fn test_scramble() {
//     let cube: Cube3 = rand::random();

//     cube.solve_with_heuristic(heuristics::mus).unwrap();

//     assert!(cube.is_solved());
// }
