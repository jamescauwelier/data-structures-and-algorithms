#[cfg(test)]
pub(in crate::cache) mod arbitrary_cache;
pub mod drop;
pub mod entry;
pub mod get;
pub mod hasher;
pub mod set;

use crate::cache::entry::{CacheEntry, CacheKey, CacheValue};
use get::GetResult::{Found, NotFound};
use hasher::Hasher;
use std::marker::PhantomData;

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
    lookup_table: Vec<Vec<*mut CacheEntry<K, V>>>,
    _keys: PhantomData<K>,
}

impl<K: CacheKey, V: CacheValue> Cache<K, V> {
    pub fn empty(capacity: usize, lookup_table_capacity: usize) -> Result<Cache<K, V>, String> {
        if capacity > 0 {
            Ok(Cache {
                capacity,
                lookup_table_capacity,
                _keys: PhantomData::default(),
                lookup_table: vec![vec![]; lookup_table_capacity],
            })
        } else {
            Err("Cannot make a cache of capacity 0".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::get::GetResult::Found;

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

    #[test]
    fn overwrite_an_existing_key() {
        let mut cache = Cache::<usize, usize>::empty(3, 3).unwrap();
        cache.set(1, 1);
        cache.set(2, 2);
        cache.set(3, 3);
        cache.set(4, 4);

        // since the capacity is 3, the first element would've been cleared now

        assert_eq!(cache.get(1), NotFound { key: 1 });
        assert_eq!(cache.get(2), Found { key: 2, value: 2 });
    }
}
