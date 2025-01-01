use crate::cache::entry::{CacheEntry, CacheKey, CacheValue};
use crate::cache::Cache;

impl<K: CacheKey, V: CacheValue> Cache<K, V> {
    pub fn set(&mut self, key: K, value: V) {
        let hash_key = key.hash(self.lookup_table_capacity);
        self.lookup_table[hash_key].push(CacheEntry::create(key, value));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn an_unkown_key_can_be_set() {
        unimplemented!()
    }

    #[test]
    fn a_known_key_can_be_overriden() {
        unimplemented!()
    }
}
