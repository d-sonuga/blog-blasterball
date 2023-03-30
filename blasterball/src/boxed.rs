use crate::allocator::LinkedListAllocator;
use core::mem;
use core::cmp::PartialEq;


pub struct Box<T> {
    ptr: *mut T,
    allocator: *mut LinkedListAllocator
}

impl<T> Box<T> {
    pub fn new(val: T, allocator: *mut LinkedListAllocator) -> Box<T> {
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