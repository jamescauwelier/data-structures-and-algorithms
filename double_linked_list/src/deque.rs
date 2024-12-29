pub mod item;

use std::ptr;
use crate::deque::item::Item;

pub struct Deque<T> {
    first: *mut Item<T>,
    last: *mut Item<T>,
    len: usize
}

impl <T> Deque<T> {
    pub fn create() -> Deque<T> {
        Deque {
            first: std::ptr::null_mut(),
            last: std::ptr::null_mut(),
            len: 0
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_left(&mut self, x: T) {
        if self.first.is_null() {
            assert!(self.last.is_null(), "If there is no first element, there can also not be a last one");
            let item = Item::create(x);

            let layout = std::alloc::Layout::new::<Item<T>>();
            let ptr = unsafe {std::alloc::alloc(layout) as *mut Item<T>};
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }

            unsafe {
                ptr::write(ptr, item);
            }

            self.first = ptr;
            self.last = ptr;
            self.len += 1;
        }
    }

    pub fn first(&self) -> Option<&Item<T>> {
        if self.first.is_null() {
            None
        } else {
            unsafe {
                let a = &(* self.first);
                Some(&(* self.first))
            }
        }
    }

    pub fn last(&self) -> Option<&Item<T>> {
        if self.last.is_null() {
            None
        } else {
            unsafe {
                Some(&(* self.last))
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