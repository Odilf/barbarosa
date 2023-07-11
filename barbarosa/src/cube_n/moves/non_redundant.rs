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
    fn single(axis: Axis, direction: Direction, amount: Amount) -> Self {
        NonRedundantAxisMove::Single(AxisMove::new(Face::new(axis, direction), amount))
    }

    fn double(axis: Axis, amount_positive: Amount, amount_negative: Amount) -> Self {
        NonRedundantAxisMove::Double {
            axis,
            amount_positive,
            amount_negative,
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            NonRedundantAxisMove::Single(mov) => mov.face.axis,
            NonRedundantAxisMove::Double { axis, .. } => *axis,
        }
    }

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
                    (Some(pos), None) => Some(Self::single(axis, Direction::Positive, pos)),
                    (None, Some(neg)) => Some(Self::single(axis, Direction::Negative, neg)),
                    (Some(pos), Some(neg)) => Some(Self::double(axis, pos, neg)),
                }
            })
        })
    }

    /// Returns every non-redundant move given the axis of the last move.
    ///
    /// # Example
    ///
    /// ```rust
    /// use barbarosa::cube_n::{moves::NonRedundantAxisMove, space::Axis};
    ///
    /// let moves: Vec<_> = NonRedundantAxisMove::given_last_axis(&Axis::X).collect();
    ///
    /// assert_eq!(moves.len(), 30);
    ///
    /// // `moves` doesn't contain any move on the X axis (so no L or R moves)
    /// ```
    pub fn given_last_axis(last_axis: &Axis) -> impl Iterator<Item = NonRedundantAxisMove> + '_ {
        Axis::basis(last_axis).into_iter().flat_map(Self::of_axis)
    }

    pub fn all() -> impl Iterator<Item = Self> {
        Axis::iter().flat_map(Self::of_axis)
    }
}

