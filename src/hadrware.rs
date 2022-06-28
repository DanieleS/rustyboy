use crate::cpu::interrupts::{Interrupt, Interrupts};
use crate::ppu::palette::Palette;
use crate::ppu::tiles::Tile;
use crate::{cartridge::Cartridge, cpu::Cpu, joypad::update_keys_status, memory::Memory, ppu::Ppu};

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
        let debug_breakpoint: u16 = 0xffbc;

        let mut next_is_extended_instruction = false;
        let mut frame_count = 0;

        loop {
            let (elapsed_cycles, next_is_extended) =
                self.cpu.step(&mut self.ram, next_is_extended_instruction);

            update_keys_status(&mut self.ram);

            next_is_extended_instruction = next_is_extended;

            for _ in 0..(elapsed_cycles * 4) {
                if let Some(interrupt) = self.ppu.step(&self.ram) {
                    Interrupts::dispatch_interrupt(interrupt, &mut self.ram);

                    if let Interrupt::VBlank = interrupt {
                        frame_count += 1;
                    }
                }
            }

            self.ppu.dma_transfer(&mut self.ram);
            self.ppu.update_memory(&mut self.ram);

            if
            /* self.cpu.registers.program_counter == debug_breakpoint */
            frame_count == 30 {
                println!("{}", self.cpu);

                // println!("{:?}", self.ram.vram);

                self.cpu.step(&mut self.ram, next_is_extended_instruction);
                println!("{}", self.cpu);
                break;
            }
        }
    }

    fn print_nintendo_logo(&mut self) {
        let palette = Palette::default();
        let mut nintendo_logo: Vec<String> = Vec::new();
        for i in (0x8340..0x83a0).step_by(0x10) {
            let tile = Tile::read_from(&self.ram, i);
            let tile_with_colors = tile.to_tile_with_colors(&palette);
            nintendo_logo.push(tile_with_colors.to_string());
        }
        for row in 0..8 {
            for tile in &nintendo_logo {
                let rows: Vec<_> = tile.split("\n").collect();
                print!("{}", rows[row]);
            }
            print!("\n");
        }
    }
}
