#![cfg(test)]

use pretty_assertions::assert_eq;
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    cube_n::{
        moves::{perms, Amount},
        space::Face,
    },
    generic::{Cube, Movable},
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

    for _ in 0..4 {
        cube.apply(&mov);
        cube.assert_consistent();
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
    let wide_t: <Cube4 as Cube>::Alg = perms::T
        .clone()
        .moves
        .into_iter()
        .map(|mov| mov.widen(1).unwrap())
        .collect();

    println!("Alg: {wide_t}");

    cube.apply(&wide_t.moves[0]);
    cube.apply(&wide_t.moves[1]);
    cube.apply(&wide_t.moves[2]);
    cube.apply(&wide_t.moves[3]);

    cube.apply(&wide_t.moves[4]);

    assert_eq!(
        cube.wing_at_faces([Face::L, Face::D], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::L, Face::D], Direction::Positive).unwrap(),
    );

    cube.apply(&wide_t.moves[5]);

    println!("{}", &wide_t.moves[5]);

    assert_eq!(
        cube.wing_at_faces([Face::L, Face::U], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::L, Face::D], Direction::Positive).unwrap(),
        "Cube: {:#?}, LD is at {:#?}", cube.wings, cube.position_of_wing(&Wing::from_faces([Face::L, Face::D], Direction::Positive).unwrap()),
    );

    cube.apply(&wide_t.moves[6]);

    assert_eq!(
        cube.wing_at_faces([Face::U, Face::F], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::R, Face::F], Direction::Positive).unwrap(),
    );

    assert_eq!(
        cube.wing_at_faces([Face::L, Face::U], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::L, Face::D], Direction::Positive).unwrap(),
    );

    cube.apply(&wide_t.moves[7]);

    assert_eq!(
        cube.wing_at_faces([Face::F, Face::U], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::L, Face::D], Direction::Positive).unwrap(),
        "Cube: {:#?}, LD is at {:#?}", cube.wings, cube.position_of_wing(&Wing::from_faces([Face::L, Face::D], Direction::Positive).unwrap()),
    );

    cube.apply(&wide_t.moves[8]);

    assert_eq!(
        cube.wing_at_faces([Face::U, Face::F], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::U, Face::F], Direction::Positive).unwrap(),
    );

    assert_eq!(
        cube.wing_at_faces([Face::F, Face::D], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::L, Face::D], Direction::Positive).unwrap(),
    );

    cube.apply(&wide_t.moves[9]);

    assert_eq!(
        cube.wing_at_faces([Face::U, Face::F], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::R, Face::U], Direction::Negative).unwrap(),
    );

    assert_eq!(
        cube.wing_at_faces([Face::F, Face::D], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::L, Face::D], Direction::Positive).unwrap(),
    );

    cube.apply(&wide_t.moves[10]);

    assert_eq!(
        cube.wing_at_faces([Face::U, Face::F], Direction::Positive).unwrap(),
        &Wing::from_faces([Face::L, Face::D], Direction::Positive).unwrap(),
    );

    cube.apply(&wide_t.moves[11]);
    cube.apply(&wide_t.moves[12]);
    cube.apply(&wide_t.moves[13]);

    cube.apply(&wide_t);

    cube.assert_consistent();

    assert_eq!(
        Cube4::solved().wings,
        cube.wings,
        "Solved: {:#?} got: {:#?}",
        Cube4::solved().wings,
        cube.wings
    );
    assert_eq!(Cube4::solved().corners, cube.corners);
    assert_eq!(Cube4::solved().centers, cube.centers);

    assert!(cube.is_solved());
}

#[test]
fn two_wide_js() {
    let mut cube = Cube4::new_solved();
    let wide_j: <Cube4 as Cube>::Alg = perms::J
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
    let wide_u: <Cube4 as Cube>::Alg = perms::U
        .clone()
        .moves
        .into_iter()
        .map(|mov| mov.widen(1).unwrap())
        .collect();

    for _ in 0..3 {
        cube.apply(&wide_u);
    }

    assert!(cube.is_solved());
}