/// Tries to absorve an [AxisMove] into a [NonRedundantAxisMove].
///
/// In this context, "absorve" means modifying the original [NonRedundantAxisMove] in such
/// a way that the result is the same as doing both moves sequentially.
///
/// This function modifies `self` in-place. It returns `Ok(())` when it is possible to absorve
/// the move, and an [AbsorveError] otherwise.
///
/// # Example
///
/// ```rust
/// use barbarosa::{cube_n::{moves::non_redundant::{NonRedundantAxisMove, absorve, AbsorveResult}, AxisMove}, generic::Parsable};
///
/// // Quick function to make example more readable
/// let parse = |mov| AxisMove::parse(mov).unwrap();
///
/// let moves_and_expected = [
///     ("R2", "R2"),
///     ("R",  "R'"),
///     ("L", "R' L"),
///     ("R'", "R2 L"),
/// ];
///
/// let mut non_redundant = None;
///
/// for (mov, expected) in moves_and_expected {
///     absorve(&mut non_redundant, &parse(mov));
///
///     assert_eq!(non_redundant.as_ref().unwrap().to_string(), expected);
/// }
///
/// // Can't absorve F (or any non R or L move)
/// assert_eq!(absorve(&mut non_redundant, &parse("F")), AbsorveResult::NotAdded);
///
/// // We can cancel out the current moves to get back to `None`
/// absorve(&mut non_redundant, &parse("L'"));
/// absorve(&mut non_redundant, &parse("R2"));
///
/// assert!(non_redundant.is_none());
///
/// // And we *can* absorve F (or any other move) into `None`
/// let result = absorve(&mut non_redundant, &AxisMove::parse("F").unwrap());
/// assert_ne!(result, AbsorveResult::NotAdded);
/// ```
pub fn absorve(
    nr_move_option: &mut Option<NonRedundantAxisMove>,
    other: &AxisMove,
) -> AbsorveResult {
    let Some(ref mut nr_move) = nr_move_option else {
        *nr_move_option = Some(NonRedundantAxisMove::Single(other.clone()));
        return AbsorveResult::Absorved;
    };

    match nr_move {
        NonRedundantAxisMove::Single(ref mut mov) if mov.face.axis == other.face.axis => {
            use Direction::*;

            match (mov.face.direction, other.face.direction) {
                // If faces are the same, just add the amounts
                (Positive, Positive) | (Negative, Negative) => match mov.amount + other.amount {
                    Some(amount) => {
                        mov.amount = amount;
                        AbsorveResult::Absorved
                    }
                    None => {
                        *nr_move_option = None;
                        AbsorveResult::Collapsed
                    }
                },

                // If faces are different, add the move to `nr_move`
                (Positive, Negative) => {
                    *nr_move =
                        NonRedundantAxisMove::double(mov.face.axis, mov.amount, other.amount);
                    AbsorveResult::Added
                }
                (Negative, Positive) => {
                    *nr_move =
                        NonRedundantAxisMove::double(mov.face.axis, other.amount, mov.amount);
                    AbsorveResult::Added
                }
            }
        }

        NonRedundantAxisMove::Double {
            axis,
            ref mut amount_positive,
            ref mut amount_negative,
        } if *axis == other.face.axis => {
            let (match_amount, other_amount) = match other.face.direction {
                Direction::Positive => (amount_positive, amount_negative),
                Direction::Negative => (amount_negative, amount_positive),
            };

            match *match_amount + other.amount {
                Some(amount) => {
                    *match_amount = amount;
                    AbsorveResult::Absorved
                }
                None => {
                    *nr_move =
                        NonRedundantAxisMove::single(*axis, -other.face.direction, *other_amount);
                    AbsorveResult::Collapsed
                }
            }
        }

        _ => {
            debug_assert_ne!(nr_move.axis(), other.face.axis);

            return AbsorveResult::NotAdded;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AbsorveResult {
    /// The move was absorved into the [NonRedundantAxisMove]. E.g.: `R2 + R => R'`
    Absorved,

    /// The move was not absorved, but it was added to the [NonRedundantAxisMove]. E.g.: `L + R2 => R2 L`
    Added,

    /// The move was not absorved, and it was not added to the [NonRedundantAxisMove]. E.g.: `R2 + F => Whoops`
    NotAdded,

    /// The move cancelled out with (one of the) [NonRedundantAxisMove]\(s\). E.g.: `R + R' => None` or `R2 L + L' => R2`
    Collapsed,
}

impl Alg<AxisMove> {
    pub fn random_with_rng(length: usize, rng: &mut impl Rng) -> Self {
        let mut moves: Vec<AxisMove> = Vec::new();

        while moves.len() < length {
            let chosen = match moves.get(0) {
                Some(mov) => NonRedundantAxisMove::given_last_axis(&mov.face.axis)
                    .choose(rng)
                    .expect("`given_last_axis` returns 30 elements"),

                None => NonRedundantAxisMove::all()
                    .choose(rng)
                    .expect("`all` returns 45 elements"),
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

    /// Returns a new `Alg` that does the same thing as `self`, but with redundant moves removed.
    ///
    /// # Example
    ///
    /// ```rust
    /// use barbarosa::{
    ///     generic::{Parsable, Alg},
    ///     cube_n::{moves::non_redundant::NonRedundantAxisMove, AxisMove},
    /// };
    ///
    /// let alg = Alg::<AxisMove>::parse("L' F R' L2 R L2 F' L B F B D").unwrap();
    /// let normalized = alg.normalized();
    ///
    /// assert_eq!(normalized.to_string(), "B2 F D");
    /// ```
    pub fn normalized(mut self) -> Self {
        let mut moves = Vec::with_capacity(self.moves.len());

        let mut nr_move: Option<NonRedundantAxisMove> = None;

        // `offload` takes any moves in `nr_move` and pushes it to `moves`
        let offload = |nr_move: &mut Option<NonRedundantAxisMove>, moves: &mut Vec<AxisMove>| {
            if let Some(nr_move) = nr_move.take() {
                for mov in nr_move.moves() {
                    moves.push(mov);
                }
            }
        };

        while let Some(mov) = &self.moves.pop() {
            match absorve(&mut nr_move, mov) {
                // If we didn't add it, it means we have to offload what we have and start again from `None`
                AbsorveResult::NotAdded => {
                    offload(&mut nr_move, &mut moves);
                    absorve(&mut nr_move, mov);
                }

                // If we collapsed something, we need to go back one move to see if we can join it with something else
                AbsorveResult::Collapsed => {
                    offload(&mut nr_move, &mut moves);
                    if let Some(mov) = moves.pop() {
                        self.moves.push(mov);
                    }
                }

                _ => (),
            }
        }

        // offload the last moves
        offload(&mut nr_move, &mut moves);

        // Reverse since we were pushing it to a stack
        moves.reverse();

        Self::new(moves)
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
