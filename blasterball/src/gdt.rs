// An artificial limit placed on the number of entries that can be placed
// in the GDT's descriptor array, for convenience
const MAX_NO_OF_ENTRIES: usize = 8;

// The Global Descriptor Table
#[repr(C, align(8))]
struct GDT {
    // The segment descriptors
    descriptors: [u64; MAX_NO_OF_ENTRIES],
    // The next index available to place a descriptor in the GDT
    next_index: usize
}

impl GDT {
    // Creates a new GDT
    fn new() -> Self {
        Self {
            descriptors: [0; MAX_NO_OF_ENTRIES],
            // Start inserting at index 1 to keep the first entry
            // as a null descriptor
            next_index: 1
        }
    }

    // Adds a segment descriptor to the descriptors array
    fn add_descriptor(&mut self, descriptor: Descriptor) -> Result<(), &'static str> {
        match descriptor {
            Descriptor::NonSystem(value) => {
                // Is array full?
                if self.next_index >= self.descriptors.len() {
                    return Err("no enough space for descriptor");
                }
                self.descriptors[self.next_index] = value;
                self.next_index += 1;
                Ok(())
            }
            Descriptor::System(higher, lower) => {
                // Is there enough space for a system descriptor?
                if self.next_index + 1 >= self.descriptors.len() {
                    return Err("No enough space for descriptor");
                }
                self.descriptors[self.next_index] = lower;
                self.descriptors[self.next_index + 1] = higher;
                self.next_index += 2;
                Ok(())
            }
        }
    }
}

// A segment descriptor
enum Descriptor {
    NonSystem(u64),
    // (upper 64 bits, lower 64 bits) of segment descriptor
    System(u64, u64)
}