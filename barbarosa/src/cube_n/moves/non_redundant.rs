use std::iter;

use itertools::{Either, Itertools};
use rand::{seq::IteratorRandom, Rng};
use strum::IntoEnumIterator;

use crate::{
    cube_n::space::{Axis, Direction, Face},
    generic::Alg,
};

use super::{Amount, AxisMove};

/// A type of move used to prevent redundancies in [AxisMove]s.
///
/// This redundancy arises
/// because you can have something like `R R` or `R L R' L'` which can clearly be simplified.
/// [NonRedundantAxisMove] implements this by encoding all possible types of move in one axis.
/// This way you can check the axis of the previous move and select only moves from another axis
/// on the next one (see [Self::of_axis] and [Self::given_last_axis] for more info).
///
/// # Note
///
/// Technically, this doesn't prevent all redundancies. For example, you could make six sexy moves or
/// two T perms at any point and that's cleary simplifiable. Actually, every sequence of more than 20
/// moves is always going to have some redundancy. However, finding this redundancies basically as
/// hard as solving the cube, so it misses the point. [NonRedundantAxisMove] is just meant to reduce
/// the very obvious and very common redundancies.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum NonRedundantAxisMove {
    Single(AxisMove),
    Double {
        axis: Axis,
        amount_positive: Amount,
        amount_negative: Amount,
    },
}

impl NonRedundantAxisMove {
    /// Returns an iterator over the moves that this move represents. This iterator has either one or two items.
    pub fn moves(&self) -> impl Iterator<Item = AxisMove> {
        match self {
            NonRedundantAxisMove::Single(mov) => Either::Left(std::iter::once(mov.clone())),
            NonRedundantAxisMove::Double {
                axis,
                amount_positive,
                amount_negative,
            } => Either::Right(
                [
                    AxisMove::new(Face::new(*axis, Direction::Positive), *amount_positive),
                    AxisMove::new(Face::new(*axis, Direction::Negative), *amount_negative),
                ]
                .into_iter(),
            ),
        }
    }

    /// Returns all possible combinations of moves that can be made with the given axis
    pub fn of_axis(axis: Axis) -> impl Iterator<Item = NonRedundantAxisMove> {
        let amounts_with_none = || iter::once(None).chain(Amount::iter().map(Some));

        amounts_with_none().flat_map(move |amount_positive| {
            amounts_with_none().filter_map(move |amount_negative| {
                match (amount_positive, amount_negative) {
                    (None, None) => None,

                    (Some(amount_positive), None) => Some(NonRedundantAxisMove::Single(
                        AxisMove::new(Face::new(axis, Direction::Positive), amount_positive),
                    )),

                    (None, Some(amount_negative)) => Some(NonRedundantAxisMove::Single(
                        AxisMove::new(Face::new(axis, Direction::Negative), amount_negative),
                    )),

                    (Some(amount_positive), Some(amount_negative)) => {
                        Some(NonRedundantAxisMove::Double {
                            axis,
                            amount_positive,
                            amount_negative,
                        })
                    }
                }
            })
        })
    }

    /// Returns every non-redundant move given the axis of the last move.
    ///
    /// # Example
    ///
    /// ```rust
    /// use barbarosa::cube_n::moves::{NonRedundantAxisMove, Axis};
    ///
    /// let moves: Vec<_> = NonRedundantAxisMove::given_last_axis(&Axis::X).collect();
    ///
    /// assert_eq!(moves.len(), 30);
    ///
    /// // `moves` doesn't contain any move on the X axis (so no L or R moves)
    /// ```
    pub fn given_last_axis(last_axis: &Axis) -> impl Iterator<Item = NonRedundantAxisMove> + '_ {
        Axis::iter()
            .filter(move |axis| axis != last_axis)
            .flat_map(Self::of_axis)
    }

    pub fn all() -> impl Iterator<Item = Self> {
        Axis::iter().flat_map(Self::of_axis)
    }
}

impl Alg<AxisMove> {
    pub fn random_with_rng(length: usize, rng: &mut impl Rng) -> Self {
        let mut moves: Vec<AxisMove> = Vec::new();

        while moves.len() < length {
            let chosen = match moves.get(0) {
                Some(mov) => NonRedundantAxisMove::given_last_axis(&mov.face.axis)
                    .choose(rng)
                    .expect("`given_last_axis` always returns 30 elements"),

                None => NonRedundantAxisMove::all()
                    .choose(rng)
                    .expect("`all` always returns 45 elements"),
            };

            for mov in chosen.moves() {
                moves.push(mov);
            }
        }

        Alg::new(moves)
    }

    pub fn random(length: usize) -> Self {
        Self::random_with_rng(length, &mut rand::thread_rng())
    }
}

impl From<AxisMove> for NonRedundantAxisMove {
    fn from(value: AxisMove) -> Self {
        NonRedundantAxisMove::Single(value)
    }
}

impl From<NonRedundantAxisMove> for Alg<AxisMove> {
    fn from(value: NonRedundantAxisMove) -> Self {
        Alg::new(value.moves().collect::<Vec<_>>())
    }
}

impl std::fmt::Display for NonRedundantAxisMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.moves().map(|mov| mov.to_string()).join(" "))
    }
}
