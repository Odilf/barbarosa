mod test;

use pest::iterators::Pair;
use pest_derive::Parser;

use crate::generic::parse::{FromPest, Parsable};

use super::{moves::Amount, space::Face, AxisMove, WideAxisMove};

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

            fn from_pest($arg: Pair<Self::Rule>) -> Self {
                $body
            }
        }
    };
}

impl_with_current_rule! {
    Amount;
    Rule::amount;

    |pair| {
        match pair.as_str() {
            "" => Amount::Single,
            "2" => Amount::Double,
            "'" => Amount::Inverse,
            _ => unreachable!(),
        }
    }
}

impl_with_current_rule! {
    Face;
    Rule::face;

    |pair| match pair.as_str() {
        "U" => Face::U,
        "D" => Face::D,
        "L" => Face::L,
        "R" => Face::R,
        "F" => Face::F,
        "B" => Face::B,
        _ => unreachable!(),
    }
}

impl_with_current_rule! {
    AxisMove;
    Rule::axis_move;

    |pair| {
        let mut parent = pair.into_inner();
        let face = Face::from_pest(parent.next().unwrap());
        let amount = Amount::from_pest(parent.next().unwrap());

        AxisMove::new(face, amount)
    }
}

impl_with_current_rule! {
    u32;
    Rule::depth;

    |pair| {
        if pair.as_str().len() == 0 { 0 } else { pair.as_str().parse().unwrap() }
    }
}

fn parse_face_small(pair: Pair<Rule>) -> Face {
    Face::parse(&pair.as_str().to_ascii_uppercase()).unwrap()
}

fn parse_face_wide(pair: Pair<Rule>) -> Face {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

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

    fn from_pest(pair: Pair<Self::Rule>) -> Self {
        let mut inner = pair.into_inner();

        let first = inner.next().unwrap();

        let (face, depth) = match first.as_rule() {
            Rule::face => (Face::from_pest(first), 0),
            Rule::depth => (
                parse_face_wide(inner.next().unwrap()),
                u32::from_pest(first).max(1),
            ),
            _ => unreachable!(),
        };

        if depth > N {
            panic!("Depth {} is greater than N {}", depth, N);
        }

        let amount = Amount::from_pest(inner.next().unwrap());

        WideAxisMove::new(face, amount, depth).unwrap()
    }
}
