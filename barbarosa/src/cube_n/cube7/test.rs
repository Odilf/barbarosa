#![cfg(test)]

use crate::{
    cube_n::{
        moves::{
            perms::{self, pll},
            Amount,
        },
        space::Face,
        AxisMove,
    },
    generic::{Cube, Movable},
};

use super::*;

#[test]
fn six_sexies() {
    let mut cube = Cube7::SOLVED;
    let alg = perms::SEXY_MOVE.clone().widen::<2>(0).unwrap();

    for _ in 0..6 {
        cube.apply(&alg);
    }

    assert!(cube.is_solved());
}

#[test]
fn two_t_perms_of_varying_depths() {
    let mut cube = Cube7::SOLVED;
    let alg = |i| pll::T.clone().widen::<2>(i).unwrap();

    for i in 0..=2 {
        cube.apply(&alg(i));
        assert!(!cube.is_solved());

        cube.apply(&alg(i));
        assert!(cube.is_solved());
    }
}

#[test]
fn varying_type_of_wide_move() {
    let axis_move = AxisMove::new(Face::R, Amount::Single);
    let w0 = WideAxisMove::<0>::new(Face::R, Amount::Single, 0).unwrap();
    let w1 = WideAxisMove::<1>::new(Face::R, Amount::Single, 0).unwrap();
    let w2 = WideAxisMove::<2>::new(Face::R, Amount::Single, 0).unwrap();

    Cube7::SOLVED.clone().apply(&axis_move);
    Cube7::SOLVED.clone().apply(&w0);
    Cube7::SOLVED.clone().apply(&w1);
    Cube7::SOLVED.clone().apply(&w2);
}
