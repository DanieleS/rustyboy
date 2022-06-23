use crate::{
    cartridge::Cartridge,
    cpu::Cpu,
    lcd::LcdControl,
    memory::Memory,
    ppu::Ppu,
    ppu::{
        palette::{Palette, PaletteType},
        tiles::Tile,
    },
};

pub struct Hardware {
    cpu: Cpu,
    ppu: Ppu,
    ram: Memory,
}

impl Hardware {
    pub fn new(cartridge: Cartridge) -> Hardware {
        let mut ram = Memory::new();
        let cpu = Cpu::new();
        let ppu = Ppu::new();

        ram.load_rom(&cartridge.data);

        Hardware { cpu, ppu, ram }
    }

    pub fn run(&mut self) {
        let debug_breakpoint: u16 = 0xffba;
        loop {
            let elapsed_cycles = self.cpu.step(&mut self.ram);

            for _ in 0..(elapsed_cycles * 4) {
                self.ppu.step();
            }

            self.ppu.update_memory(&mut self.ram);

            if self.cpu.registers.program_counter == debug_breakpoint {
                println!("{}", self.cpu);
                println!("{:?}", self.ram.io_registers);
                self.cpu.step(&mut self.ram);
                println!("{}", self.cpu);
                break;
            }
        }
    }
}
