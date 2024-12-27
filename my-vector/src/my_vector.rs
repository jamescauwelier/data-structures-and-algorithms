/// MyVector
///
/// Implements a dynamic array in Rust as a study exercise.
pub struct MyVector {
    data_ptr: *mut usize,
    len: usize,
    capacity: usize
}

impl MyVector {
    pub fn new() -> Self {
        MyVector {
            data_ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> &usize {
        unsafe {
            let ptr = self.data_ptr.add(index) as *const usize;
            let data = &*ptr;

            data
        }
    }

    pub fn find<F>(&self, predicate: F) -> Option<&usize>
    where
        F: Fn(&usize) -> bool
    {
        for n in 0..self.len {
            let v = self.get(n);
            if predicate(v) {
                return Some(v)
            }
        }

        None
    }

    fn resize(&mut self, new_capacity: usize) {
        self.data_ptr = unsafe {
           let new_data_ptr = std::alloc::alloc(
                std::alloc::Layout::array::<usize>(new_capacity).expect("Failed to allocate memory due to memory layout error")
            ) as *mut usize;
            if !self.data_ptr.is_null() {
                std::ptr::copy_nonoverlapping(self.data_ptr, new_data_ptr, self.len);
                std::alloc::dealloc(
                    self.data_ptr as *mut u8,
                    std::alloc::Layout::array::<usize>(self.capacity).expect("Failed to deallocate memory due to memory layout error")
                )
            } else {
                // current data ptr is null, so we are creating a new vector with no old data to move and deallocate
            }

            new_data_ptr
        };
        self.capacity += 1;
    }

    pub fn add(&mut self, element: usize) {

        // resize if necessary
        if self.len >= self.capacity {
            // todo: make a smarter strategy to resize the vector
            self.resize(self.capacity + 1);
        }

        // appends the element to the end fo the vector
        unsafe {
            if self.data_ptr.is_null() {
                panic!("Should've been resized...");
            } else {
                std::ptr::write(self.data_ptr.add(self.len), element);
            }
        }

        // updates the current length of the vector
        self.len += 1;
    }
}