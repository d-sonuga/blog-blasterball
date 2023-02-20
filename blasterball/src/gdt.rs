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

impl Descriptor {
    const VALID: u64 = 1 << 47;
    const BASE_0_23: u64 = 0;
    const BASE_24_31: u64 = 0;
    const GRANULARITY: u64 = 1 << 55;
    const LIMIT_0_15: u64 = 0xffff;
    const LIMIT_48_51: u64 = 0xf << 48;
    const BIT_SIZE: u64 = 0 << 54;
    const IS_CODE: u64 = 1 << 53;
    const EXECUTABLE: u64 = 1 << 43;
    const NON_SYSTEM_SEGMENT: u64 = 1 << 44;
    const READ_WRITE: u64 = 1 << 41;

    // These bits are set by both the code and data segments
    const SHARED: u64 = Self::NON_SYSTEM_SEGMENT
        | Self::GRANULARITY
        | Self::LIMIT_0_15
        | Self::LIMIT_48_51
        | Self::BASE_0_23
        | Self::BASE_24_31
        | Self::VALID
        | Self::BIT_SIZE
        | Self::READ_WRITE;

    const CODE_SEGMENT: u64 = Self::SHARED | Self::EXECUTABLE | Self::IS_CODE;
    const DATA_SEGMENT: u64 = Self::SHARED;
}