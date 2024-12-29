use std::ptr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item<T> {
    pub(in crate::deque) left: *mut Item<T>,
    pub(in crate::deque) right: *mut Item<T>,
    pub(in crate::deque) value: T,
}

impl<T> Item<T> {
    pub(in crate::deque) fn create(value: T) -> *mut Item<T> {
        let item = Item {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            value,
        };

        let layout = std::alloc::Layout::new::<Item<T>>();
        let ptr = unsafe { std::alloc::alloc(layout) as *mut Item<T> };
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        unsafe {
            ptr::write(ptr, item);
        }

        ptr
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
