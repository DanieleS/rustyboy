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
        self.cpu.step(); // Nop
        self.cpu.step(); // JP 1
        self.cpu.step(); // JP 2
        self.cpu.step(); // xor a
        self.cpu.step(); // ld hl
        self.cpu.step(); // c 10
        self.cpu.step(); // b 0
        self.cpu.step(); // ldd [hl], a
        self.cpu.step(); // dec b
    }
}
