#![cfg(test)]

use crate::cube_n::{
    moves::{
        rotation::{AxisRotation, Rotatable},
        Amount,
    },
    space::{Axis, Direction, Face},
    Edge,
};

use super::Wing;

fn wing_at(a: Face, b: Face, oriented: bool) -> Wing {
    let (normal_axis, slice_position) = Edge::position_from_faces([a, b]).unwrap();

    let corresponding_edge = Edge {
        slice_position,
        normal_axis,
        oriented,
    };

    Wing { corresponding_edge }
}

#[test]
fn has_correct_direction() {
    let wing1 = wing_at(Face::U, Face::R, true);
    let wing2 = wing_at(Face::F, Face::R, true);
    let wing3 = wing_at(Face::F, Face::U, true);

    assert_eq!(wing1.direction_along_normal(), Direction::Positive);
    assert_eq!(wing2.direction_along_normal(), Direction::Positive);
    assert_eq!(wing3.direction_along_normal(), Direction::Negative);
}

#[test]
fn turning_on_u_face() {
    let mut wing = wing_at(Face::U, Face::R, true);

    wing.rotate(&AxisRotation::new(Axis::Y, Amount::Single));

    assert_eq!(wing.normal_axis(), Axis::X);
    assert_eq!(wing.direction_along_normal(), Direction::Negative);

    wing.rotate(&AxisRotation::new(Axis::Y, Amount::Single));

    assert_eq!(wing.normal_axis(), Axis::Z);
    assert_eq!(wing.direction_along_normal(), Direction::Negative);

    wing.rotate(&AxisRotation::new(Axis::Y, Amount::Single));

    assert_eq!(wing.normal_axis(), Axis::X);
    assert_eq!(wing.direction_along_normal(), Direction::Positive);
}

#[test]
fn turning_on_r_face() {
    let mut wing = wing_at(Face::U, Face::R, true);

    wing.rotate(&AxisRotation::new(Axis::X, Amount::Single));

    assert_eq!(wing.normal_axis(), Axis::Y);
    assert_eq!(wing.direction_along_normal(), Direction::Positive);

    wing.rotate(&AxisRotation::new(Axis::X, Amount::Single));

    assert_eq!(wing.normal_axis(), Axis::Z);
    assert_eq!(wing.direction_along_normal(), Direction::Negative);

    wing.rotate(&AxisRotation::new(Axis::X, Amount::Single));

    assert_eq!(wing.normal_axis(), Axis::Y);
    assert_eq!(wing.direction_along_normal(), Direction::Negative);
}

#[test]
fn turning_on_f_face_inside_wings() {
    let mut wing = wing_at(Face::U, Face::R, true);

    wing.rotate(&AxisRotation::new(Axis::Z, Amount::Single));

    assert_eq!(wing.normal_axis(), Axis::Z);
    assert_eq!(wing.direction_along_normal(), Direction::Positive);

    wing.rotate(&AxisRotation::new(Axis::Z, Amount::Single));

    assert_eq!(wing.normal_axis(), Axis::Z);
    assert_eq!(wing.direction_along_normal(), Direction::Positive);

    wing.rotate(&AxisRotation::new(Axis::Z, Amount::Single));

    assert_eq!(wing.normal_axis(), Axis::Z);
    assert_eq!(wing.direction_along_normal(), Direction::Positive);

    wing.rotate(&AxisRotation::new(Axis::Z, Amount::Single));

    assert_eq!(wing, wing_at(Face::U, Face::R, true))
}
