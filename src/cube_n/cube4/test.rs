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
    },
    generic::{utils, Alg, Cube, Movable},
};

use super::*;

fn expect_wing(cube: &Cube4, target: ([Face; 2], Direction), expected: ([Face; 2], Direction)) {
    let target = Wing::from_faces(target.0, target.1).unwrap();
    let expected = Wing::from_faces(expected.0, expected.1).unwrap();
    let found = utils::item_at(&target, &cube.wings, &Cube4::solved().wings).unwrap();
    let position_of_expected =
        utils::position_of_item(&expected, &cube.wings, &Cube4::solved().wings);

    assert_eq!(
        found, &expected,
        "Expected {:#?} at {:#?}, found {:#?}. Expected is actually at {:#?}",
        expected, target, found, position_of_expected
    );
}

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
fn four_wide_us() {
    let mut cube = Cube4::new_solved();
    let mov = WideAxisMove::<1>::new(Face::U, Amount::Single, 1).unwrap();

    for i in 0..4 {
        println!("{}", i);
        cube.apply(&mov);
        dbg!(&cube.wings);
        cube.assert_consistent();
    }

    assert!(cube.is_solved());
}

#[test]
fn four_wide_fs() {
    let mut cube = Cube4::new_solved();
    let mov = WideAxisMove::<1>::new(Face::F, Amount::Single, 1).unwrap();

    for i in 0..4 {
        cube.apply(&mov);
        cube.assert_consistent();

        match i {
            0 => expect_wing(&cube, ([R, F], Positive), ([U, F], Negative)),
            _ => (),
        }
    }

    assert!(cube.is_solved());
}

#[test]
fn four_wide_rs() {
    let mut cube = Cube4::new_solved();
    let mov = WideAxisMove::<1>::new(Face::R, Amount::Single, 1).unwrap();

    for _ in 0..4 {
        cube.apply(&mov);
        cube.assert_consistent();
    }

    assert!(cube.is_solved());
}

#[test]
fn four_of_every_single_move() {
    for m in AxisMove::all() {
        let m_wide = m.clone().widen::<1>(1).unwrap();

        let mut cube = Cube4::new_solved();

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
    let alg = Alg::<WideAxisMove<1>>::random_with_rng(30, &mut StdRng::seed_from_u64(69420));

    let mut cube = Cube4::new_solved();

    cube.apply(&alg);
    cube.assert_consistent();

    cube.apply(&alg.reversed());
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
        cube.assert_consistent();
    }

    assert_eq!(cube.corners, Cube4::solved().corners);

    assert_eq!(
        cube.centers,
        Cube4::solved().centers,
        "Centers are not solved"
    );

    assert_eq!(cube.wings, Cube4::solved().wings, "Wings are not solved");

    assert!(cube.is_solved());
}

#[test]
fn two_wide_ts() {
    let mut cube = Cube4::new_solved();
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

    assert_eq!(cube.wings, Cube4::solved().wings);
    assert!(cube.is_solved());
}

#[test]
fn two_wide_js() {
    let mut cube = Cube4::new_solved();
    let wide_j: Alg<Cube4> = pll::J
        .clone()
        .moves
        .into_iter()
        .map(|mov| mov.widen(1).unwrap())
        .collect();

    println!("Alg: {wide_j}");

    for i in 0..2 {
        println!("Iteration {i}");

        for m in &wide_j.moves {
            cube.apply(m);
            cube.assert_consistent();
        }
    }

    cube.assert_consistent();

    assert_eq!(Cube4::solved().corners, cube.corners);
    assert_eq!(Cube4::solved().centers, cube.centers);

    assert_eq!(
        Cube4::solved().wings,
        cube.wings,
        "Solved: {:#?} got: {:#?}",
        Cube4::solved().wings,
        cube.wings
    );

    assert!(cube.is_solved());
}

#[test]
fn random_amount_of_wide_u_perms() {
    let mut cube = Cube4::new_solved();
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
    let mut cube = Cube4::new_solved();

    for _ in 0..2 {
        cube.apply(&pll::T);
    }

    assert!(cube.is_solved());
}
