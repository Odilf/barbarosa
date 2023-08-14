#![cfg(test)]

use rand::{rngs::StdRng, SeedableRng};

use crate::{
    cube3::{
        cfop::cross::{count_cross_pieces, solve_cross, solve_cross_fast},
        Cube3,
    },
    cube_n::{space::Face, AxisMove},
    generic::{Alg, Cube, Movable},
};

#[test]
fn solves_arbitrary_crosses() {
    let mut rng = StdRng::seed_from_u64(69420);

    for i in 0..1 {
        let scramble = Alg::<AxisMove>::random_with_rng(20, &mut rng);
        let cube = Cube3::SOLVED.moved(&scramble);
        println!("Solving with scramble {scramble} \n{cube}");

        let face = Face::U;
        let (cross_solution, solved) = match i {
            0 => solve_cross_fast(&cube, &face),
            1 => solve_cross(&cube, &face),
            _ => unreachable!(),
        }
        .unwrap();

        println!("Found solution {cross_solution}");
        assert_eq!(count_cross_pieces(&solved, &face), 4);
    }
}
