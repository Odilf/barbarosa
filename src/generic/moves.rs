//! Generic moves and generic move implementations

use strum::IntoEnumIterator;

use super::Piece;

/// A generic move.
pub trait Move: Sized + Clone {
    /// The inverse of this move, such that `m * m.inverse() == Self::identity()`
    fn inverse(&self) -> Self;
}

/// Something that can be moved.
///
/// A single type can be moved by multiple different types of moves. For example, a 4x4 can be moved
/// by a single [crate::cube_n::AxisMove], but also by [crate::cube_n::WideAxisMove].
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

// Auto implementation for arrays of pieces.
impl<P, M, const N: usize> Movable<M> for [P; N]
where
    P: Piece + Movable<M>,
    M: Move,
{
    fn apply(&mut self, m: &M) {
        for p in self {
            p.apply(m);
        }
    }
}

/// Returns the move that connects `from` to `to`, if it exists
///
/// # Example
///
/// ```rust
/// use barbarosa::generic::*;
/// use barbarosa::cube_n::{moves::AxisMove, Cube3};
///
/// let cube = Cube3::solved();
/// let mov = AxisMove::parse("B'").unwrap();
/// let moved = cube.clone().moved(&mov);
///
/// assert_eq!(moves::connect::<AxisMove, _>(cube, &moved), Some(mov));
/// ```
pub fn connect<M: Move + IntoEnumIterator, T: Movable<M> + Eq + Clone>(
    from: &T,
    to: &T,
) -> Option<M> {
    M::iter().find(|m| from.clone().moved(&m.clone()) == *to)
}
