#![cfg(test)]

use std::fmt::Debug;

use rand::{rngs::StdRng, SeedableRng};

use crate::{
    cube3::Cube3,
    cube_n::{
        moves::{
            perms::{self, pll},
            QuarterAxisMove,
        },
        space::{Axis, Face},
        AxisMove, Edge, WideAxisMove,
    },
    generic::{alg::Alg, Cube, Movable, Parsable},
};

use super::{
    non_redundant::NonRedundantAxisMove,
    rotation::{AxisRotation, Rotatable},
    Amount,
};

fn assert_rotations<T: Rotatable + Clone + Eq + Debug>(
    initial: T,
    expectations: &[(AxisRotation, T)],
) {
    for (rotation, expected_face) in expectations {
        let rotated = initial.clone().rotated(&rotation);
        assert_eq!(&rotated, expected_face, "Rotation: {:?}", rotation);
    }
}

#[test]
fn rotates_u_face() {
    let face = Face::U;

    let expectations = [
        (AxisRotation::new(Axis::X, Amount::Single), Face::B),
        (AxisRotation::new(Axis::X, Amount::Double), Face::D),
        (AxisRotation::new(Axis::X, Amount::Inverse), Face::F),
    ];

    assert_rotations(face, &expectations);
}

#[test]
fn rotates_d_face() {
    let face = Face::D;

    let expectations = [
        (AxisRotation::new(Axis::X, Amount::Single), Face::F),
        (AxisRotation::new(Axis::X, Amount::Double), Face::U),
        (AxisRotation::new(Axis::X, Amount::Inverse), Face::B),
    ];

    assert_rotations(face, &expectations);
}

#[test]
fn rotates_edges() {
    let edge = Edge::try_from([Face::R, Face::F]).unwrap();

    let expectations = [
        (
            AxisRotation::new(Axis::X, Amount::Single),
            Edge::try_from([Face::R, Face::U]).unwrap().flipped(),
        ),
        (
            AxisRotation::new(Axis::X, Amount::Inverse),
            Edge::try_from([Face::R, Face::D]).unwrap().flipped(),
        ),
        (
            AxisRotation::new(Axis::X, Amount::Double),
            Edge::try_from([Face::R, Face::B]).unwrap(),
        ),
    ];

    assert_rotations(edge, &expectations);

    let edge = Edge::try_from([Face::U, Face::B]).unwrap();

    let expectations = [
        (
            AxisRotation::new(Axis::Y, Amount::Single),
            Edge::try_from([Face::U, Face::R]).unwrap(),
        ),
        (
            AxisRotation::new(Axis::Y, Amount::Inverse),
            Edge::try_from([Face::U, Face::L]).unwrap(),
        ),
        (
            AxisRotation::new(Axis::Y, Amount::Double),
            Edge::try_from([Face::U, Face::F]).unwrap(),
        ),
    ];

    assert_rotations(edge, &expectations);
}

#[test]
fn parses_moves() {
    assert_eq!(
        AxisMove::parse("R2").unwrap(),
        AxisMove::new(Face::R, Amount::Double)
    );
    assert_eq!(
        AxisMove::parse("L'").unwrap(),
        AxisMove::new(Face::L, Amount::Inverse)
    );
    assert_eq!(
        AxisMove::parse("D").unwrap(),
        AxisMove::new(Face::D, Amount::Single)
    );
}

#[test]
fn errors_on_invalid_algs() {
    let inputs = ["P", "µ", "R2'"];

    for input in inputs {
        assert!(AxisMove::parse(input).is_err());
    }
}

#[test]
fn errors_completely_when_parsing_invalid_move_in_alg() {
    let inputs = ["R2 P", "R2 µ", "R2 R2'"];

    for input in inputs {
        assert!(Alg::<AxisMove>::parse(input).is_err());
    }
}

#[test]
fn prints_moves() {
    let mov = WideAxisMove::<3>::new(Face::R, Amount::Double, 2).unwrap();

    assert_eq!(mov.to_string(), "3Rw2");
}

#[test]
fn six_sexy_moves_solves_cube() {
    let mut cube = Cube3::SOLVED;

    for _ in 0..6 {
        cube.apply(&perms::SEXY_MOVE);
    }

    assert_eq!(cube, Cube3::SOLVED);
}

#[test]
fn two_t_perms_solve_cube() {
    let mut cube = Cube3::SOLVED;

    for _ in 0..2 {
        cube.apply(&pll::T);
    }

    assert!(cube.is_solved());
}

#[test]
fn alg_and_inverse_solve_cube() {
    let alg = <Alg<AxisMove>>::random(30);

    let mut cube = Cube3::SOLVED;

    cube.apply(&alg);
    cube.apply(&alg.reversed());

    assert!(cube.is_solved());
}

#[test]
fn quarter_moves() {
    let alg: Alg<AxisMove> = Alg::random_with_rng(10, &mut StdRng::seed_from_u64(69420));

    let quarter_move_alg: Alg<QuarterAxisMove> = alg.clone().into();

    assert!(alg.moves.len() <= quarter_move_alg.moves.len());

    assert_eq!(
        Cube3::SOLVED.moved(&alg),
        Cube3::SOLVED.moved(&quarter_move_alg),
    );
}

#[test]
fn non_redundant_of_axis() {
    let moves = NonRedundantAxisMove::of_axis(Axis::X).collect::<Vec<_>>();

    let expected = [
        "R", "R'", "R2", "L", "L'", "L2", "R L", "R L'", "R L2", "R' L", "R' L'", "R' L2", "R2 L",
        "R2 L'", "R2 L2",
    ];

    assert_eq!(moves.len(), 15);

    for mov in moves {
        assert!(expected.contains(&mov.to_string().as_str()));
    }
}

#[test]
fn non_redundant_given_last_axis() {
    let moves = NonRedundantAxisMove::given_last_axis(&Axis::X).collect::<Vec<_>>();

    let not_expected = [
        "R", "R'", "R2", "L", "L'", "L2", "R L", "R L'", "R L2", "R' L", "R' L'", "R' L2", "R2 L",
        "R2 L'", "R2 L2",
    ];

    assert_eq!(moves.len(), 30);

    for mov in moves {
        assert!(!not_expected.contains(&mov.to_string().as_str()));
    }
}

#[test]
fn extended_moves() {
    todo!("parse extended moves")
}
