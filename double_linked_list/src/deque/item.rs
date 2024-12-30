use std::ptr;

#[derive(Debug, Clone)]
pub struct Item<T>
where
    T: PartialEq,
{
    pub(in crate::deque) left_ptr: *mut Item<T>,
    pub(in crate::deque) right_ptr: *mut Item<T>,
    pub(in crate::deque) value: T,
}

impl<T> Item<T>
where
    T: PartialEq,
{
    pub(in crate::deque) fn create(value: T) -> *mut Item<T> {
        let item = Item {
            left_ptr: std::ptr::null_mut(),
            right_ptr: std::ptr::null_mut(),
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
        if self.left_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*self.left_ptr })
        }
    }

    pub fn right(&self) -> Option<&Item<T>> {
        if self.right_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*self.right_ptr })
        }
    }

    fn equal_values(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn equal_left_values(&self, other: &Self) -> bool {
        match (self.left(), other.left()) {
            (None, None) => true,
            (Some(x1), Some(x2)) => x1.value == x2.value,
            _ => false,
        }
    }

    fn equal_right_values(&self, other: &Self) -> bool {
        match (self.right(), other.right()) {
            (None, None) => true,
            (Some(x1), Some(x2)) => x1.value == x2.value,
            _ => false,
        }
    }
}

impl<T: PartialEq> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.equal_values(other) && self.equal_left_values(other) && self.equal_right_values(other)
    }
}

impl<T: PartialEq> Eq for Item<T> {}
