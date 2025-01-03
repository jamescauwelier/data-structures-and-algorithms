use crate::deque::item::Item;
use crate::deque::{Deque, DequeTypeRequirements};

pub struct DequeIterator<'a, T: DequeTypeRequirements> {
    current: Option<&'a Item<T>>,
}

impl<'a, T: DequeTypeRequirements> Iterator for DequeIterator<'a, T> {
    type Item = &'a Item<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_item = self.current;
        self.current = match self.current {
            // if there is no current item, no right item can be found
            None => None,
            // if there is a current item, pick the next right, which may be a None
            // note that there are two levels of Option here
            // lvl1 -> the optionality of a next iterator item
            // lvl2 -> the optionality of a right link on the item
            Some(item) => match item.right() {
                None => None,
                Some(item) => Some(item),
            },
        };

        current_item
    }
}

impl<'a, T: DequeTypeRequirements> IntoIterator for &'a Deque<T> {
    type Item = &'a Item<T>;
    type IntoIter = DequeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        DequeIterator {
            current: self.first(),
        }
    }
}
