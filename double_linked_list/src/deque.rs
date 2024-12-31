pub mod clone_it;
mod eq;
pub mod item;
mod iterator;
pub mod macros;

use crate::deque::item::Item;
use std::alloc::Layout;

pub trait DequeTypeRequirements: PartialEq + Clone {}
impl<T: PartialEq + Clone> DequeTypeRequirements for T {}

#[derive(Debug)]
pub struct Deque<T: DequeTypeRequirements> {
    first: *mut Item<T>,
    last: *mut Item<T>,
    len: usize,
}

impl<T: DequeTypeRequirements> Deque<T> {
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

    pub fn pop_left(&mut self) -> Option<T> {
        if let Some(first) = self.first() {
            let ptr = self.first;
            let deleted = Some((*first.value()).clone());
            if first.right_ptr.is_null() {
                // there is only one element, so the deque is now empty
                self.first = std::ptr::null_mut();
                self.last = std::ptr::null_mut();
            } else {
                // if the first item points to a second, the second needs to update its left pointer
                let second_ptr = first.right_ptr;
                let second = unsafe { &mut *second_ptr };
                second.left_ptr = std::ptr::null_mut();

                // the second item now also becomes the first
                self.first = second_ptr;
            }

            // the first item needs to be deallocated
            unsafe {
                std::alloc::dealloc(ptr as *mut u8, Layout::new::<Item<T>>());
            }

            // adjust the len
            self.len -= 1;

            deleted
        } else {
            None
        }
    }

    pub fn pop_right(&mut self) -> Option<T> {
        if let Some(last) = self.last() {
            let ptr = self.last;
            let deleted = Some((*last.value()).clone());
            if last.left_ptr.is_null() {
                // there is only one element, so the deque is now empty
                self.first = std::ptr::null_mut();
                self.last = std::ptr::null_mut();
            } else {
                // if the last item points to a second to last, the second to last needs to update its right pointer to null
                let previous_ptr = last.left_ptr;
                let second_to_last = unsafe { &mut *previous_ptr };
                second_to_last.right_ptr = std::ptr::null_mut();

                // the second to last item now also becomes the last
                self.last = previous_ptr;
            }

            // the first item needs to be deallocated
            unsafe {
                std::alloc::dealloc(ptr as *mut u8, Layout::new::<Item<T>>());
            }

            // adjust the len
            self.len -= 1;

            deleted
        } else {
            None
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

impl<T: DequeTypeRequirements> Drop for Deque<T> {
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
pub(crate) mod arbitrary_deque {
    use crate::deque::Deque;
    use proptest::arbitrary::{any, Arbitrary};
    use proptest::collection::vec;
    use proptest::prelude::{BoxedStrategy, Just, Strategy};
    use proptest::prop_oneof;
    use std::fmt::Debug;

    pub(crate) trait TestRequirements:
        PartialEq + Clone + Debug + Arbitrary + 'static
    {
    }
    impl<T: PartialEq + Clone + Debug + Arbitrary + 'static> TestRequirements for T {}

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) enum ArbitraryDequeOperation<T: TestRequirements> {
        PushLeft(T),
        PushRight(T),
        PopLeft,
        PopRight,
    }

    impl<T: TestRequirements> ArbitraryDequeOperation<T> {
        pub(super) fn apply(&self, mut deque: Deque<T>) -> Deque<T> {
            match self {
                ArbitraryDequeOperation::PushLeft(v) => {
                    let _ = deque.push_left(v.clone());
                }
                ArbitraryDequeOperation::PushRight(v) => {
                    let _ = deque.push_right(v.clone());
                }
                ArbitraryDequeOperation::PopLeft => {
                    let _ = deque.pop_left();
                }
                ArbitraryDequeOperation::PopRight => {
                    let _ = deque.pop_right();
                }
            }

            deque
        }
    }

    impl<T: TestRequirements> Arbitrary for ArbitraryDequeOperation<T> {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (any::<T>())
                .prop_flat_map(|value| {
                    prop_oneof![
                        Just(ArbitraryDequeOperation::PushLeft(value.clone())),
                        Just(ArbitraryDequeOperation::PushRight(value.clone())),
                        Just(ArbitraryDequeOperation::PopLeft),
                        Just(ArbitraryDequeOperation::PopRight)
                    ]
                })
                .boxed()
        }

        type Strategy = BoxedStrategy<ArbitraryDequeOperation<T>>;
    }

    #[allow(unused_mut)]
    fn apply_operations<T: TestRequirements>(
        mut input: (Deque<T>, Vec<ArbitraryDequeOperation<T>>),
    ) -> Deque<T> {
        let mut data = input.0;
        for op in input.1 {
            data = op.apply(data);
        }

        data
    }

    impl<T: TestRequirements> Arbitrary for Deque<T> {
        type Parameters = ();

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                Just(Deque::<T>::create()),
                vec(any::<ArbitraryDequeOperation<T>>(), 0..100),
            )
                .prop_map(apply_operations)
                .boxed()
        }

        type Strategy = BoxedStrategy<Deque<T>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deque;
    use proptest::prelude::*;

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn pushing_left_updates_the_first_item(mut original: Deque<usize>, value: usize) {
            original.push_left(value);
            assert_eq!(original.first().unwrap().value(), &value);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn pushing_left_changes_the_deque(mut original: Deque<usize>, value: usize) {
            let mut updated = original.clone();
            updated.push_left(value);
            assert_ne!(original, updated);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn pushing_left_increases_len_by_one(mut original: Deque<usize>, value: usize) {
            let original_len = original.len();
            original.push_left(value);
            assert_eq!(original.len(), original_len + 1);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn pushing_right_updates_the_last_item(mut original: Deque<usize>, value: usize) {
            original.push_right(value);
            assert_eq!(original.last().unwrap().value(), &value);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn pushing_right_changes_the_deque(mut original: Deque<usize>, value: usize) {
            let mut updated = original.clone();
            updated.push_right(value);
            assert_ne!(original, updated);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn pushing_right_increases_len_by_one(mut original: Deque<usize>, value: usize) {
            let original_len = original.len();
            original.push_right(value);
            assert_eq!(original.len(), original_len + 1);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn popping_left_decreases_len_by_one(mut original: Deque<usize>) {
            let original_size = original.len();
            let _ = original.pop_left();
            if original_size == 0 {
                assert_eq!(original_size, original.len());
            } else {
                assert_eq!(original_size-1, original.len());
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn popping_left_places_next_item_first(mut original: Deque<usize>) {
            if let Some(first) = original.clone().first() {
                let _ = original.pop_left();
                if let Some(second) = first.right() {
                    // note: we can only compare the value, as the left pointer of the second item has now changed
                    assert_eq!(original.first().unwrap().value(), second.value());
                } else {
                    assert_eq!(original.first(), None);
                }
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn popping_right_decreases_len_by_one(mut original: Deque<usize>){
            let original_size = original.len();
            let _ = original.pop_right();
            if original_size > 0 {
                assert_eq!(original_size-1, original.len());
            } else {
                assert_eq!(original_size, original.len());
            }
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn popping_right_makes_previous_item_last(mut original: Deque<usize>) {
            if let Some(last) = original.clone().last() {
                let _ = original.pop_right();
                if let Some(p) = last.left() {
                    // note: we can only compare the value, as the right pointer of the previous to last item has now changed
                    assert_eq!(original.last().unwrap().value(), p.value());
                } else {
                    assert_eq!(original.last(), None);
                }
            }
        }
    }

    #[test]
    fn pop_right_once_with_single_item_deque() {
        let mut data = deque![1];
        data.pop_right();
        assert_eq!(data, deque![]);
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
