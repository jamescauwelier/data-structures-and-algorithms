#[cfg(test)]
pub(in crate::cache) mod arbitrary_cache;
pub mod drop;
mod entries;
pub mod entry;
mod entry_location;
pub mod get;
pub mod hasher;
mod hashmap;
pub mod set;

use crate::cache::entries::CacheEntries;
use crate::cache::entry::{CacheKey, CacheValue};
use crate::cache::hashmap::HashMap;
use hasher::Hasher;

/// # Cache (LRU)
///
/// An LRU cache is a data structure that stores values by
/// their respective key. Retrieval is possible by key in O(1)
/// after having added the key-value in O(1).
///
/// An additional feature is that the cache should be limited
/// in capacity. When the capacity is exceeded, the least
/// recently used key and associated value should be
/// cleared from the cache, again without adding to the
/// time complexity.
///
#[derive(Debug)]
pub struct Cache<K: CacheKey, V: CacheValue> {
    capacity: usize,
    lookup_table_capacity: usize,
    hashmap: HashMap<K, V>,
    entries: CacheEntries<K, V>,
}

impl<K: CacheKey, V: CacheValue> Cache<K, V> {
    pub fn empty(capacity: usize, lookup_table_capacity: usize) -> Result<Cache<K, V>, String> {
        if capacity > 0 {
            Ok(Cache {
                capacity,
                lookup_table_capacity,
                entries: CacheEntries::empty(capacity),
                hashmap: HashMap::create(),
            })
        } else {
            Err("Cannot make a cache of capacity 0".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_cache_without_capacity() {
        match Cache::<usize, usize>::empty(0, 5) {
            Err(_) => {}
            Ok(_) => panic!("We shouldn't be able to make a cache without capacity"),
        }
    }

    #[test]
    fn create_an_empty_cache() {
        let cache = Cache::<usize, usize>::empty(1, 5);
        assert!(cache.is_ok());
    }
}
