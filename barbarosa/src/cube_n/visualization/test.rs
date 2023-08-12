#![cfg(test)]

use crate::{
    cube3::Cube3,
    cube_n::{moves::wide::Parsable, AxisMove},
    generic::{Cube, Movable},
};

use super::unfolded::Unfolded;

#[test]
fn test_vis_solved() {
    let cube = Cube3::SOLVED;
    let unfolded: Unfolded<3> = cube.into();

    println!("{}", unfolded);
}

#[test]
fn test_vis_r() {
    let cube = Cube3::SOLVED.moved(&AxisMove::parse("F").unwrap());
    let unfolded: Unfolded<3> = cube.into();

    println!("{}", unfolded);
}
