use crate::cube3::Corner;

use super::index::Indexable;

pub trait Deindexable: Indexable {
	fn deindex(index: usize) -> Self;
}

impl Deindexable for Corner {
    fn deindex(index: usize) -> Self {
        todo!()
    }
}
