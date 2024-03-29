//! Generic cube traits and implementations

use self::moves::AsMove;
pub use self::{
    alg::Alg,
    moves::{Movable, Move},
    parse::Parsable,
    piece::{Piece, PieceSet},
    scramble::Scrambleable,
};

pub mod alg;
pub mod moves;
pub mod parse;
pub mod piece;
pub mod search;
pub mod utils;

mod scramble;
mod visualization;

/// A generic cube (or twisty puzzle)
///
/// This trait is implemented by all cubes. It's the main trait of this library.
///
/// However, this trait is pretty barebones. For example, it can't be moved or scrambled (see [Movable] and [Scrambleable])
///
/// # About the name
///
/// Not all Rubik's cubes are actual cubes. And the name "Cube" seems way to generic. It's not
/// any cube after all, it's a *Rubik's* cube. I thought about calling it `RubiksCube`. I also
/// thought of naming it `TwistyPuzzle`. But that... didn't feel right. The community it's not
/// the "Twisty Puzzle" community, it's the **"Cubing"** community. We solve **cubes**. Sure, half
/// of the puzzles we solve aren't cubes, but that's not the point. Words are what we make them
/// and the name for the thing about which I'm writing this library about is "cube". So I'm calling
/// the trait `Cube`.
///
/// Also the cubing community has plenty of other dubious names, like "algorithm" (as in a sequence
/// of moves) and "permutations" (as in T perm). So I'm just going to call it a cube. suck it
pub trait Cube:
    Sized + Clone + PartialEq + Eq + std::fmt::Debug + AsMove + Movable<Self::Move>
{
    /// Returns a static reference to a solved cube.
    ///
    /// It's nice when implementing this to make the reference `const`, if possible.
    // fn solved() -> &'static Self;

    const SOLVED: Self;

    /// Creates a new solved cube
    fn new_solved() -> Self
    where
        Self: 'static,
    {
        Self::SOLVED.clone()
    }

    /// Checks whether a cube is solved by comparing it to [`Cube::SOLVED`]
    fn is_solved(&self) -> bool
    where
        Self: 'static,
    {
        *self == Self::SOLVED
    }
}
