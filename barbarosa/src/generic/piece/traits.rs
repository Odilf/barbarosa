use nalgebra::Vector3;

use super::Piece;

/// Position in space
pub trait Coordinates: Piece {
	/// Returns the coordinates of the piece based on its position
    fn coordinates_pos(position: Self::Position) -> Vector3<f32>;

	/// Returns the coordinates of the piece
    fn coordinates(&self) -> Vector3<f32> {
        Self::coordinates_pos(self.position())
    }
}
