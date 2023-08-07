#![cfg(test)]

use pretty_assertions::assert_eq;

use crate::{
    cube_n::{
        moves::perms::{self, pll},
        space::{Axis, Direction},
        AxisMove,
    },
    generic::{Cube, Movable},
};

use super::*;

#[test]
fn six_sexies() {
    let mut cube = Cube6::new_solved();
    let alg = perms::SEXY_MOVE.clone().widen::<2>(0).unwrap();

    for _ in 0..6 {
        cube.apply(&alg);
    }

    assert!(cube.is_solved());
}

#[test]
fn two_t_perms_of_varying_depths() {
    let mut cube = Cube6::new_solved();
    let alg = |i| pll::T.clone().widen::<2>(i).unwrap();

    for i in 0..=2 {
        let alg = alg(i);

        cube.apply(&alg);
        assert!(!cube.is_solved(), "{alg}");

        cube.apply(&alg);
        assert!(cube.is_solved(), "{alg}");
    }
}

#[test]
fn four_of_each() {
    for mov in &AxisMove::all() {
        for depth in 0..=2 {
            let mov = mov.clone().widen::<2>(depth).unwrap();
            let mut cube = Cube6::new_solved();

            for _ in 0..4 {
                cube.apply(&mov);
            }

            assert!(cube.is_solved());
        }
    }
}

#[test]
fn center_wings_normal_directions() {
    let ru = &Cube6::solved().center_wings.pieces()[0];

    assert_eq!(ru.normal_axis(), Axis::Z);
    assert_eq!(ru.normal_direction(), Direction::Positive);

    let uf = &Cube6::solved().center_wings.pieces()[1];

    assert_eq!(uf.normal_axis(), Axis::X);
    assert_eq!(uf.normal_direction(), Direction::Negative);
}
