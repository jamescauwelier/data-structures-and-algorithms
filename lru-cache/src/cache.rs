pub mod hasher;

use crate::cache::DeleteResult::KeyNotFound;
use crate::cache::GetResult::{Found, NotFound};
use hasher::Hasher;
use std::marker::PhantomData;

pub trait CacheKey: Hasher + Clone + PartialEq {}
impl<T: Hasher + Clone + PartialEq> CacheKey for T {}

pub trait CacheValue: Clone {}
impl<T: Clone> CacheValue for T {}

#[derive(Clone, Debug)]
struct CacheEntry<K: CacheKey, V: CacheValue> {
    key: K,
    value: V,
}

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
pub struct Cache<K: CacheKey, V: CacheValue> {
    capacity: usize,
    lookup_table_capacity: usize,
    lookup_table: Vec<Vec<CacheEntry<K, V>>>,
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

    pub fn set(&mut self, key: K, value: V) {
        let hash_key = key.hash(self.lookup_table_capacity);
        self.lookup_table[hash_key].push(CacheEntry { key, value });
    }

    pub fn get(&self, key: K) -> GetResult<K, V> {
        let hash_key = key.hash(self.lookup_table_capacity);

        let found = self.lookup_table[hash_key].iter().find(|x| x.key == key);

        if let Some(entry) = found {
            Found {
                key,
                value: entry.value.clone(),
            }
        } else {
            NotFound { key }
        }
    }

    pub fn delete(&self, key: K) -> DeleteResult<K> {
        KeyNotFound { key }
    }
}

#[derive(Debug, PartialEq)]
pub enum GetResult<K: CacheKey, V: CacheValue> {
    Found { key: K, value: V },
    NotFound { key: K },
}

#[derive(Debug, PartialEq)]
pub enum DeleteResult<K: CacheKey> {
    Deleted { key: K },
    KeyNotFound { key: K },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::GetResult::Found;

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
    fn get_a_non_existing_key() {
        let cache = Cache::<usize, usize>::empty(1, 5).unwrap();
        assert_eq!(cache.get(1), NotFound { key: 1 });
    }

    #[test]
    fn get_existing_keys() {
        let mut cache = Cache::<usize, usize>::empty(100, 100).unwrap();
        cache.set(1, 11);
        cache.set(2, 22);
        cache.set(3, 33);
        cache.set(4, 44);

        assert_eq!(cache.get(1), Found { key: 1, value: 11 });
        assert_eq!(cache.get(2), Found { key: 2, value: 22 });
        assert_eq!(cache.get(3), Found { key: 3, value: 33 });
        assert_eq!(cache.get(4), Found { key: 4, value: 44 });
    }

    #[test]
    fn handles_hash_collisions() {
        let mut cache = Cache::<usize, usize>::empty(100, 3).unwrap();
        cache.set(1, 1);
        cache.set(2, 2);
        cache.set(3, 3);
        cache.set(4, 4);

        // for a lookup table size 3, there is a hash key collision --> 1 % 3 == 4 % 3
        // both key / value pairs should still be accessible, since the capacity of 100 is not met

        assert_eq!(cache.get(1), Found { key: 1, value: 1 });
        assert_eq!(cache.get(4), Found { key: 4, value: 4 });
    }

    #[test]
    fn overwrite_an_existing_key() {
        unimplemented!()
    }

    #[test]
    fn get_a_cleared_key() {
        unimplemented!()
    }

    #[test]
    fn delete_a_key() {
        unimplemented!()
    }
}
