use crate::cache::entry::{CacheEntry, CacheKey, CacheValue};

#[derive(Debug, Clone, PartialEq)]
pub(in crate::cache) struct CacheEntryLocation<K: CacheKey, V: CacheValue> {
    pub(in crate::cache) key: K,
    pub(in crate::cache) location_ptr: *mut CacheEntry<K, V>,
}

impl<K: CacheKey, V: CacheValue> CacheEntryLocation<K, V> {
    pub(in crate::cache) fn create(
        key: K,
        location_ptr: *mut CacheEntry<K, V>,
    ) -> CacheEntryLocation<K, V> {
        CacheEntryLocation { key, location_ptr }
    }
}
