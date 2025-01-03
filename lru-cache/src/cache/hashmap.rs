use crate::cache::entry::{CacheKey, CacheValue};
use crate::cache::entry_location::CacheEntryLocation;

const CAPACITY: usize = 1000;

#[derive(Debug)]
pub(in crate::cache) struct HashMap<K: CacheKey, V: CacheValue> {
    data: Vec<Vec<CacheEntryLocation<K, V>>>,
}

impl<K: CacheKey, V: CacheValue> HashMap<K, V> {
    pub(in crate::cache) fn create() -> HashMap<K, V> {
        let mut data = vec![vec![]; CAPACITY];
        HashMap { data }
    }

    pub(in crate::cache) fn get(&self, key: K) -> Option<&CacheEntryLocation<K, V>> {
        self.data[key.hash(CAPACITY)].iter().find(|x| x.key == key)
    }

    pub(in crate::cache) fn set(
        &mut self,
        key: K,
        value: CacheEntryLocation<K, V>,
    ) -> HashMapSet<K, V> {
        let hash = key.hash(CAPACITY);
        let mut locations = unsafe { self.data.get_unchecked_mut(hash) };
        let old_location = locations
            .iter()
            .find(|x| x.key == key.clone())
            .map(|x| x.clone());
        match old_location.clone() {
            None => {
                locations.push(value.clone());
                HashMapSet::KeySetAsNew { key, value }
            }
            Some(overwritten) => {
                locations.retain(|x| x.key != key.clone());
                locations.push(value.clone());

                HashMapSet::KeyOverwritten {
                    key,
                    value,
                    overwritten: overwritten.clone(),
                }
            }
        }
    }

    pub(in crate::cache) fn drop(&mut self, key: K) -> HashMapDrop<K, V> {
        let hash = key.hash(CAPACITY);
        let mut locations = &mut self.data[hash];
        let item = locations.iter().find(|x| x.key == key).map(|x| x.clone());
        locations.retain(|x| x.key != key);
        match item {
            None => HashMapDrop::KeyNotFound { key },
            Some(value) => HashMapDrop::KeyDropped {
                key,
                value: value.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::cache) enum HashMapSet<K: CacheKey, V: CacheValue> {
    KeySetAsNew {
        key: K,
        value: CacheEntryLocation<K, V>,
    },
    KeyOverwritten {
        key: K,
        value: CacheEntryLocation<K, V>,
        overwritten: CacheEntryLocation<K, V>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::cache) enum HashMapDrop<K: CacheKey, V: CacheValue> {
    KeyDropped {
        key: K,
        value: CacheEntryLocation<K, V>,
    },
    KeyNotFound {
        key: K,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creat_a_new_hashmap() {
        let a = HashMap::<usize, usize>::create();
    }

    #[test]
    fn a_non_existing_key_cannot_be_found() {
        let a = HashMap::<usize, usize>::create();
        assert_eq!(a.get(1), None);
    }

    #[test]
    fn a_non_existing_key_can_be_set() {
        let mut a = HashMap::<usize, usize>::create();
        let location = CacheEntryLocation::create(1, std::ptr::null_mut());
        let set_result = a.set(1, location.clone());
        assert_eq!(
            set_result,
            HashMapSet::KeySetAsNew {
                key: 1,
                value: location
            }
        );
    }

    #[test]
    fn an_existing_key_can_be_overwritten() {
        let mut a = HashMap::<usize, usize>::create();
        let location = CacheEntryLocation::create(1, std::ptr::null_mut());
        let _ = a.set(1, location.clone());
        let set_result = a.set(1, location.clone());
        assert_eq!(
            set_result,
            HashMapSet::KeyOverwritten {
                key: 1,
                value: location.clone(),
                overwritten: location,
            }
        );
    }

    #[test]
    fn we_can_get_a_key_after_setting_it() {
        let mut a = HashMap::<usize, usize>::create();
        let location = CacheEntryLocation::create(1, std::ptr::null_mut());
        let _ = a.set(1, location.clone());
        assert_eq!(a.get(1), Some(&location));
    }

    #[test]
    fn we_can_drop_an_existing_key() {
        let mut a = HashMap::<usize, usize>::create();
        let mut location = CacheEntryLocation::create(1, std::ptr::null_mut());
        a.set(1, location.clone());
        assert_eq!(
            a.drop(1),
            HashMapDrop::KeyDropped {
                key: 1,
                value: location
            }
        );
    }

    #[test]
    fn we_cannot_drop_a_non_existing_key() {
        let mut a = HashMap::<usize, usize>::create();
        let mut location: CacheEntryLocation<usize, usize> =
            CacheEntryLocation::create(1, std::ptr::null_mut());
        assert_eq!(a.drop(1), HashMapDrop::KeyNotFound { key: 1 });
    }
}
