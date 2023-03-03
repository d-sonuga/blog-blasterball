use crate::port::Port;

const CMD_INIT: u8 = 0x11;
const MODE_8086: u8 = 0x01;
const END_OF_INTERRUPT: u8 = 0x20;


// A Programmable Interrupt Controller
struct PIC {
    // The base index in the IDT that the PIC's interrupts are mapped to
    offset: u8,
    // PIC's command port
    command: Port,
    // PIC's data port
    data: Port
}

// The chained PICs that map hardware interrupts to interrupt
// vector numbers for the CPU
pub struct PICs {
    first: PIC,
    second: PIC
}

impl PICs {
    pub fn new() -> PICs {
        let first = PIC {
            offset: 32,
            command: Port::new(0x20),
            data: Port::new(0x21)
        };
        let second = PIC {
            offset: 32 + 8,
            command: Port::new(0xa0),
            data: Port::new(0xa1)
        };
        PICs {
            first,
            second
        }
    }

    // Set up the PICs
    pub fn init(&mut self) {
        // Garbage port for waiting
        let mut wait_port = Port::new(0x80);
        let mut wait = || wait_port.write(0);
        
        // Start initialization sequence
        self.first.command.write(CMD_INIT);
        wait();
        self.second.command.write(CMD_INIT);
        wait();

        // Setup base offsets
        self.first.data.write(self.first.offset);
        wait();
        self.second.data.write(self.second.offset);
        wait();

        // Tell first that there is a second PIC at IRQ 2
        self.first.data.write(4);
        wait();
        // Tell the second PIC it's connected to the first's line 2
        self.second.data.write(2);
        wait();

        // Telling the PICs about the environment they're in
        self.first.data.write(MODE_8086);
        wait();
        self.second.data.write(MODE_8086);
        wait();

        // Setting the interrupt masks
        // Line 0 on the first PIC is for the timer,
        // so setting it to 0
        self.first.data.write(0b11111110);
        self.second.data.write(0b11111111);
    }

    // Sends the EOI command to the PICs in charge of an interrupt
    // that was just handled
    pub fn end_of_interrupt(&mut self, number: u8) {
        self.first.command.write(END_OF_INTERRUPT);
        if number >= 8 {
            self.second.command.write(END_OF_INTERRUPT);    
        }
    }
}