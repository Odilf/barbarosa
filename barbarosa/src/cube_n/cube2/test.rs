#![cfg(test)]

use crate::{
    cube_n::moves::perms::{self, pll},
    generic::{Cube, Movable},
};

use super::Cube2;

#[test]
fn trying_to_permute_edges_doesnt_unsolve() {
    let cube = Cube2::SOLVED.moved(&pll::U);
    assert!(cube.is_solved());
}

#[test]
fn two_ts_solves() {
    let cube = Cube2::SOLVED.moved(&pll::T).moved(&pll::T);

    assert!(cube.is_solved());
}

#[test]
fn six_sexies() {
    let mut cube = Cube2::SOLVED;

    for _ in 0..6 {
        cube.apply(&perms::SEXY_MOVE);
    }

    assert!(cube.is_solved());
}
