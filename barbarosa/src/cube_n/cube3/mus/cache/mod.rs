//! Caching for MUS.
//!
//! Currently we can only cache MUS if we have filesystem access. This should be changed
//! to somehow work on the web with WASM and such.

mod disk_storage;
mod generation;
mod test;

use std::{io, sync::OnceLock};

use crate::cube3::Cube3;

/// The MUS cache. Any instance of this type is guaranteed to have a complete cache
pub struct Cache {
    edges: Vec<u8>,
    corners: Vec<u8>,
}

static CACHE_LOCK: OnceLock<Cache> = OnceLock::new();

/// Gets the heuristic value of the cube from the cache, or initializes the cache if it hasn't been
/// initialized yet
pub fn get_or_init(cube: &Cube3) -> u8 {
    CACHE_LOCK.get_or_init(Cache::init).get(cube)
}

/// Gets the heuristic value of the cube from the cache, or returns `None` if the cache hasn't been
/// initialized yet
pub fn get(cube: &Cube3) -> Option<u8> {
    CACHE_LOCK.get().map(|cache| cache.get(cube))
}

impl Cache {
    /// Gets the heuristic value of the cube from the cache. This is the maximum
    /// of the heuristic values of the corners and the two edge sets
    pub fn get(&self, cube: &Cube3) -> u8 {
        let indices = cube.indices();

        let corner_heuristic = unsafe { self.corners.get_unchecked(indices.corners) };
        let edge_heuristic = *indices
            .edges
            .map(|i| unsafe { self.edges.get_unchecked(i) })
            .iter()
            .max()
            .unwrap();

        *corner_heuristic.max(edge_heuristic)
    }

    /// Initializes the cache.
    ///
    /// # Panic
    ///
    /// This panics if there is something wrong with reading or writing a file.
    ///
    /// This would probably only happen if the user doesn't have permission to write to the cache directory.
    /// If there's a folder that needs to be created in the path it does, and is in general somewhat robust
    /// already by itself.
    pub fn init() -> Self {
        generation::load_or_build().expect(
            "User should have permission to write to cache directory. \
            If you have permissions and this still failed, it should probably \
            be reported as a bug.",
        )
    }

    /// Loads the cache from disk (doesn't build it if it doesn't exist)
    pub fn load() -> io::Result<Self> {
        disk_storage::load()
    }
}
