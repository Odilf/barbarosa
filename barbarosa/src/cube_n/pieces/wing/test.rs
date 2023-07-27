#![cfg(test)]

use nalgebra::vector;

use crate::cube_n::{
    moves::{
        rotation::{AxisRotation, Rotatable},
        Amount::*,
    },
    space::{Axis::*, Direction::*, Face},
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

    assert_eq!(wing1.normal_direction(), Positive);
    assert_eq!(wing2.normal_direction(), Positive);
    assert_eq!(wing3.normal_direction(), Negative);
}

#[test]
fn turning_on_u_face() {
    let mut wing = Wing::try_from_faces([Face::R, Face::U], Positive).unwrap();
    let mut wing_side = Wing::try_from_faces([Face::R, Face::F], Positive).unwrap();

    assert_eq!(wing_side.normal_axis(), Y);
    assert_eq!(wing_side.normal_direction(), Positive);
    assert_eq!(wing_side.slice_position(), vector![Positive, Positive]);

    wing.rotate(&AxisRotation::new(Y, Single));
    wing_side.rotate(&AxisRotation::new(Y, Single));

    assert_eq!(wing.normal_axis(), X);
    assert_eq!(wing.normal_direction(), Negative);
    assert_eq!(wing.slice_position(), vector![Positive, Positive]);

    assert_eq!(wing_side.normal_axis(), Y);
    assert_eq!(wing_side.normal_direction(), Positive);
    assert_eq!(wing_side.slice_position(), vector![Positive, Negative]);

    wing.rotate(&AxisRotation::new(Y, Single));

    assert_eq!(wing.normal_axis(), Z);
    assert_eq!(wing.normal_direction(), Negative);
    assert_eq!(wing.slice_position(), vector![Negative, Positive]);

    wing.rotate(&AxisRotation::new(Y, Single));

    assert_eq!(wing.normal_axis(), X);
    assert_eq!(wing.normal_direction(), Positive);
    assert_eq!(wing.slice_position(), vector![Positive, Negative]);
}

#[test]
fn turning_on_r_face() {
    let mut wing = wing_at(Face::U, Face::R, true);
    let mut wing_inside = wing_at(Face::U, Face::F, true);

    assert_eq!(wing_inside.slice_position(), vector![Positive, Positive]);

    wing.rotate(&AxisRotation::new(X, Single));
    wing_inside.rotate(&AxisRotation::new(X, Single));

    assert_eq!(wing.normal_axis(), Y);
    assert_eq!(wing.normal_direction(), Positive);

    assert_eq!(wing_inside.slice_position(), vector![Positive, Negative]);

    wing.rotate(&AxisRotation::new(X, Single));
    wing_inside.rotate(&AxisRotation::new(X, Single));

    assert_eq!(wing.normal_axis(), Z);
    assert_eq!(wing.normal_direction(), Negative);

    assert_eq!(wing_inside.slice_position(), vector![Negative, Negative]);

    wing.rotate(&AxisRotation::new(X, Single));
    wing_inside.rotate(&AxisRotation::new(X, Single));

    assert_eq!(wing.normal_axis(), Y);
    assert_eq!(wing.normal_direction(), Negative);

    assert_eq!(wing_inside.slice_position(), vector![Negative, Positive]);
}

#[test]
fn turning_on_f_face_inside_wings() {
    let mut wing = Wing::try_from_faces([Face::U, Face::R], Positive).unwrap();

    assert_eq!(wing.slice_position(), vector![Positive, Positive]);

    wing.rotate(&AxisRotation::new(Z, Single));

    assert_eq!(wing.normal_axis(), Z);
    assert_eq!(wing.normal_direction(), Positive);
    assert_eq!(wing.slice_position(), vector![Positive, Negative]);

    wing.rotate(&AxisRotation::new(Z, Single));

    assert_eq!(wing.normal_axis(), Z);
    assert_eq!(wing.normal_direction(), Positive);
    assert_eq!(wing.slice_position(), vector![Negative, Negative]);

    wing.rotate(&AxisRotation::new(Z, Single));

    assert_eq!(wing.normal_axis(), Z);
    assert_eq!(wing.normal_direction(), Positive);
    assert_eq!(wing.slice_position(), vector![Negative, Positive]);

    wing.rotate(&AxisRotation::new(Z, Single));

    assert_eq!(wing, wing_at(Face::U, Face::R, true))
}
