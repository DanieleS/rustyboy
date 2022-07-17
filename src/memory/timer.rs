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
        io_memory_bank_data: &mut GeneralPourposeMemoryBank<0x7f>,
    ) -> bool {
        self.div_cycle(cycles, io_memory_bank_data);

        self.timer_cycle(cycles, io_memory_bank_data)
    }

    fn div_cycle(&mut self, cycles: i8, io_memory_bank_data: &mut GeneralPourposeMemoryBank<0x7f>) {
        self.div_cycles -= cycles;
        if self.div_cycles < 0 {
            io_memory_bank_data.write(
                DIV_ADDRESS,
                io_memory_bank_data.read(DIV_ADDRESS).wrapping_add(1),
            );

            self.div_cycles += 64;
        }
    }

    fn timer_cycle(
        &mut self,
        cycles: i8,
        io_memory_bank_data: &mut GeneralPourposeMemoryBank<0x7f>,
    ) -> bool {
        let tac = io_memory_bank_data.read(TAC_ADDRESS);
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

                let tima = io_memory_bank_data.read(TIMA_ADDRESS);
                let (new_tima, overflow) = tima.overflowing_add(1);
                if overflow {
                    let tma = io_memory_bank_data.read(TMA_ADDRESS);
                    io_memory_bank_data.write(TIMA_ADDRESS, tma);
                    return true;
                } else {
                    io_memory_bank_data.write(TIMA_ADDRESS, new_tima)
                }
            }
        }

        false
    }
}
