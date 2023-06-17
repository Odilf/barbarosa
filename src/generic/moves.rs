use super::Piece;

/// A generic move.
pub trait Move: Sized + Clone {
    /// The inverse of this move, such that `m * m.inverse() == Self::identity()`
    fn inverse(&self) -> Self;

    /// The iterator type that [Self::iter()] returns
    type Iter: Iterator<Item = Self>;

    /// Iterator over all possible moves
    fn iter() -> Self::Iter;

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
    /// let moved = cube.clone().moved(&mov.clone().into());
    ///
    /// assert_eq!(AxisMove::connect(cube, &moved), Some(mov));
    /// ```
    fn connect<T: Movable<Self> + Eq + Clone>(from: &T, to: &T) -> Option<Self> {
        Self::iter().find(|m| from.clone().moved(&m.clone()) == *to)
    }
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

    /// Returns every state that can be reached by applying a single move to this object
    fn successors(&self) -> Vec<Self>
    where
        Self: Clone,
    {
        todo!()
        // M::iter()
        //     .map(|m| {
        //         let mut new = self.clone();
        //         new.apply_move(&m);
        //         new
        //     })
        //     .collect()
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
