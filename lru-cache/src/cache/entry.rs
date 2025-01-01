use crate::cache::hasher::Hasher;
use std::alloc::{handle_alloc_error, Layout};

pub trait CacheKey: Hasher + Clone + PartialEq {}
impl<T: Hasher + Clone + PartialEq> CacheKey for T {}

pub trait CacheValue: Clone {}
impl<T: Clone> CacheValue for T {}

#[derive(Clone, Debug)]
pub(in crate::cache) struct CacheEntry<K: CacheKey, V: CacheValue> {
    pub(in crate::cache) key: K,
    pub(in crate::cache) value: V,
}

impl<K: CacheKey, V: CacheValue> CacheEntry<K, V> {
    pub(in crate::cache) fn create(key: K, value: V) -> *mut CacheEntry<K, V> {
        unsafe {
            let layout = Layout::new::<CacheEntry<K, V>>();
            let ptr = std::alloc::alloc(layout) as *mut CacheEntry<K, V>;
            if ptr.is_null() {
                handle_alloc_error(layout);
            }

            ptr
        }
    }
}
