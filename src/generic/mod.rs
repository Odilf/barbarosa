//! Generic cube traits and implementations

pub use self::{
    alg::Alg,
    moves::{Movable, Move},
    parse::Parsable,
    pieces::Piece,
    scramble::Scrambleable,
};

pub mod alg;
pub mod moves;
pub mod parse;

mod pieces;
mod scramble;

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
pub trait Cube: Sized + Clone + PartialEq + Eq + std::fmt::Debug {
    /// Returns a static reference to a solved cube.
    ///
    /// It's nice when implementing this to make the reference `const`, if possible.
    fn solved() -> &'static Self;

    /// The main type of move used by this cube, mainly used for convinience.
    type Move: Move;

    /// The type of algorithm used by this cube, almost always `Alg<Self::Move>`
    ///
    /// The only reason this is not default implemented as `Alg<Self::Move>` is because [associated type defaults
    /// are still unstable](https://github.com/rust-lang/rust/issues/29661)
    type Alg;

    /// Creates a new solved cube
    fn new_solved() -> Self
    where
        Self: 'static,
    {
        Self::solved().clone()
    }

    /// Checks whether a cube is solved by comparing it to [Cube::solved]   
    fn is_solved(&self) -> bool
    where
        Self: 'static,
    {
        self == Self::solved()
    }
}
