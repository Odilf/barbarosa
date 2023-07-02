#![cfg(test)]

use crate::{
    cube_n::{
        moves::{perms, Amount},
        space::{Axis, Direction, Face},
        AxisMove,
    },
    generic::{Cube, Movable, Parsable},
};

use super::*;

#[test]
fn just_solved() {
    assert!(Cube4::solved().is_solved());
}

#[test]
fn apply_move() {
    let mut cube = Cube4::new_solved();
    let mov = WideAxisMove::<1>::new(Face::R, Amount::Single, 1).unwrap();

    cube.apply(&mov);
    assert!(!cube.is_solved());
}

#[test]
fn six_sexy_moves() {
    let mut cube = Cube4::new_solved();

    for _ in 0..6 {
        cube.apply(&perms::SEXY_MOVE);
    }

    assert!(cube.is_solved());
}

#[test]
fn six_wide_sexies() {
    let mut cube = Cube4::new_solved();

    let wide_sexy: Alg<WideAxisMove<1>> = perms::SEXY_MOVE
        .moves
        .iter()
        .map(|mov| mov.clone().widen(1).unwrap())
        .collect();

    for _ in 0..6 {
        cube.apply(&wide_sexy);
    }

    assert!(cube.is_solved());
}
