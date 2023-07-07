#![cfg(test)]

use crate::{
    cube_n::moves::perms::{self, pll},
    generic::{Cube, Movable},
};

use super::*;

#[test]
fn six_sexies() {
    let mut cube = Cube7::new_solved();
    let alg = perms::SEXY_MOVE.clone().widen::<2>(0).unwrap();

    for _ in 0..6 {
        cube.apply(&alg);
    }

    assert!(cube.is_solved());
}

#[test]
fn two_t_perms_of_variyng_depths() {
    let mut cube = Cube7::new_solved();
    let alg = |i| pll::T.clone().widen::<2>(i).unwrap();

    for i in 0..=2 {
        cube.apply(&alg(i));
        assert!(!cube.is_solved());

        cube.apply(&alg(i));
        assert!(cube.is_solved());
    }
}
