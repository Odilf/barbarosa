use std::{fmt::Display, io, mem};

use chrono::{DateTime, Duration, Local};

use crate::{
    cube3::mus::{
        cache::disk_storage::{print_with_timestamp, write_partial},
        deindex::Deindexable,
        index::Indexable,
        Corners, HalfEdges,
    },
    search::successors,
};

use super::disk_storage::DiskCacheable;

pub use super::disk_storage::load_or_build;

// pub fn build() -> io::Result<Cache> {
//     let edges = build_partial::<HalfEdges>()?;
//     let corners = build_partial::<Corners>()?;

//     Ok(Cache { edges, corners })
// }

// TODO: REMOVE ALL THIS EWWWWW 🤮🤮
pub trait MovableTemp {
    fn successors(&self) -> Vec<(Self, i8)>
    where
        Self: Sized;
}

impl MovableTemp for HalfEdges {
    fn successors(&self) -> Vec<(Self, i8)> {
        successors(self)
    }
}

impl MovableTemp for Corners {
    fn successors(&self) -> Vec<(Self, i8)> {
        successors(self)
    }
}

fn cache_neighbours_at_depth<T: Indexable + Deindexable + MovableTemp + DiskCacheable>(
    cache: &mut Vec<PartialEntry>,
    move_depth: u8,
) {
    // TODO: Make paralell
    for index in 0..T::TOTAL_SET_SIZE {
        // Only cache neighbours of states that are at the current depth
        match cache[index].get() {
            Some(given) if given == move_depth - 1 => (),
            _ => continue,
        }

        let state = T::from_index(index);
        for (successor, edge_cost) in state.successors() {
            let successor_entry = &mut cache[successor.index()];

            if successor_entry.get().is_none() {
                debug_assert_eq!(edge_cost, 1);
                successor_entry.0 = move_depth;
            }
        }
    }
}

/// Builds and stores a partial cache
pub fn build_partial<T: Indexable + Deindexable + MovableTemp + DiskCacheable>(
) -> io::Result<Vec<u8>> {
    print_with_timestamp::<T>(&format!("Building partial cache for {}.", T::PATH));

    let mut cache = vec![PartialEntry::none(); T::TOTAL_SET_SIZE];

    // Map between numebr of moves and states that take that amount of moves to be reached
    // let mut move_depth = 0;
    let start_time = Local::now();

    // Start with the solved state
    cache[0] = PartialEntry(0);

    for move_depth in 1.. {
        let stats = Stats::new(&cache, start_time);

        if stats.amount_cached == T::TOTAL_SET_SIZE {
            print_with_timestamp::<T>(&format!("Finished caching! {stats}"));
            break;
        }

        print_with_timestamp::<T>(&format!("Caching at depth {move_depth}; {stats}"));

        cache_neighbours_at_depth::<T>(&mut cache, move_depth);
    }

    print_with_timestamp::<T>(&format!("Caching done! Attempting to save to disk."));

    let cache = unsafe { mem::transmute::<Vec<PartialEntry>, Vec<u8>>(cache) };

    write_partial::<T>(&cache)?;

    print_with_timestamp::<T>(&format!("Cache saved to disk."));

    Ok(cache)
}

#[derive(Debug, Clone)]
struct PartialEntry(u8);

impl PartialEntry {
    fn none() -> Self {
        Self(u8::MAX)
    }

    fn get(&self) -> Option<u8> {
        match self.0 {
            u8::MAX => None,
            value => Some(value),
        }
    }
}

fn amount_cached(cache: &Vec<PartialEntry>) -> usize {
    cache.iter().filter(|entry| entry.get().is_some()).count()
}

fn percent_cached(cache: &Vec<PartialEntry>, amount_cached: usize) -> f64 {
    amount_cached as f64 / cache.len() as f64
}

struct Stats {
    amount_cached: usize,
    percent_cached: f64,
    time_elapsed: Duration,
    states_per_second: f64,
    eta: Duration,
}

impl Stats {
    fn new(cache: &Vec<PartialEntry>, started: DateTime<Local>) -> Self {
        let amount_cached = amount_cached(cache);
        let percent_cached = percent_cached(cache, amount_cached);
        let time_elapsed = Local::now().signed_duration_since(started);
        let states_per_second =
            amount_cached as f64 / time_elapsed.num_milliseconds() as f64 * 1000.0;
        let eta = {
            let states_left = cache.len() - amount_cached;
            let time_remaining = states_left as f64 / states_per_second;
            Duration::seconds(time_remaining as i64)
        };

        Self {
            amount_cached,
            percent_cached,
            time_elapsed,
            states_per_second,
            eta,
        }
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{amount_cached: >8} states cached ({percent_cached:.2}%) in {time_elapsed}s at {states_per_second:.2} states/s (ETA: {eta:.2}m)",
            amount_cached = self.amount_cached,
            percent_cached = self.percent_cached * 100.0,
            time_elapsed = self.time_elapsed.num_seconds(),
            states_per_second = self.states_per_second,
            eta = self.eta.num_minutes(),
        )
    }
}
