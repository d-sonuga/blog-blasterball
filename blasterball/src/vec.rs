use core::mem;
use crate::allocator::Allocator;
use core::ops::{Index, IndexMut};

// A growable array on the heap
pub struct Vec<T: Clone> {
    len: usize,
    capacity: usize,
    start_ptr: *mut T,
    allocator: *mut dyn Allocator
}

impl<T: Clone> Vec<T> {

    // Creates a vector with the stated capacity
    pub fn with_capacity(capacity: usize, allocator: *mut dyn Allocator) -> Vec<T> {
        // Allocate heap memory to hold `capacity` items of type `T`
        let alloc_result = unsafe { (*allocator).alloc(mem::size_of::<T>() * capacity, mem::align_of::<T>()) };
        match alloc_result {
            // Heap allocation succeeded
            Some(ptr) => Vec {
                len: 0,
                capacity,
                start_ptr: ptr as *mut T,
                allocator
            },
            // Heap allocation failed
            None => panic!("No enough space on the heap")
        }
    }

    // Add an item to the end of the vector
    pub fn push(&mut self, item: T) {
        // There is enough space for the new item
        if self.capacity > self.len {
            unsafe { 
                let ptr_to_next_position = self.start_ptr.offset(self.len as isize);
                ptr_to_next_position.write(item);
            }
            self.len += 1;
        } else {
            // Allocate double the capacity
            let new_size = self.capacity * 2;
            let old_size = self.capacity;
            let old_start_ptr = self.start_ptr as *mut u8;
            let new_start_ptr = unsafe {
                (*self.allocator).alloc(mem::size_of::<T>()  * new_size, mem::align_of::<T>())
            }.expect("No enough space on the heap.") as *mut T;
            // Move data into the new location
            for i in 0..self.len {
                unsafe {
                    let val = self.start_ptr.offset(i as isize).read();
                    new_start_ptr.offset(i as isize).write(val);
                }
            }
            // Deallocate the previous heap memory
            unsafe { (*self.allocator).dealloc(old_start_ptr, old_size * mem::size_of::<T>()) };
            // Update the fields to reflect the new capacity and heap memory
            self.capacity = new_size;
            self.start_ptr = new_start_ptr as *mut T;
        }
    }

    // Removes and returns the last item in the vector
    // Panics when there are no items in the vector
    pub fn pop(&mut self) -> T {
        if self.len == 0 {
            panic!("No items to pop");
        }
        self.len -= 1;
        unsafe { self.start_ptr.offset(self.len as isize).read() }
    }

    // Removes and returns the last item in the vector
    // When there are no items in the vector, a `None` is returned
    pub fn try_pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            Some(self.pop())
        }
    }

    // Removes the item at index `idx` and returns it
    // Panics when `idx` is an invalid index
    pub fn remove(&mut self, idx: usize) -> T {
        // First, check if the index is valid
        if idx >= self.len {
            panic!("Invalid index");
        }
        // Retrieve the value at the index `idx`
        let value = unsafe { self.start_ptr.offset(idx as isize).read() };
        // Shift the elements that come after the removed value
        // to cover the hole (without dropping them)
        for i in idx + 1..self.len {
            let i = i as isize;
            unsafe {
                let val = self.start_ptr.offset(i).read();
                self.start_ptr.offset(i - 1).write(val);
            }
        }
        // Update the `len` field to reflect the new vector state
        self.len -= 1;
        value
    }

    // Returns the number of items in the vector
    pub fn len(&self) -> usize {
        self.len
    }

    // Returns the capacity of the vector
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // Returns the pointer to the vector data
    pub fn as_ptr(&self) -> *const T {
        self.start_ptr
    }

    // Creates a new iterator over the references of the vector's elements
    pub fn iter(&self) -> core::slice::Iter<T> {
        unsafe { core::slice::from_raw_parts(self.start_ptr as *const T, self.len) }
            .iter()
    }

    // Creates a new iterator over mutable references of the vector's elements
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        unsafe { core::slice::from_raw_parts_mut(self.start_ptr, self.len) }
            .iter_mut()
    }
}

