use crate::cache::entry::{CacheEntry, CacheKey, CacheValue};
use std::alloc::Layout;

type CacheEntryPtr<K: CacheKey, V: CacheValue> = *mut CacheEntry<K, V>;

pub(in crate::cache) struct CacheEntries<K: CacheKey, V: CacheValue> {
    first: *mut CacheEntry<K, V>,
    last: *mut CacheEntry<K, V>,
    len: usize,
    capacity: usize,
}

impl<K: CacheKey, V: CacheValue> CacheEntries<K, V> {
    /// Creates an empty CacheEntries doubly linked list
    pub(in crate::cache) fn empty(capacity: usize) -> CacheEntries<K, V> {
        CacheEntries {
            len: 0,
            capacity,
            first: std::ptr::null_mut(),
            last: std::ptr::null_mut(),
        }
    }

    /// Appends a new entry to the collection
    pub(in crate::cache) unsafe fn append(&mut self, key: K, value: V) -> CacheEntryPtr<K, V> {
        // make sure the capacity is respected
        while self.len >= self.capacity {
            self.reduce_by_one();
        }

        // creates the new entry
        let mut entry_ptr = CacheEntry::create(key, value);

        if !self.last.is_null() {
            // update the pointer of the second to last element to point to the new last element
            let second_to_last = &mut *self.last;
            second_to_last.right = entry_ptr;

            // update the new entry to point to the second to last element
            let last = &mut *entry_ptr;
            last.left = second_to_last as *mut CacheEntry<K, V>;

            // update the reference to the last element
            self.last = entry_ptr;
        } else {
            // if there is no last element, it is the only element
            self.first = entry_ptr;
            self.last = entry_ptr;
        }

        // update how much of the capacity has been consumed
        self.len += 1;

        entry_ptr
    }

    /// drops the entry with the given raw pointer
    pub(in crate::cache) unsafe fn drop(
        &mut self,
        ptr: CacheEntryPtr<K, V>,
    ) -> Option<DroppedEntry<K, V>> {
        if ptr.is_null() {
            None
        } else {
            let entry = &*ptr;
            let key = entry.key.clone();
            let value = entry.value.clone();
            let left_of_dropped_ptr = entry.left;
            let right_of_dropped_ptr = entry.right;

            // link left item to right
            if !left_of_dropped_ptr.is_null() {
                let left = &mut *left_of_dropped_ptr;
                left.right = right_of_dropped_ptr;
            }

            // link right item to left
            if !right_of_dropped_ptr.is_null() {
                let right = &mut *right_of_dropped_ptr;
                right.left = left_of_dropped_ptr;
            }

            // update pointers to first and last
            match (
                !left_of_dropped_ptr.is_null(),
                !right_of_dropped_ptr.is_null(),
            ) {
                (true, true) => {
                    // first and last pointers need no updates
                }
                (true, false) => {
                    // there is no right pointers, so we are last, and need to update last
                    self.last = left_of_dropped_ptr;
                }
                (false, true) => {
                    // there is no left pointer, so we are first, and need to update first
                    self.first = right_of_dropped_ptr;
                }
                (false, false) => {
                    // there is no left or right, so it is the only item
                    // therefore, we set the first and last to null
                    self.first = std::ptr::null_mut();
                    self.last = std::ptr::null_mut();
                }
            }

            // deallocate the memory
            std::alloc::dealloc(ptr as *mut u8, Layout::new::<CacheEntry<K, V>>());

            // adjust the used length of the data structure
            self.len -= 1;

            Some(DroppedEntry {
                key,
                value,
                left_of_dropped_ptr,
                right_of_dropped_ptr,
            })
        }
    }

    /// read an entry
    pub(in crate::cache) unsafe fn read(&mut self, ptr: CacheEntryPtr<K, V>) -> V {
        let entry = &*ptr;
        let key = entry.key.clone();
        let value = entry.value.clone();

        // first drop it, to remove its current positioning
        let _ = self.drop(ptr);

        // now re-append to have it take priority in clearing order (in that it gets cleared last)
        let _ = self.append(key, value.clone());

        value
    }

