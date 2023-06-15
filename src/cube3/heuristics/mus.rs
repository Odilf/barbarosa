use crate::cube3::{mus, Cube};

pub fn mus_with_fallback(fallback: impl Fn(&Cube) -> i8) -> impl Fn(&Cube) -> i8 {
    move |cube| {
        mus::cache::get(cube)
            .map(|x| x as i8)
            .unwrap_or_else(|| fallback(cube))
    }
}

pub fn mus(cube: &Cube) -> i8 {
    mus::cache::get_or_init(cube) as i8
}
