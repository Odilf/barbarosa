//! Generic utilities

use nalgebra::{Vector2, Vector3};

use crate::cube_n::space::Axis;

macro_rules! map_array_const {
    ($array:expr, $array_length:literal, $f:expr) => {
        {
            const_assert_eq!($array.len(), $array_length);
            let mut output = arr![$f ($array[0]); $array_length];
            let mut i = 0;

            while i < $array.len() {
                output[i] = $f ($array[i]);

                i += 1;
            };

            output
        }
    };
}

pub(crate) use map_array_const;

/// Returns the coordinates of the plane determined by a normal
pub fn slice_coords<T: Copy>(vector: Vector3<T>, normal: Axis) -> Vector2<T> {
    let (x, y) = match normal {
        Axis::X => (1, 2),
        Axis::Y => (0, 2),
        Axis::Z => (0, 1),
    };

    Vector2::new(vector[x], vector[y])
}
