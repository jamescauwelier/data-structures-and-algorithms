use crate::cache::entry::{CacheKey, CacheValue};
use crate::cache::Cache;

impl<K: CacheKey, V: CacheValue> Cache<K, V> {
    pub fn drop(&mut self, key: K) -> DropResult<K> {
        match self.hashmap.get(key.clone()) {
            None => DropResult::KeyNotFound { key },
            Some(cache_entry_location) => {
                unsafe { self.entries.drop(cache_entry_location.location_ptr) };
                self.hashmap.drop(key.clone());

                DropResult::Dropped { key }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DropResult<K: CacheKey> {
    Dropped { key: K },
    KeyNotFound { key: K },
}

#[cfg(test)]
mod tests {
    use crate::cache::drop::DropResult;
    use crate::cache::drop::DropResult::KeyNotFound;
    use crate::cache::get::GetResult;
    use crate::cache::Cache;

    #[test]
    fn cannot_drop_a_missing_key() {
        let mut cache = Cache::<usize, usize>::empty(10, 500).unwrap();
        assert_eq!(cache.drop(1), KeyNotFound { key: 1 });
    }

    #[test]
    fn can_drop_a_known_key() {
        let mut cache = Cache::<usize, usize>::empty(10, 500).unwrap();
        cache.set(1, 123);
        assert_eq!(cache.drop(1), DropResult::Dropped { key: 1 });
    }

    #[test]
    fn a_dropped_key_cannot_be_get() {
        let mut cache = Cache::<usize, usize>::empty(10, 500).unwrap();
        cache.set(1, 123);
        cache.drop(1);
        assert_eq!(cache.get(1), GetResult::NotFound { key: 1 });
    }
}
