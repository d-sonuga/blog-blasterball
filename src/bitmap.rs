// The first section of a file in the BMP file format
#[repr(C, packed)]
pub struct FileHeader {
    // The first 2 bytes in the file which are always 0x42 and 0x4d, "BM" in ASCII
    pub bmp_id: [u8; 2],
    // The size of the whole BMP file in bytes
    pub file_size: u32,
    // These bytes are reserved
    reserved: [u8; 4],
    // The offset into the file of the starting byte of the bitmap pixel array
    pub pixel_array_offset: u32
}

// Gives some information about the BMP file
#[repr(C, packed)]
pub struct DIBHeader {
    // The size of the DIB header itself
    pub size_of_self: u32,
    // The width of the image
    pub image_width: u32,
    // The height of the image
    pub image_height: u32,
    // A number that is always 1
    pub always_1: u16,
    // The number of bits used to define a single pixel in the
    // bitmap's pixel array.
    // This representation of the bitmap assumes that the value is always 8
    pub bits_per_pixel: u16,
    // We aren't going to use these fields
    unneeded1: [u8; 104],
    // These bytes are reserved
    reserved: [u8; 4]
}

// A color in the color table
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Color {
    // The blue intensity of the color
    pub blue: u8,
    // The green intensity of the color
    pub green: u8,
    // The red intensity of the color
    pub red: u8,
    // These bits are reserved
    pub reserved: u8
}

#[repr(transparent)]
pub struct ColorTable(pub [Color; 256]);