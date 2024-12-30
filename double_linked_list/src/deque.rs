pub mod clone;
mod eq;
pub mod item;
mod iterator;
pub mod macros;

use crate::deque::item::Item;
use std::alloc::Layout;

#[derive(Debug)]
pub struct Deque<T>
where
    T: PartialEq,
{
    first: *mut Item<T>,
    last: *mut Item<T>,
    len: usize,
}

impl<T> Deque<T>
where
    T: PartialEq,
{
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

    fn push_first(&mut self, x: T) -> &Item<T> {
        // function can only be used when there are no other elements in the deque
        assert!(self.first.is_null());
        assert!(self.last.is_null());

        let ptr = Item::create(x);
        self.first = ptr;
        self.last = ptr;
        self.len += 1;

        unsafe { &*ptr }
    }

    pub fn push_left(&mut self, x: T) -> &Item<T> {
        if self.first.is_null() || self.last.is_null() {
            self.push_first(x)
        } else {
            // creates the new item
            let ptr = Item::create(x);
            let item = unsafe { &mut *ptr };
            let first = unsafe { &mut *self.first };

            // sets the correct links between this item and the previous outer left item (first)
            item.right_ptr = self.first;
            first.left_ptr = ptr;
            self.first = ptr;

            // updates inner state
            self.len += 1;

            unsafe { &*ptr }
        }
    }

    pub fn push_right(&mut self, x: T) -> &Item<T> {
        if self.first.is_null() || self.last.is_null() {
            self.push_first(x)
        } else {
            // creates the new item
            let ptr = Item::create(x);
            let item = unsafe { &mut *ptr };
            let last = unsafe { &mut *self.last };

            // sets the correct links between this item and the previous outer right item (last)
            item.left_ptr = self.last;
            last.right_ptr = ptr;
            self.last = ptr;

            // updates inner state
            self.len += 1;

            unsafe { &*ptr }
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

impl<T> Drop for Deque<T>
where
    T: PartialEq,
{
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
                nxt = item.right_ptr;

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
    use proptest::prelude::*;

    impl Arbitrary for Deque<usize> {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            let data = Deque::create();

            Just(data).boxed()
        }

        type Strategy = BoxedStrategy<Deque<usize>>;
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn an_item_pushed_left_becomes_the_first_item(mut data: Deque<usize>, value: usize) {
            data.push_left(value);

            assert_eq!(data.first().unwrap().value(), &value);
        }
    }

    #[test]
    fn push_left_once() {
        let mut data = Deque::create();
        let it = data.push_left(1);

        assert_eq!(it.value(), &1);

        unsafe {
            assert_eq!(1, (*data.first).value);
            assert_eq!(1, (*data.last).value);
        }
    }
}
