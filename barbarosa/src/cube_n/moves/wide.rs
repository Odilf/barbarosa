//! Moves for 4x4x4 and up

use std::{mem, ops::Deref};

use rand::prelude::Distribution;
use thiserror::Error;

use crate::cube_n::space::Face;
pub use crate::generic::{
    self,
    parse::{self, Parsable},
};

use self::generic::{Alg, Cube, Movable, Piece, PieceSet};

use super::{
    rotation::{AxisRotation, Rotatable},
    Amount, AxisMove,
};

/// A wide move of at most depth `N`.
///
/// [`WideAxisMove<0>`] should never be implemented directly. Instead, you should implement [`AxisMove`]
/// which automatically implements [`WideAxisMove<0>`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WideAxisMove<const N: u32> {
    /// invariant: depth <= N
    depth: u32,

    /// The corresponding depth 0 axis move
    pub axis_move: AxisMove,
}

impl<const N: u32> generic::Move for WideAxisMove<N> {
    fn inverse(&self) -> Self {
        let mut output = self.clone();
        output.axis_move = output.axis_move.inverse();
        output
    }
}

impl<const N: u32> WideAxisMove<N> {
    /// Creates a new [`WideAxisMove<N>`].
    ///
    /// Returns [WideMoveCreationError] if the depth is greater than `N`.
    pub fn new(face: Face, amount: Amount, depth: u32) -> Result<Self, WideMoveCreationError> {
        if depth > N {
            return Err(WideMoveCreationError::ExcededDepth(depth, N));
        }

        Ok(Self {
            depth,
            axis_move: AxisMove { face, amount },
        })
    }

    /// Face of the move
    pub fn face(&self) -> &Face {
        &self.axis_move.face
    }

    /// Amount of the move
    pub fn amount(&self) -> Amount {
        self.axis_move.amount
    }

    /// Depth of the move
    pub fn depth(&self) -> u32 {
        self.depth
    }

    /// Tries to set the depth of the move in-place.
    ///
    /// Fails and returns [WideMoveCreationError] if the depth is greater than `N`.
    pub fn set_depth(&mut self, new_depth: u32) -> Result<(), WideMoveCreationError> {
        if new_depth > N {
            return Err(WideMoveCreationError::ExcededDepth(new_depth, N));
        }

        self.depth = new_depth;

        Ok(())
    }

    /// Returns a new [`WideAxisMove<N>`] with the max depth set to `M`.
    pub fn set_max_depth<const M: u32>(self) -> Result<WideAxisMove<M>, WideMoveCreationError> {
        if self.depth > M {
            return Err(WideMoveCreationError::ExcededDepth(self.depth, M));
        }

        // Safe because we're changing between wide axis moves which have the same exact structure
        let output = unsafe { mem::transmute::<WideAxisMove<N>, WideAxisMove<M>>(self) };

        Ok(output)
    }
}

/// An error if the depth of a wide move is greater than `N`.
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum WideMoveCreationError {
    #[error("Exceded maximum depth (given: {0}, max: {1})")]
    ExcededDepth(u32, u32),
}

impl<const N: u32> std::fmt::Display for WideAxisMove<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.depth {
            0 => write!(f, "{}", self.axis_move),
            1 => write!(f, "{}w{}", self.axis_move.face, self.axis_move.amount),
            i => write!(
                f,
                "{}{}w{}",
                i + 1,
                self.axis_move.face,
                self.axis_move.amount
            ),
        }
    }
}

impl<C: Cube + Movable<AxisMove>> Movable<WideAxisMove<0>> for C {
    fn apply(&mut self, m: &WideAxisMove<0>) {
        self.apply(&m.axis_move);
    }
}

macro_rules! impl_movable_wide_move_inductively {
    ($cube:ty, $max_width:literal, [$($width:tt),*]) => {
        $(
            impl_movable_wide_move_inductively!($cube, $max_width, $width);
        )*
    };

    ($cube:ty, $max_width:literal, 0) => {
        impl crate::generic::Movable<crate::cube_n::AxisMove> for $cube {
            fn apply(&mut self, m: &crate::cube_n::AxisMove) {
                <$cube as crate::generic::Movable<WideAxisMove<$max_width>>>::apply(self, &m.clone().widen($max_width).unwrap());
            }
        }
    };

    ($cube:ty, $max_width:literal, $width:literal) => {
        static_assertions::const_assert!($width < $max_width);
        impl crate::generic::Movable<WideAxisMove<$width>> for $cube {
            fn apply(&mut self, m: &WideAxisMove<$width>) {
                // Safe because width is statically asserted to be less than max_width
                // (and even if it wasn't it wouldn't be memory unsafe, just incorrect)
                let wider = unsafe {
                    std::mem::transmute::<&WideAxisMove<$width>, &WideAxisMove<$max_width>>(m)
                };

                self.apply(wider);
            }
        }
    };
}

pub(crate) use impl_movable_wide_move_inductively;

impl<const N: u32> Distribution<WideAxisMove<N>> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> WideAxisMove<N> {
        let face = rng.gen();
        let amount = rng.gen();
        let depth = rng.gen_range(0..=N);
        WideAxisMove::new(face, amount, depth).unwrap()
    }
}

impl Alg<AxisMove> {
    /// Widens an axis move into a [WideAxisMove].
    ///
    /// Fails if the depth is greater than `N`.
    pub fn widen<const N: u32>(
        self,
        depth: u32,
    ) -> Result<Alg<WideAxisMove<N>>, WideMoveCreationError> {
        self.moves
            .into_iter()
            .map(|axis_move| axis_move.widen(depth))
            .collect()
    }
}

/// A piece that can be moved by a wide move
pub trait DepthPiece<const N: usize>: Piece<N> {
    /// Whether the piece is in the wide move if it has the given normal and tangent depth.
    fn is_in_wide_move<const M: u32>(
        &self,
        normal_depth: u32,
        tangent_depth: u32,
        m: &WideAxisMove<M>,
    ) -> bool;
}

/// A set of pieces that have varying depths and can be moved by a wide move.
///
/// Actually, this struct is just a wrapper around a [`PieceSet`] that adds the depth information as generics and
/// implements [`Movable`] for [`WideAxisMove`]s.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DepthPieceSet<
    P: DepthPiece<N>,
    const N: usize,
    const NORMAL_DEPTH: u32,
    const TANGENT_DEPTH: u32 = 0,
> {
    /// The original set of pieces.
    pub set: PieceSet<P, N>,
}

impl<P: DepthPiece<N>, const N: usize, const ND: u32, const TD: u32> DepthPieceSet<P, N, ND, TD> {
    /// Alias to [`Piece::SOLVED`]
    pub const SOLVED: Self = Self {
        set: PieceSet::SOLVED,
    };
}

impl<P: DepthPiece<N> + Rotatable, const M: u32, const N: usize, const ND: u32, const TD: u32>
    Movable<WideAxisMove<M>> for DepthPieceSet<P, N, ND, TD>
{
    fn apply(&mut self, m: &WideAxisMove<M>) {
        self.set
            .iter_mut_unchecked()
            .filter(|piece| piece.is_in_wide_move::<M>(ND, TD, m))
            .for_each(|piece| piece.rotate(&AxisRotation::from(&m.axis_move)));
    }
}

impl<P: DepthPiece<N>, const N: usize, const ND: u32, const TD: u32> Deref
    for DepthPieceSet<P, N, ND, TD>
{
    type Target = PieceSet<P, N>;

    fn deref(&self) -> &Self::Target {
        &self.set
    }
}
