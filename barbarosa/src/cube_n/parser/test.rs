#![cfg(test)]

use pest::Parser;

use crate::cube_n::{
    moves::{rotation::AxisRotation, wide::Parsable, Amount, ExtendedAxisMove},
    parser::{parse_slice_move, CubeNParser, Rule},
    space::Face,
    AxisMove, WideAxisMove,
};

#[test]
fn amount() {
    assert_eq!(Amount::Single, Amount::parse("").unwrap());
    assert_eq!(Amount::Double, Amount::parse("2").unwrap());
    assert_eq!(Amount::Inverse, Amount::parse("'").unwrap());

    assert!(Amount::parse("3").is_err());
    assert!(Amount::parse(" ").is_err());
    assert!(Amount::parse("2 ").is_err());
    assert!(Amount::parse(" 2").is_err());
}

#[test]
fn face() {
    assert_eq!(Face::U, Face::parse("U").unwrap());
    assert_eq!(Face::D, Face::parse("D").unwrap());
    assert_eq!(Face::L, Face::parse("L").unwrap());
    assert_eq!(Face::R, Face::parse("R").unwrap());
    assert_eq!(Face::F, Face::parse("F").unwrap());
    assert_eq!(Face::B, Face::parse("B").unwrap());

    assert!(Face::parse("A").is_err());
    assert!(Face::parse("u").is_err());
    assert!(Face::parse(" ").is_err());
    assert!(Face::parse("U caca").is_err());
    assert!(Face::parse(" U").is_err());
}

#[test]
fn moves() {
    let assert_move = |s: &str, face: Face, amount: Amount| {
        assert_eq!(AxisMove::new(face, amount), AxisMove::parse(s).unwrap())
    };

    assert_move("R", Face::R, Amount::Single);
    assert_move("R2", Face::R, Amount::Double);
    assert_move("R'", Face::R, Amount::Inverse);
    assert_move("B'", Face::B, Amount::Inverse);

    assert!(AxisMove::parse("R3").is_err());
    assert!(AxisMove::parse("R2'").is_err());
}

#[test]
fn wide_moves() {
    let assert_move = |s: &str, face: Face, amount: Amount, depth: u32| {
        assert_eq!(
            WideAxisMove::<2>::new(face, amount, depth).unwrap(),
            WideAxisMove::<2>::parse(s).unwrap()
        )
    };

    assert_move("R", Face::R, Amount::Single, 0);
    assert_move("Rw", Face::R, Amount::Single, 1);
    assert_move("Rw2", Face::R, Amount::Double, 1);
    assert_move("Rw'", Face::R, Amount::Inverse, 1);

    assert_move("r'", Face::R, Amount::Inverse, 1);

    assert_move("2r'", Face::R, Amount::Inverse, 2);
    assert_move("2Rw'", Face::R, Amount::Inverse, 2);

    assert!(WideAxisMove::<2>::parse("Rw3").is_err());
    assert!(WideAxisMove::<2>::parse("rw").is_err());
    // TODO: Proper error handling
    // assert!(WideAxisMove::<2>::parse("3Rw").is_err());
}

#[test]
fn extended_moves() {
    macro_rules! assert_move {
        ($testing:literal, "slice") => {
            assert_eq!(
                ExtendedAxisMove::parse($testing).unwrap(),
                parse_slice_move(
                    CubeNParser::parse(Rule::slice_move, "E")
                        .unwrap()
                        .next()
                        .unwrap()
                )
                .into()
            );
        };
        ($testing:literal, $type:ty) => {
            assert_eq!(
                ExtendedAxisMove::parse($testing).unwrap(),
                <$type>::parse($testing).unwrap().into()
            );
        };
    }

    assert_move!("R", AxisMove);
    assert_move!("R2", AxisMove);
    assert_move!("Rw", WideAxisMove<2>);

    assert_move!("x", AxisRotation);
    assert_move!("x2", AxisRotation);
    assert_move!("z'", AxisRotation);
    assert_move!("y", AxisRotation);

    assert_move!("E", "slice");
    assert_move!("E2", "slice");
    assert_move!("e'", "slice");
    assert_move!("S", "slice");
    assert_move!("M'", "slice");
}
