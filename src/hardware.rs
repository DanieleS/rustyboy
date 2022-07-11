use crate::cpu::interrupts::{Interrupt, Interrupts};
use crate::joypad::{JoypadKey, JoypadState};
use crate::ppu::palette::Color;
use crate::{cartridge::Cartridge, cpu::Cpu, memory::Memory, ppu::Ppu};

pub struct Hardware {
    cpu: Cpu,
    ppu: Ppu,
    ram: Memory,
    joypad: JoypadState,
    next_is_extended_instruction: bool,
}

impl Hardware {
    pub fn new(cartridge: Cartridge) -> Hardware {
        let mut ram = Memory::new();
        let cpu = Cpu::new();
        let ppu = Ppu::new();
        let joypad = JoypadState::new();

        ram.load_rom(&cartridge.data);

        Hardware {
            cpu,
            ppu,
            ram,
            joypad,
            next_is_extended_instruction: false,
        }
    }

    pub fn run(&mut self) -> [Color; 160 * 144] {
        let mut trace = false;
        let mut instructions = 10;

        loop {
            let (elapsed_cycles, next_is_extended, _) = self
                .cpu
                .step(&mut self.ram, self.next_is_extended_instruction);

            self.next_is_extended_instruction = next_is_extended;

            self.joypad.update_keys_status(&mut self.ram);

            let mut buffer: Option<[Color; 160 * 144]> = None;
            for _ in 0..(elapsed_cycles * 4) {
                if let Some(interrupt) = self.ppu.step(&self.ram) {
                    Interrupts::dispatch_interrupt(interrupt, &mut self.ram);

                    if let Interrupt::VBlank = interrupt {
                        buffer = Some(self.ppu.buffer);
                    }
                }
            }

            if self.ram.io_registers.timer_step(elapsed_cycles as i8) {
                Interrupts::dispatch_interrupt(Interrupt::Timer, &mut self.ram);
            }

            self.ppu.dma_transfer(&mut self.ram);
            self.ppu.update_memory(&mut self.ram);

            if let Some(buffer) = buffer {
                return buffer;
            }
        }
    }

    pub fn button_pressed(&mut self, button: JoypadKey) {
        self.joypad.set_key_pressed(button);
    }

    pub fn button_released(&mut self, button: JoypadKey) {
        self.joypad.set_key_released(button);
    }
}

impl Drop for Hardware {
    fn drop(&mut self) {
        println!("CPU: {}", self.cpu.registers);
    }
}
