//! Generic moves and generic move implementations

use strum::IntoEnumIterator;

/// A generic move.
pub trait Move: Sized + Clone {
    /// The inverse of this move, such that `m * m.inverse() == Self::identity()`
    fn inverse(&self) -> Self;
}

/// Something that can be used as a move. Basically, [`Move`]s and [`Cube`](super::Cube)s implement [`AsMove`].
pub trait AsMove {
    /// The move type of this object
    type Move: Move;
}

/// Something that can be moved.
///
/// A single type can be moved by multiple different types of moves. For example, a 4x4 can be moved
/// by a single [`crate::cube_n::AxisMove`], but also by [`crate::cube_n::WideAxisMove`].
///
/// [Movable] is auto-implemented for arrays of movables.
// Note: M is `?Sized` to work with slices and vecs.
pub trait Movable<M: ?Sized>: Sized {
    /// Applies a move to this object (in-place)
    fn apply(&mut self, m: &M);

    /// Moves this object, taking and returning ownership
    fn moved(mut self, m: &M) -> Self {
        self.apply(m);
        self
    }
}

macro_rules! impl_movable_array {
    ($piece:ty, $mov:ty) => {
        impl<const N: usize> crate::generic::Movable<$mov> for [$piece; N] {
            fn apply(&mut self, m: &$mov) {
                for piece in self {
                    piece.apply(m);
                }
            }
        }
    };
}

pub(crate) use impl_movable_array;

impl<M: Move> AsMove for M {
    type Move = Self;
}

/// Returns the move that connects `from` to `to`, if it exists
///
/// # Example
///
/// ```rust
/// use barbarosa::prelude::*;
/// use barbarosa::generic::moves::connect;
///
/// let cube = Cube3::SOLVED;
/// let mov = AxisMove::parse("B'").unwrap();
/// let moved = cube.clone().moved(&mov);
///
/// assert_eq!(connect::<AxisMove, _>(&cube, &moved), Some(mov));
/// ```
pub fn connect<M: Move + IntoEnumIterator, T: Movable<M> + Eq + Clone>(
    from: &T,
    to: &T,
) -> Option<M> {
    M::iter().find(|m| from.clone().moved(&m.clone()) == *to)
}
