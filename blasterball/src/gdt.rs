use core::mem;
use core::arch::asm;
use crate::tss::TSS;

// An artificial limit placed on the number of entries that can be placed
// in the GDT's descriptor array, for convenience
const MAX_NO_OF_ENTRIES: usize = 8;

// The Global Descriptor Table
#[repr(C, align(8))]
pub struct GDT {
    // The segment descriptors
    descriptors: [u64; MAX_NO_OF_ENTRIES],
    // The next index available to place a descriptor in the GDT
    next_index: usize
}

impl GDT {
    // Creates a new GDT
    pub fn new() -> Self {
        Self {
            descriptors: [0; MAX_NO_OF_ENTRIES],
            // Start inserting at index 1 to keep the first entry
            // as a null descriptor
            next_index: 1
        }
    }

    // Adds a segment descriptor to the descriptors array
    pub fn add_descriptor(&mut self, descriptor: Descriptor) -> Result<SegmentSelector, &'static str> {
        match descriptor {
            Descriptor::NonSystem(value) => {
                // Is array full?
                if self.next_index >= self.descriptors.len() {
                    return Err("no enough space for descriptor");
                }
                self.descriptors[self.next_index] = value;
                self.next_index += 1;
                Ok(SegmentSelector::new(self.next_index as u16 - 1))
            }
            Descriptor::System(higher, lower) => {
                // Is there enough space for a system descriptor?
                if self.next_index + 1 >= self.descriptors.len() {
                    return Err("No enough space for descriptor");
                }
                self.descriptors[self.next_index] = lower;
                self.descriptors[self.next_index + 1] = higher;
                self.next_index += 2;
                Ok(SegmentSelector::new(self.next_index as u16 - 2))
            }
        }
    }

    // Creates a descriptor table pointer that tells to tell
    // the processor where the GDT is located
    pub fn as_pointer(&'static self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            base: self as *const _ as *const u8,
            limit: (mem::size_of::<Self>() - 1) as u16
        }
    }

    // Loads the GDT in the GDTR register
    pub fn load(&self, pointer: &DescriptorTablePointer) {
        unsafe {
            asm!("lgdt [{}]", in(reg) pointer);
        }
    }
}

// A segment descriptor
pub enum Descriptor {
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
    const BIT_SIZE: u64 = 1 << 54;
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

    pub fn code_segment() -> Self {
        Self::NonSystem(Self::CODE_SEGMENT)
    }

    pub fn data_segment() -> Self {
        Self::NonSystem(Self::DATA_SEGMENT)
    }

    pub fn tss_segment(tss: &'static TSS) -> Self {
        let mut upper_descriptor_value = 0u64;
        let mut lower_descriptor_value = 0u64;

        let limit = mem::size_of::<TSS>() - 1;
        // Retaining only the lower 16 bits of the limit
        let limit_lower_16_bits = limit & 0xffff;
        // Setting the descriptor's first 16 bits to the limit's lower 16 bits
        lower_descriptor_value = lower_descriptor_value | (limit_lower_16_bits as u64);

        let base = tss as *const _ as u64;
        // Retaining only the lower 24 bits of the base
        let base_lower_24_bits = base & 0xffffff;
        // Setting bits 16..=39 of the descriptor to the lower 24 bits of the base
        lower_descriptor_value = lower_descriptor_value | (base_lower_24_bits << 16);

        let mut access_byte = 0u8;
        // Setting the first 4 bits of the access byte to 1001, the indicator
        // of a TSS
        access_byte = access_byte | 0b1001;
        // Setting bit 7 of the access byte
        access_byte = access_byte | (1 << 7);
        // Setting the access byte to bits 40..=47 of the descriptor
        lower_descriptor_value = lower_descriptor_value | ((access_byte as u64) << 40);

        // NEW:
        // Shifting out the lower 16 bits of the limit and retaining only the upper 4 bits
        let limit_upper_4_bits = (limit >> 16) & 0xf;
        // Setting the upper 4 bits of the limit to bit 48..=51 of the descriptor
        lower_descriptor_value = lower_descriptor_value | ((limit_upper_4_bits as u64) << 48);
        
        // Setting bit 55 of the descriptor for a granularity of 1-Kib units
        lower_descriptor_value = lower_descriptor_value | (1 << 55);

        // NEW:
        // Shifting out the lower 24 bits and retaining only the upper 40
        let base_upper_40_bits = (base >> 24) & 0xffffffffff;
        // Retaining only the lower 8 bits of the base's upper 40 bits
        let base_lower_8_of_upper_40 = base_upper_40_bits & 0xff;
        // Shifting out the lower 8 bits and
        // retaining only the upper 32 bits of the base's upper 40 bits
        let base_upper_32_of_upper_40 = (base_upper_40_bits >> 8) & 0xffffffff;
        // Setting the lower 8 bits of the base's upper 40 bits to bits 56..=63 of the descrtiptor
        lower_descriptor_value = lower_descriptor_value | (base_lower_8_of_upper_40 << 56);
        // Setting the upper 32 bits of the base's upper 40 bits to bits 64..=95 of the descriptor
        upper_descriptor_value = upper_descriptor_value | base_upper_32_of_upper_40;

        Self::System(upper_descriptor_value, lower_descriptor_value)
    }
}

// Index into a GDT
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    // Create a segment selector from an index
    fn new(index: u16) -> Self {
        Self(index << 3)
    }
}

// Tells the processor where a descriptor table is located
#[repr(C, packed)]
pub struct DescriptorTablePointer {
    // Size of the descriptor table - 1
    pub limit: u16,
    // The starting address of the descriptor table
    pub base: *const u8
}

pub trait SegmentRegister {
    fn set(&self, selector: SegmentSelector);
}

// The data segment register
pub struct DS;

// The stack segment register
pub struct SS;

// The code segment register
pub struct CS;

impl SegmentRegister for DS {
    fn set(&self, selector: SegmentSelector) {
        unsafe { asm!("mov ds, ax", in("ax") selector.0); }
    }
}

impl SegmentRegister for SS {
    fn set(&self, selector: SegmentSelector) {
        unsafe { asm!("mov ds, ax", in("ax") selector.0); }
    }
}

impl SegmentRegister for CS {
    fn set(&self, selector: SegmentSelector) {
        unsafe {
            asm!(
                "push {sel:r}",
                "lea {tmp}, [1f + rip]",
                "push {tmp}",
                "retfq",
                "1:",
                sel = in(reg) selector.0,
                tmp = lateout(reg) _
            );
        }
    }
}