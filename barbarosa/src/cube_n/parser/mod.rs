mod test;

use pest::iterators::Pair;
use pest_derive::Parser;

use crate::generic::parse::{FromPest, IntoParseErr, Parsable, ParseError};

type Result<T> = std::result::Result<T, ParseError<Rule>>;

use super::{
    moves::{rotation::AxisRotation, wide::WideMoveCreationError, Amount, ExtendedAxisMove},
    space::{Axis, Direction, Face},
    AxisMove, WideAxisMove,
};

#[derive(Parser)]
#[grammar = "grammar/cube_n.pest"]
pub struct CubeNParser;

macro_rules! impl_with_current_rule {
    ($implementor:ty; $rule:expr; |$arg:ident| $body:expr) => {
        impl FromPest for $implementor {
            type Rule = Rule;
            type Parser = CubeNParser;

            fn rule() -> Self::Rule {
                $rule
            }

            fn from_pest($arg: Pair<Self::Rule>) -> Result<Self> {
                $body
            }
        }
    };
}

impl_with_current_rule! {
    Amount;
    Rule::amount;

    |pair| match pair.as_str() {
        "" => Ok(Amount::Single),
        "2" => Ok(Amount::Double),
        "'" => Ok(Amount::Inverse),
        other => Err(ParseError::Unreachable(other.to_string())),
    }
}

impl_with_current_rule! {
    Face;
    Rule::face;

    |pair| match pair.as_str() {
        "U" => Ok(Face::U),
        "D" => Ok(Face::D),
        "L" => Ok(Face::L),
        "R" => Ok(Face::R),
        "F" => Ok(Face::F),
        "B" => Ok(Face::B),
        other => Err(ParseError::Unreachable(other.to_string())),
    }
}

impl_with_current_rule! {
    AxisMove;
    Rule::axis_move;

    |pair| {
        let mut parent = pair.into_inner();
        let face = Face::from_pest(parent.next().into_err()?)?;
        let amount = Amount::from_pest(parent.next().into_err()?)?;

        Ok(AxisMove::new(face, amount))
    }
}

impl_with_current_rule! {
    u32;
    Rule::depth;

    |pair| {
        if pair.as_str().is_empty() { Ok(0) } else { Ok(pair.as_str().parse::<u32>()?) }
    }
}

fn parse_face_small(pair: Pair<Rule>) -> Result<Face> {
    Face::parse(&pair.as_str().to_ascii_uppercase())
}

fn parse_face_wide(pair: Pair<Rule>) -> Result<Face> {
    let mut inner = pair.into_inner();
    let first = inner.next().into_err()?;

    match inner.next() {
        Some(_) => Face::from_pest(first),
        None => parse_face_small(first),
    }
}

impl<const N: u32> FromPest for WideAxisMove<N> {
    type Rule = Rule;
    type Parser = CubeNParser;

    fn rule() -> Self::Rule {
        Rule::wide_move
    }

    fn from_pest(pair: Pair<Self::Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();

        let first = inner.next().into_err()?;

        let (face, depth) = match first.as_rule() {
            Rule::face => (Face::from_pest(first)?, 0),
            Rule::depth => (
                parse_face_wide(inner.next().into_err()?)?,
                u32::from_pest(first)?.max(1),
            ),
            _ => return Err(ParseError::Unreachable(first.as_str().to_string())),
        };

        if depth > N {
            return Err(ParseError::uknown(WideMoveCreationError::ExcededDepth(
                depth, N,
            )));
        }

        let amount = Amount::from_pest(inner.next().into_err()?)?;

        WideAxisMove::new(face, amount, depth).map_err(ParseError::uknown)
    }
}

impl_with_current_rule! {
    Axis;
    Rule::axis;

    |pair| match pair.as_str() {
        "x" => Ok(Axis::X),
        "y" => Ok(Axis::Y),
        "z" => Ok(Axis::Z),
        other => Err(ParseError::Unreachable(other.to_string())),
    }
}

impl_with_current_rule! {
    AxisRotation;
    Rule::rotation;

    |pair| {
        let mut parent = pair.into_inner();
        let axis = Axis::from_pest(parent.next().into_err()?)?;
        let amount = Amount::from_pest(parent.next().into_err()?)?;

        Ok(AxisRotation::new(axis,amount))
    }
}

fn parse_slice(pair: Pair<Rule>) -> Result<(Axis, bool)> {
    match pair.as_str() {
        "M" => Ok((Axis::X, false)),
        "m" => Ok((Axis::X, true)),
        "E" => Ok((Axis::Y, false)),
        "e" => Ok((Axis::Y, true)),
        "S" => Ok((Axis::Z, false)),
        "s" => Ok((Axis::Z, true)),
        other => Err(ParseError::Unreachable(other.to_string())),
    }
}

fn parse_slice_move(pair: Pair<Rule>) -> Result<(AxisRotation, bool)> {
    let mut inner = pair.into_inner();

    let (axis, wide) = parse_slice(inner.next().into_err()?)?;
    let mut amount = Amount::from_pest(inner.next().into_err()?)?;

    // because E and M slices are inverted :eyeroll:
    if axis != Axis::Z {
        amount = amount * Direction::Negative;
    }

    Ok((AxisRotation::new(axis, amount), wide))
}

impl_with_current_rule! {
    ExtendedAxisMove;
    Rule::extended_move;

    |pair| {
        let pair = pair.into_inner().next().into_err()?;
        let mov = match pair.as_rule() {
            Rule::axis_move => ExtendedAxisMove::Regular(AxisMove::from_pest(pair)?),
            Rule::wide_move => {
                // If it's a depth 0 wide move, just make it an axis move
                let mov = WideAxisMove::from_pest(pair)?;
                match mov.depth() {
                    0 => ExtendedAxisMove::Regular(mov.axis_move),
                    _ => ExtendedAxisMove::Wide(mov),
                }
            },
            Rule::rotation => ExtendedAxisMove::Rotation(AxisRotation::from_pest(pair)?),
            Rule::slice_move => {
                let (rot, wide) = parse_slice_move(pair)?;
                ExtendedAxisMove::Slice { rot, wide }
            },
            _ => return Err(ParseError::Unreachable(pair.as_str().to_string())),
        };

        Ok(mov)
    }
}
