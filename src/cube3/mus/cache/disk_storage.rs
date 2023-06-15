use std::{fs, io, path::Path};

use chrono::Local;

use crate::cube3::mus::{index::Indexable, Corners, HalfEdges, deindex::Deindexable};

use super::{generation::{build_partial, MovableTemp}, Cache};

pub fn load() -> io::Result<Cache> {
    let edges = load_partial::<HalfEdges>()?;
    let corners = load_partial::<Corners>()?;

    Ok(Cache { edges, corners })
}

pub fn load_or_build() -> io::Result<Cache> {
    let edges = load_or_build_partial::<HalfEdges>()?;
    let corners = load_or_build_partial::<Corners>()?;

    Ok(Cache { edges, corners })
}

fn load_or_build_partial<T: DiskCacheable + Deindexable + MovableTemp>() -> io::Result<Vec<u8>> {
    match load_partial::<T>() {
        Ok(cache) => Ok(cache),
        Err(err) if err.kind() == io::ErrorKind::NotFound => build_partial::<T>(),
        Err(err) => Err(err),
    }
}

pub trait DiskCacheable: Indexable {
    const PATH: &'static str;
}

impl DiskCacheable for Corners {
    const PATH: &'static str = "mus-cache/corners.barbarosa";
}

impl DiskCacheable for HalfEdges {
    const PATH: &'static str = "mus-cache/edges.barbarosa";
}

pub fn load_partial<T: DiskCacheable>() -> io::Result<Vec<u8>> {
    print_with_timestamp::<T>("Attempting to load cache from disk");

    let bytes = fs::read(T::PATH)?;

    assert_correct_cache_size::<T>(bytes.len());
    print_with_timestamp::<T>("Cache has been loaded");

    Ok(bytes)
}

// TODO: Would be nice if there was some metadata stored along the actual cache. Mainly the cube for which this
// cache was built, since the reference cube changes the indices. 
pub fn write_partial<T: DiskCacheable>(bytes: &[u8]) -> io::Result<()> {
    print_with_timestamp::<T>("Writing cache to disk");
    assert_correct_cache_size::<T>(bytes.len());

    match fs::write(T::PATH, bytes) {
        Ok(data) => data,
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            let Some(parent) = Path::new(T::PATH).parent() else {
                return Err(err);
            };

            fs::create_dir_all(parent)?;
            fs::write(T::PATH, bytes)?
        }
        Err(err) => return Err(err),
    };

    print_with_timestamp::<T>("Cache has been written");

    Ok(())
}

// TODO: Maybe only do this in debug mode?
pub fn print_with_timestamp<T: DiskCacheable>(msg: &str) {
    println!(
        "{}: {} ({})",
        Local::now().format("%H:%M:%S.%3f"),
        msg,
        T::PATH
    );
}

fn assert_correct_cache_size<T: DiskCacheable>(given: usize) {
    if given != T::TOTAL_SET_SIZE {
        panic!("Cache file seems to be corrupted. It should be {} bytes long, but it's {} bytes long (help: might be wise to delete `mus-cache/`)", T::TOTAL_SET_SIZE, given);
    }
}
