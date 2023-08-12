#![cfg(test)]

use nalgebra::vector;

use super::*;

#[test]
fn closest_to_3() {
    let mut square = Square::<3>::new(Color::RED);
    square.data[0][0] = Color::ORANGE;

    assert_eq!(
        *square.at_mut(&vector![-1., -1., -1.], &Face::U).unwrap(),
        Color::ORANGE
    );

    assert!(square.at_mut(&vector![-1.5, -1., -1.], &Face::U).is_none());
    assert!(square
        .at_mut(&vector![-1.5 + 0.0001, -1., -1.], &Face::U)
        .is_some());
}

#[test]
fn closest_to_4() {
    let mut square = Square::<4>::new(Color::RED);
    square.data[0][0] = Color::ORANGE;
    square.data[2][2] = Color::BLUE;
    square.data[3][3] = Color::GREEN;

    assert_eq!(
        *square.at_mut(&vector![-1., -1., -1.], &Face::U).unwrap(),
        Color::ORANGE
    );

    assert_eq!(
        *square.at_mut(&vector![0.33, 1., 0.33], &Face::U).unwrap(),
        Color::BLUE
    );

    assert_eq!(
        *square.at_mut(&vector![0.5, 1., 0.5], &Face::U).unwrap(),
        Color::BLUE
    );

    assert_eq!(
        *square.at_mut(&vector![1.0, 1.0, 1.0], &Face::U).unwrap(),
        Color::GREEN
    );

    assert!(square
        .at_mut(&vector![-1.375, -1., -1.], &Face::U)
        .is_none());
    assert!(square
        .at_mut(&vector![-1.374, -1., -1.], &Face::U)
        .is_some());
}
