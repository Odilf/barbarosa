mod disk_storage;
mod generation;

use std::sync::OnceLock;

use crate::cube3::Cube;
pub struct Cache {
    edges: Vec<u8>,
    corners: Vec<u8>,
}

static CACHE_LOCK: OnceLock<Cache> = OnceLock::new();

pub fn get_or_init(cube: &Cube) -> u8 {
    CACHE_LOCK.get_or_init(|| Cache::init()).get(cube)
}

pub fn get(cube: &Cube) -> Option<u8> {
    CACHE_LOCK.get().map(|cache| cache.get(cube))
}

impl Cache {
    pub fn get(&self, cube: &Cube) -> u8 {
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

    pub fn init() -> Self {
        disk_storage::load_or_build().unwrap()
    }
}
