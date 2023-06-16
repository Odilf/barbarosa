use nalgebra::Vector3;

/// A piece on the cube
pub trait Piece {
    /// The coordinates in space of the (center of the) piece
    fn coordinates(&self) -> Vector3<f32>;
}
