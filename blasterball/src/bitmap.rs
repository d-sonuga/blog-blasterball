use crate::{Screen, Pixel};

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

// A structure for handling bitmaps
pub struct Bitmap {
    // The bitmap's file header
    pub file_header: &'static FileHeader,
    // The bitmap's DIB header
    pub dib_header: &'static DIBHeader,
    // The bitmap's color table
    pub color_table: &'static ColorTable,
    // The bitmap's pixel array
    pub pixel_array: &'static [u8]
}

impl Bitmap {
    // Initializes the Bitmap structure from the bitmap bytes
    pub fn new(bitmap_bytes: &'static [u8]) -> Self {
        // Retrieving a pointer to the block.bmp's bytes
        let bitmap_bytes_ptr: *const u8 = bitmap_bytes.as_ptr();
        // Reinterpreting a pointer to bytes as a pointer to a FileHeader instance
        let file_header_ptr = bitmap_bytes_ptr as *const FileHeader;
        // Interpreting the first section of the bitmap as the file header
        let file_header = unsafe { &(*file_header_ptr) };
        // The number of bytes that make up the FileHeader
        const FILE_HEADER_SIZE: usize = core::mem::size_of::<FileHeader>();
        // The DIB header comes immediately after the file header
        const DIB_HEADER_OFFSET: isize = FILE_HEADER_SIZE as isize;
        // Reinterpreting a pointer to the bytes at offset DIB_HEADER_OFFSET
        // as a pointer to the DIB header
        let dib_header_ptr = unsafe { bitmap_bytes_ptr.offset(DIB_HEADER_OFFSET) as *const DIBHeader };
        // Interpreting the second section of the bitmap as the DIB header
        let dib_header = unsafe { &(*dib_header_ptr) };
        // The number of bytes that make up the DIB header
        const DIB_HEADER_SIZE: usize = core::mem::size_of::<DIBHeader>();
        // The color table comes immediately after the file header and the DIB header
        const COLOR_TABLE_OFFSET: isize = (FILE_HEADER_SIZE + DIB_HEADER_SIZE) as isize;
        // Reinterpreting a pointer to the bytes at offset COLOR_TABLE_OFFSET as a pointer
        // to the color table
        let color_table_ptr = unsafe { bitmap_bytes_ptr.offset(COLOR_TABLE_OFFSET) as *const ColorTable };
        // Interpreting the bytes at `COLOR_TABLE_OFFSET` as the color table
        let color_table = unsafe { &(*color_table_ptr) };

        // Get a slice to the bitmap's pixel array 
        let pixel_array = &bitmap_bytes[file_header.pixel_array_offset as usize..];

        Self {
            file_header,
            dib_header,
            color_table,
            pixel_array
        }
    }
 
    // Returns the height of the image
    pub fn height(&self) -> usize {
        // Retrieving the image height from the DIB header
        // Casting it as a usize so that it can be used to index into arrays and slices
        self.dib_header.image_height as usize
    }

    // Returns the width of the image
    pub fn width(&self) -> usize {
        // Retrieving the image width from the DIB header
        // Casting it as a usize so that it can be used to index into arrays and slices
        self.dib_header.image_width as usize
    }
}

// Draws the bitmap on the screen
pub fn draw_bitmap(screen: &mut Screen, bitmap: &Bitmap, pos: (usize, usize)) {
    for row in 0..bitmap.height() {
        for col in 0..bitmap.width() {
            // The image is upside down in the pixel array,
            // so the pixel array's rows have to retrieved from the bottom up
            let inverted_row = bitmap.height() - row - 1;
            let color_table_index = bitmap.pixel_array[inverted_row * bitmap.width() + col];
            let color = bitmap.color_table.0[color_table_index as usize];
            screen.pixels[row + pos.0][col + pos.1] = Pixel {
                red: color.red,
                green: color.green,
                blue: color.blue,
                reserved: 0
            };
        }
    }
}

// Erase a bitmap from the screen
pub fn erase_bitmap(screen: &mut Screen, bitmap: &Bitmap, pos: (usize, usize)) {
    for row in 0..bitmap.width() {
        for col in 0..bitmap.height() {
            // Blacking out the bitmap on screen
            // A color value of all 0s is black
            screen.pixels[row + pos.0][col + pos.1] = Pixel {
                red: 0,
                green: 0,
                blue: 0,
                reserved: 0
            };
        }
    }
}