    /// should reduce the capacity by one depending on what is recently used
    fn reduce_by_one(&mut self) -> Option<K> {
        match unsafe { self.drop(self.first) } {
            None => {
                // nothing here to reduce
                None
            }
            Some(details) => {
                self.first = details.right_of_dropped_ptr;

                Some(details.key)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DroppedEntry<K: CacheKey, V: CacheValue> {
    key: K,
    value: V,
    left_of_dropped_ptr: *mut CacheEntry<K, V>,
    right_of_dropped_ptr: *mut CacheEntry<K, V>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_instance_is_empty() {
        let instance = CacheEntries::<usize, usize>::empty(10);
        assert_eq!(instance.len, 0);
    }

    #[test]
    fn an_entry_can_be_appended() {
        let mut instance = CacheEntries::<usize, usize>::empty(10);
        unsafe {
            let ptr = instance.append(1, 1);
            assert!(!ptr.is_null());
        }
        assert_eq!(instance.len, 1);
    }

    #[test]
    fn an_appended_entry_can_be_read() {
        let mut instance = CacheEntries::<usize, usize>::empty(10);
        unsafe {
            let ptr = instance.append(1, 1);
            let read_value = instance.read(ptr);
            assert_eq!(read_value, 1);
        }
    }

    #[test]
    fn an_appended_entry_is_linked_to_the_second_to_last_one() {
        let mut instance = CacheEntries::<usize, usize>::empty(10);
        unsafe {
            let first_ptr = instance.append(1, 1);
            let second_ptr = instance.append(1, 2);

            let first_entry = &*first_ptr;
            let second_entry = &*second_ptr;

            assert_eq!(second_entry.left, first_ptr);
            assert_eq!(first_entry.right, second_ptr);
        }
    }

    #[test]
    fn appended_entries_can_be_dropped() {
        let mut instance = CacheEntries::<usize, usize>::empty(10);
        unsafe {
            let first_ptr = instance.append(1, 1);
            let second_ptr = instance.append(1, 2);

            assert_eq!(instance.len, 2);
            assert_eq!(
                instance.drop(second_ptr),
                Some(DroppedEntry {
                    key: 1,
                    value: 2,
                    left_of_dropped_ptr: first_ptr,
                    right_of_dropped_ptr: std::ptr::null_mut()
                })
            );
            assert_eq!(instance.len, 1);
        }
    }

    #[test]
    fn the_least_recently_appended_items_are_cleared() {
        let mut instance = CacheEntries::<usize, usize>::empty(3);
        unsafe {
            // all the same key, values because key uniqueness is not guaranteed in this struct
            let _ = instance.append(1, 11);
            let _ = instance.append(2, 22);
            let _ = instance.append(3, 33);
            let _ = instance.append(4, 44);

            assert_eq!(instance.len, 3);

            let first = &*instance.first;
            assert_eq!(first.key, 2);
            assert_eq!(first.value, 22);
        }
    }

    #[test]
    fn reading_prevents_an_item_from_being_cleared() {
        let mut instance = CacheEntries::<usize, usize>::empty(3);
        unsafe {
            // all the same key, values because key uniqueness is not guaranteed in this struct
            let ptr_1 = instance.append(1, 11);
            let _ = instance.append(2, 22);
            let _ = instance.append(3, 33);
            let value_1 = instance.read(ptr_1);

            // now the last item should be the first one that was appended
            let last = unsafe { &*instance.last };
            assert_eq!(last.key, 1);
            assert_eq!(last.value, 11);

            let _ = instance.append(4, 44);

            assert_eq!(instance.len, 3);

            // now the first element is 3 - 33, because 2 - 22 got cleared
            let first = &*instance.first;
            assert_eq!(first.key, 3);
            assert_eq!(first.value, 33);
        }
    }
}
