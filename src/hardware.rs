use core::panic;
use std::ops::Deref;
use std::thread;

use crate::cpu::interrupts::{Interrupt, Interrupts};
use crate::ppu::palette::Color;
use crate::utils::performance::mesure_performance;
use crate::{cartridge::Cartridge, cpu::Cpu, joypad::update_keys_status, memory::Memory, ppu::Ppu};

pub struct Hardware {
    cpu: Cpu,
    ppu: Ppu,
    ram: Memory,
    next_is_extended_instruction: bool,
}

impl Hardware {
    pub fn new(cartridge: Cartridge) -> Hardware {
        let mut ram = Memory::new();
        let cpu = Cpu::new();
        let ppu = Ppu::new();

        ram.load_rom(&cartridge.data);

        Hardware {
            cpu,
            ppu,
            ram,
            next_is_extended_instruction: false,
        }
    }

    pub fn run(&mut self) -> [Color; 160 * 144] {
        loop {
            let (elapsed_cycles, next_is_extended, _) = self
                .cpu
                .step(&mut self.ram, self.next_is_extended_instruction);

            self.next_is_extended_instruction = next_is_extended;

            update_keys_status(&mut self.ram);

            let mut buffer: Option<[Color; 160 * 144]> = None;
            for _ in 0..(elapsed_cycles * 4) {
                if let Some(interrupt) = self.ppu.step(&self.ram) {
                    Interrupts::dispatch_interrupt(interrupt, &mut self.ram);

                    if let Interrupt::VBlank = interrupt {
                        buffer = Some(self.ppu.buffer);
                    }
                }
            }

            if let Some(buffer) = buffer {
                return buffer;
            }

            self.ppu.dma_transfer(&mut self.ram);
            self.ppu.update_memory(&mut self.ram);
        }
    }
}
