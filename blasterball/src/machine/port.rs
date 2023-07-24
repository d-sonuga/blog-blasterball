use core::arch::asm;

// An IO port
pub struct Port(u16);

impl Port {
    // Create a new port
    pub const fn new(port_no: u16) -> Self {
        Self(port_no)
    }

    // Read from the port
    pub fn read(&self) -> u8 {
        let value: u8;
        unsafe {
            asm!("in al, dx", out("al") value, in("dx") self.0);
        }
        value
    }

    // Write to the port
    pub fn write(&mut self, value: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.0, in("al") value);
        }
    }
}