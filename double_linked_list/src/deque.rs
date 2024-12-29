pub mod item;

use crate::deque::item::Item;
use std::alloc::Layout;

pub struct Deque<T> {
    first: *mut Item<T>,
    last: *mut Item<T>,
    len: usize,
}

impl<T> Deque<T> {
    pub fn create() -> Deque<T> {
        Deque {
            first: std::ptr::null_mut(),
            last: std::ptr::null_mut(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn push_first(&mut self, x: T) {
        // function can only be used when there are no other elements in the deque
        assert!(self.first.is_null());
        assert!(self.last.is_null());

        let ptr = Item::create(x);
        self.first = ptr;
        self.last = ptr;
        self.len += 1;
    }

    pub fn push_left(&mut self, x: T) {
        if self.first.is_null() || self.last.is_null() {
            self.push_first(x);
        } else {
            // creates the new item
            let ptr = Item::create(x);
            let item = unsafe { &mut *ptr };
            let first = unsafe { &mut *self.first };

            // sets the correct links between this item and the previous outer left item (first)
            item.right = self.first;
            first.left = ptr;
            self.first = ptr;

            // updates inner state
            self.len += 1;
        }
    }

    pub fn push_right(&mut self, x: T) {
        if self.first.is_null() || self.last.is_null() {
            self.push_first(x);
        } else {
            // creates the new item
            let ptr = Item::create(x);
            let item = unsafe { &mut *ptr };
            let last = unsafe { &mut *self.last };

            // sets the correct links between this item and the previous outer right item (last)
            item.left = self.last;
            last.right = ptr;
            self.last = ptr;

            // updates inner state
            self.len += 1;
        }
    }

    pub fn first(&self) -> Option<&Item<T>> {
        if self.first.is_null() {
            None
        } else {
            unsafe { Some(&(*self.first)) }
        }
    }

    pub fn last(&self) -> Option<&Item<T>> {
        if self.last.is_null() {
            None
        } else {
            unsafe { Some(&(*self.last)) }
        }
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        // checks if there are any elements to drop
        if !self.first.is_null() {
            let mut nxt = self.first;

            // iterates through the list using the pointers in the
            // `right` property of each item, dropping each item
            // in the process
            while !nxt.is_null() {
                let ptr = nxt;
                let item = unsafe { &*nxt };
                nxt = item.right;

                unsafe {
                    std::alloc::dealloc(ptr as *mut u8, Layout::new::<Item<T>>());
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_left_once() {
        let mut data = Deque::create();
        data.push_left(1);

        unsafe {
            assert_eq!(1, (*data.first).value);
            assert_eq!(1, (*data.last).value);
        }
    }
}
