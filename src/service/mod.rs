//! Service layer for caching and transforming data

use std::collections::HashMap;
use std::hash::Hash;

pub mod advertisement;
pub mod container;
pub mod image;
pub mod video;

fn group<I, K, V>(iter: I) -> HashMap<K, Vec<V>>
where
    K: Eq + Hash,
    I: Iterator<Item = (K, V)>,
{
    let mut hash_map: HashMap<K, Vec<V>> = match iter.size_hint() {
        (_, Some(len)) => HashMap::with_capacity(len),
        (len, None) => HashMap::with_capacity(len),
    };

    for (key, value) in iter {
        hash_map
            .entry(key)
            .or_insert_with(|| Vec::with_capacity(1))
            .push(value)
    }

    hash_map
}
