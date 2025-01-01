use crate::cache::entry::{CacheKey, CacheValue};
use crate::cache::get::GetResult;
use crate::cache::Cache;

impl<K: CacheKey, V: CacheValue> Cache<K, V> {
    pub fn drop(&self, key: K) -> DropResult<K> {
        match self.get(key) {
            GetResult::NotFound { key } => DropResult::KeyNotFound { key },
            GetResult::Found { key, .. } => DropResult::Dropped { key },
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
    use crate::cache::Cache;

    #[test]
    fn cannot_drop_a_missing_key() {
        let cache = Cache::<usize, usize>::empty(10, 500).unwrap();
        assert_eq!(cache.drop(1), KeyNotFound { key: 1 });
    }

    #[test]
    fn can_drop_a_known_key() {
        let mut cache = Cache::<usize, usize>::empty(10, 500).unwrap();
        cache.set(1, 123);
        assert_eq!(cache.drop(1), DropResult::Dropped { key: 1 });
    }
}
