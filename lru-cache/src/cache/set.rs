use crate::cache::entry::{CacheKey, CacheValue};
use crate::cache::entry_location::CacheEntryLocation;
use crate::cache::hashmap::HashMapSet;
use crate::cache::Cache;

impl<K: CacheKey, V: CacheValue> Cache<K, V> {
    pub fn set(&mut self, key: K, value: V) {

        //
        self.

        // do the actual setting
        let entry_ptr = unsafe { self.entries.append(key.clone(), value.clone()) };
        let new_location = CacheEntryLocation::create(key.clone(), entry_ptr);
        match self.hashmap.set(key, new_location) {
            HashMapSet::KeySetAsNew { .. } => {
                // nothing to do, location saved already in hashmap
            }
            HashMapSet::KeyOverwritten { overwritten, .. } => {
                unsafe { self.entries.drop(overwritten.location_ptr) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::get::GetResult::{Found, NotFound};
    use crate::cache::Cache;

    #[test]
    fn an_unkown_key_can_be_set() {
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
    fn a_known_key_can_be_overriden() {
        let mut cache = Cache::<usize, usize>::empty(100, 100).unwrap();
        cache.set(1, 11);
        cache.set(1, 22);

        assert_eq!(cache.get(1), Found { key: 1, value: 22 });
    }

    #[test]
    fn the_capacity_of_the_cache_is_limited() {
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
