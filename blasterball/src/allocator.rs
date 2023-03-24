// An allocator for managing heap memory
// Keeps track of free regions with a linked list whose nodes
// are stored in the free regions themselves
pub struct LinkedListAllocator {
    // The first free region
    head: *mut ListNode
}

// A free region of memory
struct ListNode {
    // The size of the free region
    size: usize,
    // The next free region
    next: Option<*mut ListNode>
}

impl ListNode {
    // The address of the ListNode, and hence, the free
    // region it represents
    fn addr(&self) -> usize {
        self as *const _ as usize
    }
}

// A chunk of memory
pub struct MemChunk {
    start_addr: usize,
    size: usize
}

impl MemChunk {
    fn start_addr(&self) -> usize {
        self.start_addr
    }

    fn end_addr(&self) -> usize {
        self.start_addr + self.size - 1
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl LinkedListAllocator {
    // Finds a suitable memory chunk of size `size` and with an address
    // of alignment `alignment` and returns a pointer to the region
    unsafe fn find_free_region(&mut self, size: usize, alignment: usize) -> Option<*mut u8> {
        let mut prev_node_ptr = self.head;
        while let Some(curr_node_ptr) = (*prev_node_ptr).next {
            // Computing the closest aligned pointer
            let region_ptr = (*curr_node_ptr).addr() as *mut u8;
            let closest_aligned_ptr_distance = region_ptr.align_offset(alignment);
            let closest_aligned_ptr = region_ptr.offset(closest_aligned_ptr_distance as isize);

            // Perfect fit and CAP is the region start
            if (*curr_node_ptr).size == size && closest_aligned_ptr == region_ptr {
                // Before: Node -> Node to Remove -> Node
                // After: Node -> Node
                core::mem::swap(&mut (*prev_node_ptr).next, &mut (*curr_node_ptr).next);
                return Some(region_ptr);
            }
            // Perfect fit and CAP is not the region start
            if (*curr_node_ptr).size == size && closest_aligned_ptr != region_ptr {
                (*curr_node_ptr).size -= size;
                return Some(closest_aligned_ptr);
            }
            
            // Bigger and CAP is the region start
            if (*curr_node_ptr).size > size && closest_aligned_ptr == region_ptr {
                // Reduce by requested size
                (*curr_node_ptr).size -= size;
                // The new location for the node
                let new_location = (*curr_node_ptr).addr() + size;
                let mut new_location_ptr = new_location as *mut ListNode;
                // Moving the node to the new location
                new_location_ptr.write_unaligned(curr_node_ptr.read_unaligned());
                // Setting the previous node's next to the new location
                (*prev_node_ptr).next = Some(new_location_ptr);
                return Some(curr_node_ptr as *mut u8);
            }

            // Bigger and CAP is not the region start
            if (*curr_node_ptr).size > size && closest_aligned_ptr != region_ptr {
                let region_end = (*curr_node_ptr).addr() + size;

                // Make the node end right before CAP
                let curr_node_size = closest_aligned_ptr as usize - region_ptr as usize;
                (*curr_node_ptr).size = curr_node_size;

                // Create a new node starting at CAP + size and ending at the original region end
                let mut new_node_ptr = closest_aligned_ptr.offset(size as isize) as *mut ListNode;
                (*new_node_ptr).size = region_end - new_node_ptr as usize;

                // Set the new node's next to the previous node's next
                (*new_node_ptr).next = (*curr_node_ptr).next;

                // Set the current node's next to the new node
                (*curr_node_ptr).next = Some(new_node_ptr);
                
                return Some(closest_aligned_ptr);
            }

            // Region beginning at CAP is too small
            // Moving to the next region
            prev_node_ptr = curr_node_ptr;
        }
        // No suitable region found
        None
    }

    // Manage the free region described by `mem_chunk`
    unsafe fn add_free_region(&mut self, mem_chunk: MemChunk) {
        let mut prev_node_ptr = self.head;
        while let Some(curr_node_ptr) = (*prev_node_ptr).next {
            let mut chunk_comes_imm_after_curr_region = false;
            let mut chunk_comes_imm_before_next_region = false;

            let region_end_addr = curr_node_ptr as usize + (*curr_node_ptr).size - 1;
            let next_node_ptr_opt = (*curr_node_ptr).next;

            // The memory chunk comes immediately after the current free region
            // Regions: -----NNNN--------
            // Chunk:   ---------MMM-----
            if mem_chunk.start_addr() == region_end_addr + 1 {
                chunk_comes_imm_after_curr_region = true;
            }

            // The memory chunk comes immediately before the next free region
            // Regions: ------NNNN-------
            // Chunk:   ---MMM-----------
            if let Some(next_node_ptr) = next_node_ptr_opt {
                if (*next_node_ptr).addr() == mem_chunk.end_addr() + 1 {
                    chunk_comes_imm_before_next_region = true;
                }
            }

            if chunk_comes_imm_after_curr_region && !chunk_comes_imm_before_next_region {
                // Merge the new chunk with the current region
                (*curr_node_ptr).size += mem_chunk.size();
                return;
            }

            if chunk_comes_imm_before_next_region && !chunk_comes_imm_after_curr_region {
                // Shift the node to the mem chunk's start address and increase the size
                let new_region_start_ptr = mem_chunk.start_addr() as *mut ListNode;
                let next_node_ptr = next_node_ptr_opt.unwrap();
                let new_region_size = mem_chunk.size() + (*next_node_ptr).size;
                new_region_start_ptr.write_unaligned(next_node_ptr.read_unaligned());
                (*new_region_start_ptr).size = new_region_size;
                return;
            }

            // Regions: ----NNNN---NNNNNN
            // Chunk:   --------MMM------
            if chunk_comes_imm_before_next_region && chunk_comes_imm_after_curr_region {
                // Merge the chunk and the second node into the first node
                let next_node_ptr = next_node_ptr_opt.unwrap();
                let new_region_size = (*curr_node_ptr).size + mem_chunk.size() + (*next_node_ptr).size;
                (*curr_node_ptr).size = new_region_size;
                (*curr_node_ptr).next = (*next_node_ptr).next;
                return;
            }

            // Memory chunk come immediately before the current free region
            if mem_chunk.end_addr() + 1 == (*curr_node_ptr).addr() {
                let new_region_start_ptr = mem_chunk.start_addr() as *mut ListNode;
                let new_region_size = mem_chunk.size() + (*curr_node_ptr).size;
                new_region_start_ptr.write_unaligned(curr_node_ptr.read_unaligned());
                (*new_region_start_ptr).size = new_region_size;
                (*prev_node_ptr).next = Some(new_region_start_ptr);
                return;
            }

            if mem_chunk.start_addr() < (*curr_node_ptr).addr() {
                // The region to be added doesn't come immediately before or after any free region
                // but still comes before a region
                // The region must be placed before those that come after it to maintain
                // the order
                // Regions: ---NNN-----------NNNN-----
                // Chunk:   ----------MMM-------------
                let mut new_node = mem_chunk.start_addr() as *mut ListNode;
                (*new_node).size = mem_chunk.size();
                (*new_node).next = Some(curr_node_ptr);
                (*prev_node_ptr).next = Some(new_node);
                return;
            }
            
            prev_node_ptr = curr_node_ptr;
        }
        // The region to be added comes after all other regions
        // Regions: ---NNN----------------
        // Chunk:   ----------MMM---------
        let mut new_node = mem_chunk.start_addr() as *mut ListNode;
        (*new_node).size = mem_chunk.size();
        (*new_node).next = None;
        (*prev_node_ptr).next = Some(new_node);
    }

    // Allocate heap memory
    pub fn alloc(&mut self, size: usize, alignment: usize) -> Option<*mut u8> {
        unsafe {
            self.find_free_region(size, alignment)
        }
    }

    // Deallocate heap memory
    pub fn dealloc(&mut self, ptr: *mut u8, size_to_dealloc: usize) {
        unsafe {
            self.add_free_region(MemChunk {
                start_addr: ptr as usize,
                size: size_to_dealloc
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::ManuallyDrop;
    use std::vec::Vec as StdVec;

    const ONE_KIB: usize = 2usize.pow(10);

    #[test]
    fn test_allocate_3Kib() {
        unsafe { 
            let mut allocator = new_allocator(10 * ONE_KIB);
            let first_region = (*allocator.head).next.unwrap();
            assert_eq!((*first_region).size, 10 * ONE_KIB, "The first node should be 10Kib");
            assert_eq!((*first_region).next, None, "There should be only one node");
            let ptr = allocator.alloc(3 * ONE_KIB, 1);
            assert!(ptr.is_some(), "Allocation should have been successful");
            let first_region = (*allocator.head).next.unwrap();
            assert_eq!((*first_region).size, 7 * ONE_KIB, "The first node should now be 7Kib");
            assert_eq!((*first_region).next, None, "There should still be only one node");
        }
    }

    #[test]
    fn test_allocate_3_4_2Kib() {
        unsafe { 
            let mut allocator = new_allocator(10 * ONE_KIB);
            let first_region = (*allocator.head).next.unwrap();
            assert_eq!((*first_region).size, 10 * ONE_KIB, "The first node should be 10Kib");
            assert_eq!((*first_region).next, None, "There should be only one node");
            let ptr_3 = allocator.alloc(3 * ONE_KIB, 1);
            let ptr_4 = allocator.alloc(4 * ONE_KIB, 1);
            let ptr_2 = allocator.alloc(2 * ONE_KIB, 1);
            assert!(ptr_3.is_some(), "Allocation of 3Kib should have been successful");
            assert!(ptr_4.is_some(), "Allocation of 4Kib should have been successful");
            assert!(ptr_2.is_some(), "Allocation of 2Kib should have been successful");
            let first_region = (*allocator.head).next.unwrap();
            assert_eq!((*first_region).size, 1 * ONE_KIB, "The first node should now be 1Kib");
            assert_eq!((*first_region).next, None, "There should still be only one node");
        }
    }

    #[test]
    fn test_dealloc_3_4_2Kib1() {
        unsafe {
            let (mut allocator, four_kib_ptr) = setup_3_4_2_dealloc_test1();
            let first_region = (*allocator.head).next.unwrap();            
            assert_eq!((*first_region).size, 1 * ONE_KIB, "The first node should be 1Kib");
            assert_eq!((*first_region).next, None, "There should still be only one node");
            allocator.dealloc(four_kib_ptr, 4 * ONE_KIB);
            let first_region = (*allocator.head).next.unwrap();
            assert_eq!((*first_region).size, 4 * ONE_KIB, "The first node should now be 4Kib");
            assert!((*first_region).next.is_some(), "There should be a next node");
            let second_region = (*first_region).next.unwrap();
            assert_eq!((*second_region).size, 1 * ONE_KIB, "The second node should be 1Kib");
            assert_eq!((*second_region).next, None, "There should be only two nodes");
        }
    }

    unsafe fn setup_3_4_2_dealloc_test1() -> (LinkedListAllocator, *mut u8) {
        let mut allocator = new_allocator(10 * ONE_KIB);
        let ptr_3 = allocator.alloc(3 * ONE_KIB, 1);
        let ptr_4 = allocator.alloc(4 * ONE_KIB, 1).unwrap();
        let ptr_2 = allocator.alloc(2 * ONE_KIB, 1);
        (allocator, ptr_4)
    }

    #[test]
    fn test_dealloc_3_4_2Kib2() {
        unsafe {
            let (mut allocator, three_kib_ptr) = setup_3_4_2_dealloc_test2();
            let first_region = (*allocator.head).next.unwrap();
            assert_eq!((*first_region).size, 4 * ONE_KIB, "First node should be 4Kib");
            assert!((*first_region).next.is_some(), "There should be a second node");
            let second_region = (*first_region).next.unwrap();
            assert_eq!((*second_region).size, 1 * ONE_KIB, "The second node should be 1Kib");
            assert!((*second_region).next.is_none(), "There should be only two nodes");

            allocator.dealloc(three_kib_ptr, 3 * ONE_KIB);

            let first_region = (*allocator.head).next.unwrap();
            assert_eq!((*first_region).size, 7 * ONE_KIB, "The first node should be 7Kib");
            assert!((*first_region).next.is_some(), "There should be a second node");
            let second_region = (*first_region).next.unwrap();
            assert_eq!((*second_region).size, 1 * ONE_KIB, "The second node should be 1Kib");
            assert!((*second_region).next.is_none(), "There should be only two nodes");

        }
    }

    unsafe fn setup_3_4_2_dealloc_test2() -> (LinkedListAllocator, *mut u8) {
        let mut allocator = new_allocator(10 * ONE_KIB);
        let ptr_3 = allocator.alloc(3 * ONE_KIB, 1).unwrap();
        let ptr_4 = allocator.alloc(4 * ONE_KIB, 1).unwrap();
        let ptr_2 = allocator.alloc(2 * ONE_KIB, 1);
        allocator.dealloc(ptr_4, 4 * ONE_KIB);
        (allocator, ptr_3)
    }

    #[test]
    fn test_dealloc_3_4_2Kib3() {
        unsafe {
            let (mut allocator, two_kib_ptr) = setup_3_4_2_dealloc_test3();
            let first_region = (*allocator.head).next.unwrap();
            assert_eq!((*first_region).size, 7 * ONE_KIB, "First node should be 7Kib");
            assert!((*first_region).next.is_some(), "There should be a second node");
            let second_region = (*first_region).next.unwrap();
            assert_eq!((*second_region).size, 1 * ONE_KIB, "The second node should be 1Kib");
            assert!((*second_region).next.is_none(), "There should be only two nodes");

            allocator.dealloc(two_kib_ptr, 2 * ONE_KIB);

            let first_region = (*allocator.head).next.unwrap();
            assert_eq!((*first_region).size, 10 * ONE_KIB, "The first node should be 10Kib");
            assert!((*first_region).next.is_none(), "There should not be a second node");
        }
    }

    unsafe fn setup_3_4_2_dealloc_test3() -> (LinkedListAllocator, *mut u8) {
        let mut allocator = new_allocator(10 * ONE_KIB);
        let ptr_3 = allocator.alloc(3 * ONE_KIB, 1).unwrap();
        let ptr_4 = allocator.alloc(4 * ONE_KIB, 1).unwrap();
        let ptr_2 = allocator.alloc(2 * ONE_KIB, 1).unwrap();
        allocator.dealloc(ptr_4, 4 * ONE_KIB);
        allocator.dealloc(ptr_3, 3 * ONE_KIB);
        (allocator, ptr_2)
    }

    fn new_allocator(memory_size: usize) -> LinkedListAllocator {
        // Allocate `memory_size` bytes from the OS's heap
        // Using `ManuallyDrop` so the memory won't be freed after a return from this function
        let mem: ManuallyDrop<StdVec<u8>> = ManuallyDrop::new(StdVec::with_capacity(memory_size));
        let mem_ptr = mem.as_ptr() as *mut u8;
        let dummy_node_ptr = Box::into_raw(Box::new(ListNode {
            size: 0,
            next: None
        }));
        let mut allocator = LinkedListAllocator {
            // A dummy head, as expected by the allocator functions
            head: dummy_node_ptr
        };
        // Adding the memory to manage
        unsafe {
            allocator.add_free_region(MemChunk {
                start_addr: mem_ptr as usize,
                size: memory_size
            });
        }
        return allocator;
    }
}