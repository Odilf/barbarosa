#![cfg(test)]

use crate::{
    cube_n::{moves::perms::pll, space::Face},
    generic::{Alg, Cube, Movable},
};

use super::Cube5;

#[test]
fn two_t_pems_solve_it() {
    let mut cube = Cube5::SOLVED;

    let wide_t = pll::T.clone().widen::<1>(1).unwrap();

    cube.apply(&wide_t);

    assert!(!cube.is_solved());

    cube.apply(&wide_t);

    assert!(cube.is_solved());
}

#[test]
fn half_wide_t_perm() {
    let mut cube = Cube5::SOLVED;

    let half_wide_t: Alg<Cube5> = pll::T
        .clone()
        .moves
        .into_iter()
        .map(|mov| {
            let depth = if mov.face == Face::R { 1 } else { 0 };
            mov.widen(depth).unwrap()
        })
        .collect();

    cube.apply(&half_wide_t);

    assert!(!cube.is_solved());

    cube.apply(&half_wide_t);

    assert!(cube.is_solved());
}
