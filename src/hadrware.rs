use crate::{cartridge::Cartridge, cpu::Cpu};

pub struct Hardware {
    cpu: Cpu,
}

impl Hardware {
    pub fn new(cartridge: Cartridge) -> Hardware {
        let mut cpu = Cpu::new();

        cpu.ram.load_rom(&cartridge.data);

        Hardware { cpu }
    }

    pub fn run(&mut self) {
        let debug_breakpoint: u16 = 0x0237;
        loop {
            self.cpu.step();
            if self.cpu.registers.program_counter == debug_breakpoint {
                println!("{}", self.cpu);
                self.cpu.step();
                println!("{}", self.cpu);
                break;
            }
        }
    }
}
