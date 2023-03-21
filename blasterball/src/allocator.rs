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
struct MemChunk {
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
        let prev_node_ptr = self.head;
        while let Some(curr_node_ptr) = (*prev_node_ptr).next {
            let mut chunk_comes_after_free_region = false;
            let mut chunk_comes_before_free_region = false;

            let region_end_addr = curr_node_ptr as usize + (*curr_node_ptr).size - 1;
            let next_node_ptr_opt = (*curr_node_ptr).next;

            // The memory chunk comes immediately after another free region
            // Regions: -----NNNN--------
            // Chunk:   ---------MMM-----
            if mem_chunk.start_addr() == region_end_addr + 1 {
                chunk_comes_after_free_region = true;
            }

            // The memory chunk comes immediately before another free region
            // Regions: ------NNNN-------
            // Chunk:   ---MMM-----------
            if let Some(next_node_ptr) = next_node_ptr_opt {
                if (*next_node_ptr).addr() == mem_chunk.end_addr() + 1 {
                    chunk_comes_before_free_region = true;
                }
            }

            if chunk_comes_after_free_region && !chunk_comes_before_free_region {
                // Merge the new chunk with the region that comes before
                (*curr_node_ptr).size += mem_chunk.size();
                return;
            }

            if chunk_comes_before_free_region && !chunk_comes_after_free_region {
                // Shift the node to the mem chunk's start address and increase the size
                let new_region_start_ptr = mem_chunk.start_addr() as *mut ListNode;
                let new_region_size = mem_chunk.size() + (*curr_node_ptr).size;
                new_region_start_ptr.write_unaligned(curr_node_ptr.read_unaligned());
                (*new_region_start_ptr).size = new_region_size;
                return;
            }

            // Regions: ----NNNN---NNNNNN
            // Chunk:   --------MMM------
            if chunk_comes_before_free_region && chunk_comes_after_free_region {
                // Merge the chunk and the second node into the first node
                let new_region_size = (*curr_node_ptr).size + mem_chunk.size() + (*next_node_ptr_opt.unwrap()).size;
                (*curr_node_ptr).size = new_region_size;
                return;
            }
        }
        // The region to be added doesn't come immediately before or after any free region
        // Regions: ---NNN-----------NNNN-----
        // Chunk:   ----------MMM-------------
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
