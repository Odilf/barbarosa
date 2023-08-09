#![cfg(test)]

use pretty_assertions::assert_eq;
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    cube_n::{
        moves::{
            perms::{self, pll},
            Amount,
        },
        space::{
            faces::*,
            Direction::{self, *},
            Face,
        },
        AxisMove, Wing,
    },
    generic::{Alg, Cube, Movable},
};

use super::*;

fn expect_wing(cube: &Cube4, target: ([Face; 2], Direction), expected: ([Face; 2], Direction)) {
    let target = Wing::try_from_faces(target.0, target.1).unwrap();
    let expected = Wing::try_from_faces(expected.0, expected.1).unwrap();

    let found = cube.wings.original_position_of_piece_at(&target);
    let position_of_expected = cube.wings.original_position_of_piece_at(&expected);

    assert_eq!(
        found, expected,
        "Expected {:#?} at {:#?}, found {:#?}. Expected is actually at {:#?}",
        expected, target, found, position_of_expected
    );
}

#[test]
fn just_solved() {
    assert!(Cube4::SOLVED.is_solved());
}

#[test]
fn apply_move() {
    let mut cube = Cube4::SOLVED;
    let mov = WideAxisMove::<1>::new(Face::R, Amount::Single, 1).unwrap();

    cube.apply(&mov);
    assert!(!cube.is_solved());
}

#[test]
fn six_sexy_moves() {
    let mut cube = Cube4::SOLVED;

    for _ in 0..6 {
        cube.apply(&perms::SEXY_MOVE);
    }

    assert!(cube.is_solved());
}

#[test]
fn four_wide_us() {
    let mut cube = Cube4::SOLVED;
    let mov = WideAxisMove::<1>::new(Face::U, Amount::Single, 1).unwrap();

    for _ in 0..4 {
        cube.apply(&mov);
    }

    assert!(cube.is_solved());
}

#[test]
fn four_wide_fs() {
    let mut cube = Cube4::SOLVED;
    let mov = WideAxisMove::<1>::new(Face::F, Amount::Single, 1).unwrap();

    for i in 0..4 {
        cube.apply(&mov);

        match i {
            0 => expect_wing(&cube, ([R, F], Positive), ([U, F], Negative)),
            _ => (),
        }
    }

    assert!(cube.is_solved());
}

#[test]
fn four_wide_rs() {
    let mut cube = Cube4::SOLVED;
    let mov = WideAxisMove::<1>::new(Face::R, Amount::Single, 1).unwrap();

    for _ in 0..4 {
        cube.apply(&mov);
    }

    assert!(cube.is_solved());
}

#[test]
fn four_of_every_single_move() {
    for m in AxisMove::all() {
        let m_wide = m.clone().widen::<1>(1).unwrap();

        let mut cube = Cube4::SOLVED;

        for _ in 0..4 {
            cube.apply(&m_wide);
        }

        assert!(cube.is_solved());

        for _ in 0..4 {
            cube.apply(&m);
        }

        assert!(cube.is_solved());
    }
}

#[test]

fn solve_unsolve_journey() {
    let alg =
        Alg::<WideAxisMove<1>>::random_unnormalized_with_rng(30, &mut StdRng::seed_from_u64(69420));

    let mut cube = Cube4::SOLVED;

    cube.apply(&alg);

    cube.apply(&alg.reversed());
    assert!(cube.is_solved());
}

#[test]
fn six_wide_sexies() {
    let mut cube = Cube4::SOLVED;

    let wide_sexy: Alg<WideAxisMove<1>> = perms::SEXY_MOVE
        .moves
        .iter()
        .map(|mov| mov.clone().widen(1).unwrap())
        .collect();

    for _ in 0..6 {
        cube.apply(&wide_sexy);
    }

    assert_eq!(cube.corners, Cube4::SOLVED.corners);

    assert_eq!(
        cube.centers,
        Cube4::SOLVED.centers,
        "Centers are not solved"
    );

    assert_eq!(cube.wings, Cube4::SOLVED.wings, "Wings are not solved");

    assert!(cube.is_solved());
}

#[test]
fn two_wide_ts() {
    let mut cube = Cube4::SOLVED;
    let wide_t: Alg<Cube4> = pll::T
        .clone()
        .moves
        .into_iter()
        .map(|mov| mov.widen(1).unwrap())
        .collect();

    println!("Excecuting {}", wide_t);

    for (i, m) in wide_t.moves.iter().enumerate() {
        println!("Appling move {i} ({m})");
        cube.apply(m);

        match i {
            12 => expect_wing(&cube, ([R, F], Negative), ([R, U], Negative)),
            13 => expect_wing(&cube, ([U, F], Positive), ([R, U], Negative)),
            _ => (),
        }
    }

    cube.apply(&wide_t);

    assert_eq!(cube.wings, Cube4::SOLVED.wings);
    assert!(cube.is_solved());
}

#[test]
fn two_wide_js() {
    let mut cube = Cube4::SOLVED;
    let wide_j: Alg<Cube4> = pll::J
        .clone()
        .moves
        .into_iter()
        .map(|mov| mov.widen(1).unwrap())
        .collect();

    println!("Alg: {wide_j}");

    for _ in 0..2 {
        cube.apply(&wide_j);
    }

    assert_eq!(Cube4::SOLVED.corners, cube.corners);
    assert_eq!(Cube4::SOLVED.centers, cube.centers);

    assert_eq!(
        Cube4::SOLVED.wings,
        cube.wings,
        "Solved: {:#?} got: {:#?}",
        Cube4::SOLVED.wings,
        cube.wings
    );

    assert!(cube.is_solved());
}

#[test]
fn random_amount_of_wide_u_perms() {
    let mut cube = Cube4::SOLVED;
    let wide_u: Alg<Cube4> = pll::U
        .clone()
        .moves
        .into_iter()
        .map(|mov| mov.widen(1).unwrap())
        .collect();

    for _ in 0..5 {
        cube.apply(&wide_u);
    }

    assert!(cube.is_solved());
}

#[test]
fn regular_ass_t_perm_lol() {
    let mut cube = Cube4::SOLVED;

    for _ in 0..2 {
        cube.apply(&pll::T);
    }

    assert!(cube.is_solved());
}

#[test]
fn moves_by_all_widenesses() {
    let normal = AxisMove::new(Face::R, Amount::Single);
    let w0 = WideAxisMove::<0>::new(Face::R, Amount::Single, 0).unwrap();
    let w1 = WideAxisMove::<1>::new(Face::R, Amount::Single, 0).unwrap();

    Cube4::SOLVED.clone().apply(&normal);
    Cube4::SOLVED.clone().apply(&w0);
    Cube4::SOLVED.clone().apply(&w1);
}
