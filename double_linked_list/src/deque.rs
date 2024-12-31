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

    pub fn pop_left(&mut self) -> Option<&Item<T>> {
        None
    }

    pub fn pop_right(&mut self) -> Option<&Item<T>> {
        None
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
    }

    impl<T: TestRequirements> ArbitraryDequeOperation<T> {
        pub(super) fn apply(&self, mut deque: Deque<T>) -> Deque<T> {
            match self {
                ArbitraryDequeOperation::PushLeft(v) => {
                    deque.push_left(v.clone());
                }
                ArbitraryDequeOperation::PushRight(v) => {
                    deque.push_right(v.clone());
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
                        Just(ArbitraryDequeOperation::PushRight(value.clone()))
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
                if let Some(second) = first.right() {
                    let _ = original.pop_left();
                    // note: we can only compare the value, as the left pointer of the second item has now changed
                    assert_eq!(original.first().unwrap().value(), second.value());
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
                if let Some(previous) = last.left() {
                    let _ = original.pop_right();
                    // note: we can only compare the value, as the right pointer of the previous to last item has now changed
                    assert_eq!(original.last().unwrap().value(), previous.value());
                }
            }
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
