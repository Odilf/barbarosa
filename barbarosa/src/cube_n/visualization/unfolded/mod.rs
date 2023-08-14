//! Cube unfolding.
//!
//! See [`Unfolded`] for more info.

mod test;

use core::fmt;
use nalgebra::Vector3;
use palette::Srgb;
use std::{
    array,
    ops::{Index, IndexMut},
};

use crate::{
    cube3::Cube3,
    cube_n::{
        space::{faces, Face},
        visualization::Color,
    },
    generic::{
        piece::{Coordinates, PieceSetDescriptor},
        PieceSet,
    },
};

use super::{Colored, Colorscheme};

/// An `NxN` array of colors, used to represent a face of an `NxN` cube in [`Unfolded`]
pub struct Square<const N: usize> {
    /// The actual arrays of colors
    pub data: [[Color; N]; N],
}

impl<const N: usize> Square<N> {
    /// Creates a new square of the given color
    pub fn new(color: Color) -> Self {
        Self {
            data: array::from_fn(|_| array::from_fn(|_| color.clone())),
        }
    }

    fn basis(face: &Face) -> [Vector3<f32>; 2] {
        use faces::*;

        let faces = match *face {
            U => [F, R],
            F => [D, R],
            D => [B, R],
            B => [U, R],
            L => [D, F],
            R => [D, B],
        };

        faces.map(|face| {
            let mut basis = Vector3::zeros();
            basis[face.axis as usize] = face.direction.scalar() as f32;

            basis
        })
    }

    fn indices(point: &Vector3<f32>, face: &Face) -> Option<[usize; 2]> {
        let [bx, by] = Self::basis(face);

        let x = point.dot(&bx);
        let y = point.dot(&by);

        let x = Self::transform_coord(x)?;
        let y = Self::transform_coord(y)?;

        Some([x, y])
    }

    /// The color at the given coordinates. The vector is projected to the given face and then the color is retrieved.
    ///
    /// Note: Since the coordinates are `f32` trying to match exact values is a bad idea (because of floating point
    /// inaccuracies). Instead, we return the face that is closest to the coordinates. If the vector is too far outside
    /// the bounds of the square, `None` is returned. The meaning of "too far" is kind of messy and shoudn't be relied upon,
    /// but it gets smaller the bigger the `N`.
    pub fn at(&self, point: &Vector3<f32>, face: &Face) -> Option<&Color> {
        let [x, y] = Self::indices(point, face)?;

        Some(&self.data[x][y])
    }

    /// Mutable reference to the color at the given coordinates.
    ///
    /// See also the note from [`Square::at`]
    pub fn at_mut(&mut self, point: &Vector3<f32>, face: &Face) -> Option<&mut Color> {
        let [x, y] = Self::indices(point, face)?;

        Some(&mut self.data[x][y])
    }

    fn transform_coord(x: f32) -> Option<usize> {
        let x = x + 1.0;
        let x = x * N as f32 / 3.0;
        let x = x.round();

        if x < 0.0 || x as usize >= N {
            return None;
        }

        Some(x as usize)
    }

    /// Writes the square into a formatter with the specified colorscheme and padding at the front
    pub fn write(
        &self,
        colorscheme: &Colorscheme<Srgb>,
        padding: usize,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for row in &self.data {
            for _ in 0..padding {
                write!(f, "  ")?;
            }

            for color in row {
                color.write(colorscheme, f)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

/// An unfolded representation of a cube of size `N`.
///
/// The unfolded structure is the following:
///
/// ```text
///   U
/// R F L
///   D
///   B
/// ```
///
/// Note that the way the unfolding works is like if it were a piece of paper. This means that the B face is not obtained by doing a y2 rotation,
/// but instead by doing an x2, which might make it look like it's "upside down".
///
/// Mainly used for visualization.
pub struct Unfolded<const N: usize> {
    faces: [Square<N>; 6],
}

impl<const N: usize> Default for Unfolded<N> {
    fn default() -> Self {
        Self {
            faces: [
                Color::RED,
                Color::WHITE,
                Color::GREEN,
                Color::ORANGE,
                Color::YELLOW,
                Color::BLUE,
            ]
            .map(Square::new),
        }
    }
}

impl<const N: usize> Unfolded<N> {
    /// Iterates over a tuple of squares, and the face the square belong to
    pub fn iter(&self) -> impl Iterator<Item = (Face, &Square<N>)> {
        Face::iter().zip(self.faces.iter())
    }

    /// Mutable reference of the pice at the given coordinates.
    ///
    /// See note from [`Square::at`]
    pub fn at_mut(&mut self, coords: &Vector3<f32>, face: &Face) -> Option<&mut Color> {
        self[face].at_mut(coords, face)
    }

    /// Populates the unfolded cube with the pieces of the piece set.
    ///
    /// # Panics
    ///
    /// If there is a piece on the set that has coordinates too far outside the range [-1, 1]
    pub fn populate_with<P: Colored + Coordinates + PieceSetDescriptor<M>, const M: usize>(
        &mut self,
        piece_set: &PieceSet<P, M>,
    ) {
        for (original_pos, piece) in piece_set.iter_with_pos() {
            for (face, color) in piece.colors(original_pos) {
                *self.at_mut(&piece.coordinates(), &face).unwrap() = color;
            }
        }
    }

    /// Writes the unfolded cube into a formatter with the given colorscheme
    pub fn write(
        &self,
        colorscheme: &Colorscheme<Srgb>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let padding = N + 1;

        self[&Face::U].write(colorscheme, padding, f)?;

        writeln!(f)?;

        for ((l_row, f_row), r_row) in self[&Face::L]
            .data
            .iter()
            .zip(self[&Face::F].data.iter())
            .zip(self[&Face::R].data.iter())
        {
            for color in l_row {
                color.write(colorscheme, f)?;
            }

            write!(f, "  ")?;

            for color in f_row {
                color.write(colorscheme, f)?;
            }

            write!(f, "  ")?;

            for color in r_row {
                color.write(colorscheme, f)?;
            }

            writeln!(f)?;
        }

        writeln!(f)?;

        self[&Face::D].write(colorscheme, padding, f)?;

        writeln!(f)?;

        self[&Face::B].write(colorscheme, padding, f)?;

        writeln!(f)?;

        Ok(())
    }
}

impl<const N: usize> Index<&Face> for Unfolded<N> {
    type Output = Square<N>;

    fn index(&self, face: &Face) -> &Self::Output {
        &self.faces[face.index()]
    }
}

impl<const N: usize> IndexMut<&Face> for Unfolded<N> {
    fn index_mut(&mut self, face: &Face) -> &mut Self::Output {
        &mut self.faces[face.index()]
    }
}

impl From<Cube3> for Unfolded<3> {
    fn from(cube: Cube3) -> Self {
        let mut unfolded = Self::default();

        unfolded.populate_with(&cube.corners);
        unfolded.populate_with(&cube.edges);

        unfolded
    }
}

impl<const N: usize> fmt::Display for Unfolded<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(&Colorscheme::default(), f)
    }
}
