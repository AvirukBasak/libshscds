use std::alloc;

pub fn allocate<T>(size: usize) -> *mut T {
    let layout = alloc::Layout::array::<T>(size).expect("allocate: Invalid layout");
    unsafe {
        let ptr = alloc::alloc(layout);
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        ptr as *mut T
    }
}

pub fn reallocate<T>(ptr: *mut T, newsz: usize) -> *mut T {
    let old_layout = alloc::Layout::for_value(unsafe { &*ptr });
    let new_layout = alloc::Layout::array::<T>(newsz).expect("reallocate: Invalid layout");
    unsafe {
        let new_ptr = alloc::realloc(ptr as *mut u8, old_layout, new_layout.size());
        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }
        new_ptr as *mut T
    }
}

pub fn deallocate<T>(ptr: *mut T, size: usize) {
    let layout = alloc::Layout::array::<T>(size).expect("deallocate: Invalid layout");
    unsafe {
        alloc::dealloc(ptr as *mut u8, layout);
    }
}
