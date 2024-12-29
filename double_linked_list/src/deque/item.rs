#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item<T> {
    pub(in crate::deque) left: *mut Item<T>,
    pub(in crate::deque) right: *mut Item<T>,
    pub(in crate::deque) value: T
}

impl <T> Item<T> {
    pub(in crate::deque) fn create(value: T) -> Item<T> {
        Item {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            value
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn left(&self) -> Option<&Item<T>> {
        None
    }

    pub fn right(&self) -> Option<&Item<T>> {
        None
    }
}