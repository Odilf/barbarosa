#![cfg(test)]

use rand::{rngs::StdRng, SeedableRng};

use crate::{
    cube3::{heuristics::mus, Cube3},
    generic::{Alg, Cube, Movable},
};

#[test]
#[ignore = "MUS takes too long to build"]
fn correct_cache() {
    let mut rng = StdRng::seed_from_u64(69420);

    for move_amount in 0..=20 {
        for _ in 0..100 {
            let alg = Alg::random_with_rng(move_amount, &mut rng);
            let cube = Cube3::SOLVED.moved(&alg);

            assert!(mus(&cube) <= alg.moves.len() as f32);
        }
    }
}
