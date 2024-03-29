#![cfg(test)]

use crate::{
    cube3::Cube3,
    cube_n::{
        moves::{perms::SEXY_MOVE, rotation::AxisRotation, wide::Parsable, Amount},
        space::{Axis, Face},
        AxisMove,
    },
    generic::{Alg, Cube},
};

use super::*;

#[test]
fn default_orientation() {
    let orientation = Orientation::default();

    assert_eq!(orientation.r_face(), &Face::R);
    assert_eq!(orientation.u_face(), Face::U);
}

#[test]
fn rotates_orientation() {
    let mut orientation = Orientation::default();

    orientation.rotate(&AxisRotation::new(Axis::X, Amount::Double));

    assert_eq!(orientation.r_face(), &Face::R);
    assert_eq!(orientation.u_face(), Face::D);

    orientation.rotate(&AxisRotation::new(Axis::Y, Amount::Inverse));

    assert_eq!(orientation.r_face(), &Face::B);
    assert_eq!(orientation.u_face(), Face::D);
}

#[test]
fn moves_rotated_simple() {
    let mut cube = Cube3::SOLVED.orientable();

    cube.orientation
        .rotate(&AxisRotation::new(Axis::X, Amount::Double));

    cube.apply(&AxisMove::new(Face::U, Amount::Single));

    let expected = Cube3::SOLVED.moved(&AxisMove::new(Face::D, Amount::Single));

    assert_eq!(cube.base_cube, expected)
}

#[test]
fn moves_rotated() {
    let mut cube = Cube3::SOLVED.orientable();

    cube.orientation
        .rotate(&AxisRotation::new(Axis::X, Amount::Double));
    cube.orientation
        .rotate(&AxisRotation::new(Axis::Y, Amount::Inverse));

    cube.apply(&SEXY_MOVE);

    let expected = Cube3::SOLVED.moved(&Alg::<AxisMove>::parse("B D B' D'").unwrap());

    assert_eq!(cube.base_cube, expected)
}
