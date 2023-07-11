use crate::cube3::{mus, Cube3};

/// Same as [mus()], but with a fallback function that is called if the cache has not been initialized.
pub fn mus_with_fallback(fallback: impl Fn(&Cube3) -> f32) -> impl Fn(&Cube3) -> f32 {
    move |cube| {
        mus::cache::get(cube)
            .map(|x| x as f32)
            .unwrap_or_else(|| fallback(cube))
    }
}

/// Returns the maximum of the number of moves it takes to
/// solve the corners and the two sets of 6 edges.
///
/// See also [crate::cube3::mus].
pub fn mus(cube: &Cube3) -> f32 {
    mus::cache::get_or_init(cube) as f32
}