impl<T: Clone> Drop for Vec<T> {
    fn drop(&mut self) {
        use core::ptr;
        unsafe {
            unsafe {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.start_ptr, self.len));
                (*self.allocator).dealloc(self.start_ptr as *mut u8, self.capacity * mem::size_of::<T>());
            };
        };
    }
}

impl<T: Clone> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!(idx < self.len);
        unsafe { &*self.start_ptr.offset(idx as isize) }
    }
}

impl<T: Clone> IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!(idx < self.len);
        unsafe { &mut *self.start_ptr.offset(idx as isize) }
    }
}

impl<T: PartialEq + Clone> PartialEq<Vec<T>> for Vec<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for (val1, val2) in self.iter().zip(other.iter()) {
            if val1 != val2 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::Allocator;

    #[test]
    fn test_push1() {
        let mut v: Vec<u8> = Vec::with_capacity(5, successful_allocator());
        v.push(1);
        v.push(2);
        v.push(255);
        assert_eq!(1, unsafe { *v.start_ptr.offset(0) });
        assert_eq!(2, unsafe { *v.start_ptr.offset(1) });
        assert_eq!(255, unsafe { *v.start_ptr.offset(2) });
    }

    #[test]
    fn test_push2() {
        let mut v: Vec<u8> = Vec::with_capacity(3, successful_allocator());
        v.push(2);
        v.push(5);
        v.push(10);
        v.push(11);
        assert_eq!(v.capacity(), 6);
    }

    #[test]
    fn test_pop1() {
        let mut v = Vec::with_capacity(3, successful_allocator());
        v.push(212);
        v.push(33);
        assert_eq!(v.len(), 2);
        assert_eq!(v.pop(), 33);
        assert_eq!(v.len(), 1);
        assert_eq!(v.pop(), 212);
        assert_eq!(v.len(), 10);
    }

    #[should_panic]
    #[test]
    fn test_pop2() {
        let mut v = Vec::with_capacity(1, successful_allocator());
        v.pop();
    }

    #[test]
    fn test_index() {
        let mut v = Vec::with_capacity(5, successful_allocator());
        v.push(2);
        assert_eq!(v[0], 2);
        v.push(3);
        assert_eq!(v[1], 3);
        v[0] = 100_000;
        assert_eq!(v[0], 100_000);
    }

    #[test]
    fn test_iter() {
        let mut v = Vec::with_capacity(2, successful_allocator());
        v.push(2);
        v.push(4);
        let mut v_iter = v.iter();
        assert_eq!(v_iter.next(), Some(&2));
        assert_eq!(v_iter.next(), Some(&4));
        assert_eq!(v_iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut v = Vec::with_capacity(2, successful_allocator());
        v.push(4);
        v.push(9);
        {
            let mut v_iter_mut = v.iter_mut();
            let first_item = v_iter_mut.next();
            assert_eq!(first_item, Some(&mut 4));
            let first_item = first_item.unwrap();
            *first_item = 999_999_999;
        }
        assert_eq!(v[0], 999_999_999);
    }

    #[test]
    fn vec_clone() {
        let v = crate::vec![4, 5, 87777; &AlwaysSuccessfulAllocator];
        let mut v = Vec::with_capacity(3, successful_allocator());
        v.push(4);
        v.push(5);
        v.push(87777);
        let other_v = v.clone();
        assert_eq!(v, other_v);
        assert_eq!(v.len(), other_v.len());
        assert_eq!(v.capacity(), other_v.capacity());
    }

    // Convenience function for getting the always successful allocator
    fn successful_allocator() -> *mut SuccessfulAllocator {
        &mut SuccessfulAllocator as *mut _
    }

        // Convenience function for getting the always fail allocator
    fn failing_allocator() -> *mut FailingAllocator {
        &mut FailingAllocator as *mut _
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
            // Using an alignment of 1 here because I think the alignment no
            // longer matters here. We're deallocating memory because we're
            // done using it
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

