use super::{GeneralPourposeMemoryBank, MemoryBank};

pub const DIV_ADDRESS: u16 = 0xff04;
pub const TIMA_ADDRESS: u16 = 0xff05;
pub const TMA_ADDRESS: u16 = 0xff06;
pub const TAC_ADDRESS: u16 = 0xff07;

pub struct Timer {
    div_cycles: i8,
    tma_cycles: i16,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div_cycles: 64,
            tma_cycles: 1024,
        }
    }
}

impl Timer {
    pub fn tick(
        &mut self,
        cycles: i8,
        io_memory_bank: &mut GeneralPourposeMemoryBank<0x7f>,
    ) -> bool {
        self.div_cycles -= cycles;
        if self.div_cycles < 0 {
            io_memory_bank.write(
                DIV_ADDRESS,
                io_memory_bank.read(DIV_ADDRESS).wrapping_add(1),
            );

            self.div_cycles += 64;
        }

        let tac = io_memory_bank.read(TAC_ADDRESS);
        if tac & 0x04 == 0x04 {
            let cycles_mult = match tac & 0b11 {
                0b00 => 1,
                0b01 => 64,
                0b10 => 16,
                0b11 => 4,
                _ => unreachable!(),
            };
            self.tma_cycles -= cycles as i16 * cycles_mult;
            if self.tma_cycles < 0 {
                self.tma_cycles += 1024;

                let tima = io_memory_bank.read(TIMA_ADDRESS);
                let (new_tima, overflow) = tima.overflowing_add(1);
                if overflow {
                    let tac = io_memory_bank.read(TAC_ADDRESS);
                    io_memory_bank.write(TIMA_ADDRESS, tac);
                    return true;
                } else {
                    io_memory_bank.write(TIMA_ADDRESS, new_tima)
                }
            }
        }

        false
    }
}
