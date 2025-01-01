use crate::cache::entry::{CacheKey, CacheValue};
use crate::cache::Cache;

impl<K: CacheKey, V: CacheValue> Cache<K, V> {
    pub fn get(&self, key: K) -> GetResult<K, V> {
        let hash_key = key.hash(self.lookup_table_capacity);

        let found = self.lookup_table[hash_key]
            .iter()
            .find(|x| unsafe { (***x).key == key })
            .map(|x| unsafe { (**x).value.clone() });

        if let Some(value) = found {
            GetResult::Found { key, value }
        } else {
            GetResult::NotFound { key }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GetResult<K: CacheKey, V: CacheValue> {
    Found { key: K, value: V },
    NotFound { key: K },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::get::GetResult::{Found, NotFound};

    #[test]
    fn cannot_find_an_unknown_key() {
        let cache = Cache::<usize, usize>::empty(1, 5).unwrap();
        assert_eq!(cache.get(1), NotFound { key: 1 });
    }

    #[test]
    fn can_find_known_keys() {
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
        // TODO: move this to a property test that simply sets and gets from an arbitrary cache
        // in arbitrary values, there should be one where a hashed key already exists

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
}
