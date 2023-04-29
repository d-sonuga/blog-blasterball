use crate::allocator::Allocator;
use core::mem;
use core::cmp::PartialEq;
use core::fmt;


pub struct Box<T> {
    ptr: *mut T,
    allocator: *mut dyn Allocator
}

impl<T> Box<T> {
    pub fn new(val: T, allocator: *mut dyn Allocator) -> Box<T> {
        // Allocate heap memory for `val`
        let alloc_result = unsafe { (*allocator).alloc(mem::size_of::<T>(), mem::align_of::<T>()) };
        match alloc_result {
            // Heap allocation succeeded
            Some(ptr) => {
                // Interpret the pointer as a pointer to T
                let ptr = ptr as *mut T;
                // Move `val` into the memory just allocated for it
                unsafe { *ptr = val };
                Box {
                    ptr,
                    allocator
                }
            }
            // Heap allocation failed
            None => panic!("No enough space on the heap")
        }
    }

    // Consumes the box, returning the underlying pointer to the data
    pub fn into_raw(b: Box<T>) -> *mut T {
        use core::mem::ManuallyDrop;
        // Drop should not be called, so that deallocation won't take place
        let b = ManuallyDrop::new(b);
        b.ptr
    }

    // Creates a box from a raw pointer to a value already on the heap
    // and an allocator.
    // The caller has to ensure that `ptr` is pointing to a valid area on the heap
    pub unsafe fn from_raw<U>(ptr: *mut U, allocator: *mut dyn Allocator) -> Box<U> {
        Box {
            ptr,
            allocator
        }
    }
}

impl<T> core::ops::Drop for Box<T> {
    fn drop(&mut self) {
        unsafe { (*self.allocator).dealloc(self.ptr as *mut u8, mem::size_of::<T>()); }
    }
}

impl<T> core::ops::Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<T> core::ops::DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl<T: PartialEq> PartialEq<Box<T>> for Box<T> {
    fn eq(&self, other: &Box<T>) -> bool {
        unsafe { *self.ptr == *other.ptr }
    }
}

impl<T: PartialEq> PartialEq<T> for Box<T> {
    fn eq(&self, other: &T) -> bool {
        unsafe { *self.ptr == *other }
    }
}

impl<T: fmt::Debug> fmt::Debug for Box<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Box")
            .field("val", unsafe { &*self.ptr })
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::Allocator;

    #[test]
    fn test_partial_eq1() {
        let box1 = Box::new(32, successful_allocator());
        let box2 = Box::new(32, successful_allocator());
        let box3 = Box::new(1984, successful_allocator());
        assert_eq!(box1, box2);
        assert_ne!(box1, box3);
    }

    #[test]
    fn test_partial_eq2() {
        let box1 = Box::new(32, successful_allocator());
        let n1 = 32;
        let n2 = 1984;
        assert_eq!(box1, n1);
        assert_ne!(box1, n2);
    }

    #[test]
    fn test_deref() {
        let box1 = Box::new(45, successful_allocator());
        let mut n = 1_000;
        n = *box1;
        assert_eq!(n, 45);
    }

    #[test]
    fn test_deref_mut() {
        let mut b: Box<i32> = Box::new(45, successful_allocator());
        *b = 999_999;
        assert_eq!(b, 999_999);
    }

    #[test]
    #[should_panic]
    fn test_failed_allocation() {
        let box1 = Box::new(32, failing_allocator());
    }

    #[test]
    fn box_into_raw() {
        let b: Box<usize> = Box::new(1984, successful_allocator());
        let b_ptr = Box::into_raw(b);
        unsafe { assert_eq!(*b_ptr, 1984) }
    }

    #[test]
    fn box_from_raw() {
        let allocator = successful_allocator();
        let ptr = unsafe {
            let ptr = (*allocator).alloc(mem::size_of::<i32>(), mem::align_of::<i32>())
                .unwrap() as *mut i32;
            *ptr = 100_000_000;
            ptr
        };
        let b = unsafe { Box::<i32>::from_raw(ptr, allocator) };
        assert_eq!(*b, 100_000_000);
    }


    // Convenience function for getting the always fail allocator
    fn failing_allocator() -> *mut FailingAllocator {
        &mut FailingAllocator as *mut _
    }

    // Convenience function for getting the always successful allocator
    fn successful_allocator() -> *mut SuccessfulAllocator {
        &mut SuccessfulAllocator as *mut _
    }

    // Dummy allocator that we can depend on to always succeed
    struct SuccessfulAllocator;

    use std::alloc::Global as PlatformAllocator;
    use std::alloc::Layout;
    use std::ptr::NonNull;
    use std::alloc::Allocator as StdAllocator;

    // Use your computer's allocator to allocate and deallocate memory
    // Much more reliable than using our own custom allocator,
    // so we can depend on it succeeding (under normal circumstances)
    unsafe impl Allocator for SuccessfulAllocator {
        unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Option<*mut u8> {
            let mem_layout = Layout::from_size_align(size, alignment).unwrap();
            let mem = PlatformAllocator.allocate(mem_layout).unwrap();
            let ptr = mem.as_ptr() as *mut u8;
            Some(ptr)
        }
        unsafe fn dealloc(&mut self, ptr: *mut u8, size_to_dealloc: usize) {
            let mem_layout = Layout::from_size_align(size_to_dealloc, 1).unwrap();
            PlatformAllocator.deallocate(NonNull::new(ptr).unwrap(), mem_layout);
        }
    }

    
    // Dummy allocator we can depend on to always fail
    struct FailingAllocator;

    unsafe impl Allocator for FailingAllocator {
        unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Option<*mut u8> {
            None
        }
        unsafe fn dealloc(&mut self, ptr: *mut u8, size_to_dealloc: usize) {}
    }
}