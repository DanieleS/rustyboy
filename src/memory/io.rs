use super::{
    timer::{Timer, DIV_ADDRESS},
    GeneralPourposeMemoryBank, Memory, MemoryBank,
};

const JOYP_ADDRESS: u16 = 0xff00;
pub const DMA_ADDRESS: u16 = 0xff46;

pub struct IOMemoryBank {
    joyp: u8,
    data: GeneralPourposeMemoryBank<0x7f>,
    timer: Timer,
    pub dma_transfer_requested: bool,
}

impl IOMemoryBank {
    pub fn new() -> Self {
        Self {
            joyp: 0xff,
            data: GeneralPourposeMemoryBank::new(0xFF01),
            timer: Timer::new(),
            dma_transfer_requested: false,
        }
    }
}

impl IOMemoryBank {
    pub fn timer_step(&mut self, cycles: i8) -> bool {
        self.timer.tick(cycles, &mut self.data)
    }
}

impl MemoryBank for IOMemoryBank {
    fn read(&self, address: u16) -> u8 {
        match address {
            JOYP_ADDRESS => self.joyp,
            DIV_ADDRESS => self.data.read(DIV_ADDRESS),
            _ => self.data.read(address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            JOYP_ADDRESS => self.joyp = handle_joyp_write(self.joyp, value),
            DIV_ADDRESS => self.data.write(DIV_ADDRESS, 0x00),
            DMA_ADDRESS => {
                self.data.write(DMA_ADDRESS, value);
                self.dma_transfer_requested = true;
            }
            _ => self.data.write(address, value),
        }
    }
}

fn handle_joyp_write(old_value: u8, value: u8) -> u8 {
    if value & 0b0011_0000 != 0 {
        // Writing upper nibble
        (old_value & 0b0000_1111) | (value & 0b0011_0000) | 0xc0
    } else {
        // Writing lower nibble
        (old_value & 0b1111_0000) | (value & 0b0000_1111)
    }
}
