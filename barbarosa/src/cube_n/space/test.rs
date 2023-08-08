#![cfg(test)]

use super::*;

#[test]
fn text_next_around() {
    assert_eq!(Face::F.next_around(&Axis::X), Face::U);
    assert_eq!(Face::U.next_around(&Axis::X), Face::B);
    assert_eq!(Face::B.next_around(&Axis::X), Face::D);
    assert_eq!(Face::D.next_around(&Axis::X), Face::F);
}

#[test]
fn test_cross() {
    assert_eq!(Face::R.cross(&Face::U), Some(Face::F));
    assert_eq!(Face::U.cross(&Face::R), Some(Face::B));
    assert_eq!(Face::F.cross(&Face::U), Some(Face::L));
}
