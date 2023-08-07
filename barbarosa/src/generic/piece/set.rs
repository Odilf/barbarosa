use core::hash::Hash;
use std::fmt::Debug;

use rand::{seq::SliceRandom, Rng};
use thiserror::Error;

use crate::generic::{Movable, Move};

use super::Piece;

/// A set of `N` pieces.
///
/// This struct provides methods in order to ensure that each piece is unique and that each position
/// exactly 1 piece. Some methods have an `_unchecked` variant which skips these checks.
/// 
/// [`PieceSet`] is not responsible for enforcing the specific invariants of the different piece types. 
/// For example, [`EdgeSet`](crate::cube_n::pieces::edge::EdgeSet) does not check that the parity of the edges is correct.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PieceSet<P: Piece<N>, const N: usize> {
    pieces: [P; N],
}

impl<P: Piece<N>, const N: usize> PieceSet<P, N> {
    /// Alias to [`Piece::SOLVED`]
    pub const SOLVED: Self = Self { pieces: P::SOLVED };

    /// Creates a new [`PieceSet`] from an array of pieces. Fails if one of the invariants is
    /// not upheld.
    pub fn new(pieces: [P; N]) -> Result<Self, ValidationError<P, N>> {
        let output = Self { pieces };

        match output.validate() {
            Some(err) => Err(err),
            None => Ok(output),
        }
    }

    /// Checks whether there is a duplicate piece or position in the set.
    pub fn validate(&self) -> Option<ValidationError<P, N>> {
        if let Some(dup) = find_duplicates(self.pieces.iter().map(P::position)) {
            return Some(ValidationError::DuplicatePosition(dup));
        }

        if let Some(dup) = find_duplicates(&self.pieces) {
            return Some(ValidationError::DuplicatePiece(dup.to_owned()));
        }

        None
    }

    /// Returns a reference to the underlying set of pieces
    pub fn pieces(&self) -> &[P; N] {
        &self.pieces
    }

    /// Zips the given iterator with the reference positions of the [`Piece`]
    pub fn zip_positions<T>(
        other: impl Iterator<Item = T>,
    ) -> impl Iterator<Item = (P::Position, T)> {
        P::REFERENCE_POSITIONS.into_iter().zip(other)
    }

    /// Iterator over pieces of [`PieceSet`]
    pub fn iter(&self) -> impl Iterator<Item = &P> {
        self.pieces.iter()
    }

    /// Iterator over the pieces and their original positions
    pub fn iter_with_pos(&self) -> impl Iterator<Item = (P::Position, &P)> {
        Self::zip_positions(self.pieces.iter())
    }

    /// Mutable iterator over the pieces. It doesn't check the invariants.
    // TODO: Validate anyway in debug mode
    pub fn iter_mut_unchecked(&mut self) -> impl Iterator<Item = &mut P> {
        self.pieces.iter_mut()
    }

    /// Whether the set is solved
    pub fn is_solved(&self) -> bool {
        self.iter_with_pos()
            .all(|(pos, piece)| piece.is_solved(&pos))
    }

    /// Returns the piece that was originally at `target_pos`
    pub fn piece_originally_at(&self, target_pos: &P::Position) -> &P {
        self.iter_with_pos()
            .find(|(pos, _piece)| pos == target_pos)
            .map(|(_, piece)| piece)
            .expect("There should be a piece at each position")
    }

    /// Returns the original position of the piece that is currently at `target_pos`.
    ///
    /// Returns [`None`] if there is no piece at `target_pos`.
    pub fn original_position_of_piece_at(&self, target_pos: &P::Position) -> P::Position {
        self.iter_with_pos()
            .find(|(_, piece)| piece.position() == *target_pos)
            .map(|(pos, _piece)| pos)
            .expect("There should be a piece at each position")
    }

    /// Returns the piece that is at `index`
    pub fn at_index(&self, index: usize) -> Option<&P> {
        self.pieces.get(index)
    }

    /// Shuffles the pieces according to the `rng`
    pub fn shuffle(&mut self, rng: &mut (impl Rng + ?Sized)) {
        self.pieces.shuffle(rng);
    }

    /// Swaps the pieces with original positions `a` and `b`
    // TODO: Check this. I don't think I can just unwrap the indices. Also there might be a nicer way to do this
    // TODO: Maybe it's easier to just swap the pieces with current positions `a` and `b`?
    pub fn swap(&mut self, a: P::Position, b: P::Position) {
        let index_a = P::REFERENCE_POSITIONS
            .iter()
            .position(|pos| pos == &a)
            .unwrap();

        let index_b = P::REFERENCE_POSITIONS
            .iter()
            .position(|pos| pos == &b)
            .unwrap();

        (self.pieces[index_a], self.pieces[index_b]) =
            (self.pieces[index_b].clone(), self.pieces[index_a].clone());
    }
}

fn find_duplicates<T: PartialEq>(iter: impl IntoIterator<Item = T>) -> Option<T> {
    let mut visited = Vec::new();

    for elem in iter.into_iter() {
        if visited.contains(&elem) {
            return Some(elem);
        }

        visited.push(elem);
    }

    None
}

#[derive(Debug, Error)]
pub enum ValidationError<P: Piece<N>, const N: usize> {
    #[error("Duplicate position ({0:?})")]
    DuplicatePosition(P::Position),

    #[error("Duplicate piece ({0:?})")]
    DuplicatePiece(P),
}

impl<M, P, const N: usize> Movable<M> for PieceSet<P, N>
where
    M: Move,
    P: Piece<N> + Movable<M>,
{
    fn apply(&mut self, m: &M) {
        for piece in &mut self.pieces {
            piece.apply(m);
        }
    }
}
