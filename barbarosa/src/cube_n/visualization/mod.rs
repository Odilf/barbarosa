//! Ways to visualize `NxN` cubes.

use core::fmt;

use crossterm::style::{ResetColor, SetBackgroundColor};
use palette::{IntoColor, Srgb};

use crate::{cube3::Cube3, generic::Piece};

use self::unfolded::Unfolded;

use super::{space::Face, Corner, Edge};

mod test;
pub mod unfolded;

/// An `NxN` sticker color. Newtype around face, since faces and colors biject. Mostly for clarity in type signatures.
#[derive(Clone, PartialEq, Eq)]
pub struct Color {
    /// The face that the color belongs to
    pub face: Face,
}

impl Color {
    /// Creates a new color based on its face
    pub const fn new(face: Face) -> Self {
        Self { face }
    }

    #[allow(missing_docs)]
    pub const RED: Self = Self::new(Face::R);
    #[allow(missing_docs)]
    pub const WHITE: Self = Self::new(Face::U);
    #[allow(missing_docs)]
    pub const GREEN: Self = Self::new(Face::F);
    #[allow(missing_docs)]
    pub const ORANGE: Self = Self::new(Face::L);
    #[allow(missing_docs)]
    pub const YELLOW: Self = Self::new(Face::D);
    #[allow(missing_docs)]
    pub const BLUE: Self = Self::new(Face::B);

    /// Writes the color to a formatter
    pub fn write(
        &self,
        colorscheme: &Colorscheme<Srgb>,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let color = colorscheme.get(&self.face);
        let color = crossterm::style::Color::Rgb {
            r: (color.red * 255.0) as u8,
            g: (color.green * 255.0) as u8,
            b: (color.blue * 255.0) as u8,
        };

        let set_color = SetBackgroundColor(color);

        write!(f, "{set_color}  {ResetColor}")
    }
}

/// Pieces on `NxNs` where you can extract color information
pub trait Colored: Piece {
    /// Returns a vec that maps faces to colors.
    // TODO: This should be an `impl Iterator`
    fn colors(&self, original_pos: Self::Position) -> Vec<(Face, Color)>;
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self.face {
            Face::R => "red",
            Face::L => "orange",
            Face::U => "white",
            Face::D => "yellow",
            Face::F => "green",
            Face::B => "blue",
        };

        write!(f, "Color {{ {color} }}")
    }
}

impl Colored for Corner {
    fn colors(&self, original_pos: Self::Position) -> Vec<(Face, Color)> {
        let mut current_axis = self.orientation_axis;
        let mut original_axis = Corner::ORIENTED_AXIS;

        let mut output = Vec::with_capacity(3);

        for _ in 0..3 {
            let current_face = Face::new(current_axis, self.position()[current_axis]);
            let original_face = Face::new(original_axis, original_pos[original_axis]);

            let color = Color::new(original_face);

            output.push((current_face, color));

            current_axis = Corner::next_axis(&self.position, &current_axis);
            original_axis = Corner::next_axis(&original_pos, &original_axis);
        }

        output
    }
}

impl Colored for Edge {
    fn colors(&self, original_pos: Self::Position) -> Vec<(Face, Color)> {
        let mut output = Vec::with_capacity(2);

        let orig_f1 = Edge::orientation_face(original_pos);
        let orig_f2 = Edge::non_orientation_face(original_pos);

        let mut cur_f1 = Edge::orientation_face(self.position());
        let mut cur_f2 = Edge::non_orientation_face(self.position());

        if !self.oriented {
            (cur_f1, cur_f2) = (cur_f2, cur_f1);
        }

        output.push((cur_f1, Color::new(orig_f1)));
        output.push((cur_f2, Color::new(orig_f2)));

        output
    }
}

/// A colorscheme for an `NxN` cube
pub struct Colorscheme<C: IntoColor<Srgb>> {
    data: [C; 6],
}

impl<C: IntoColor<Srgb>> Colorscheme<C> {
    /// Creates a new colorscheme given some colors in R, U, F, L, D, B order
    pub fn new(data: [C; 6]) -> Self {
        Self { data }
    }

    /// Gets the color of the given face
    pub fn get(&self, face: &Face) -> &C {
        &self.data[face.index()]
    }
}

impl Default for Colorscheme<Srgb> {
    fn default() -> Self {
        Self {
            data: [
                Srgb::new(0.529, 0.207, 0.207), // R, red
                Srgb::new(1.000, 1.000, 1.000), // U, white
                Srgb::new(0.207, 0.588, 0.227), // F, green
                Srgb::new(0.768, 0.443, 0.094), // L, orange
                Srgb::new(0.858, 0.686, 0.254), // D, yellow
                Srgb::new(0.207, 0.243, 0.439), // B, blue
            ],
        }
    }
}

impl fmt::Display for Cube3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unfolded: Unfolded<3> = self.clone().into();
        write!(f, "{unfolded}")
    }
}
