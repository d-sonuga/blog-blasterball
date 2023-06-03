pub mod allocator;
pub mod boxed_fn;
pub mod boxed;
pub mod vec;

#[cfg(test)]
pub mod test_utils {
    use crate::allocator::Allocator;

    // Convenience function for getting the always fail allocator
    pub fn failing_allocator() -> &'static FailingAllocator {
        &FailingAllocator
    }

    // Convenience function for getting the always successful allocator
    pub fn successful_allocator() -> &'static SuccessfulAllocator {
        &SuccessfulAllocator
    }

    // Dummy allocator that we can depend on to always succeed
    pub struct SuccessfulAllocator;

    use std::alloc::Global as PlatformAllocator;
    use std::alloc::Layout;
    use std::ptr::NonNull;
    use std::alloc::Allocator as StdAllocator;

    // Use your computer's allocator to allocate and deallocate memory
    // Much more reliable than using our own custom allocator,
    // so we can depend on it succeeding (under normal circumstances)
    unsafe impl Allocator for SuccessfulAllocator {
        unsafe fn alloc(&self, size: usize, alignment: usize) -> Option<*mut u8> {
            let mem_layout = Layout::from_size_align(size, alignment).unwrap();
            let mem = PlatformAllocator.allocate(mem_layout).unwrap();
            let ptr = mem.as_ptr() as *mut u8;
            Some(ptr)
        }
        unsafe fn dealloc(&self, ptr: *mut u8, size_to_dealloc: usize) {
            let mem_layout = Layout::from_size_align(size_to_dealloc, 1).unwrap();
            PlatformAllocator.deallocate(NonNull::new(ptr).unwrap(), mem_layout);
        }
    }

    
    // Dummy allocator we can depend on to always fail
    pub struct FailingAllocator;

    unsafe impl Allocator for FailingAllocator {
        unsafe fn alloc(&self, size: usize, alignment: usize) -> Option<*mut u8> {
            None
        }
        unsafe fn dealloc(&self, ptr: *mut u8, size_to_dealloc: usize) {}
    }
}