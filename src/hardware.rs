use crate::cpu::interrupts::{Interrupt, Interrupts};
use crate::joypad::{JoypadKey, JoypadState};
use crate::ppu::palette::Color;
use crate::utils::array::print_array_16;
use crate::{cartridge::Cartridge, cpu::Cpu, memory::Memory, ppu::Ppu};

pub struct Hardware {
    cpu: Cpu,
    ppu: Ppu,
    memory_bus: Memory,
    joypad: JoypadState,
}

impl Hardware {
    pub fn new(cartridge: Cartridge) -> Hardware {
        let mut memory_bus = Memory::new();
        let cpu = Cpu::new();
        let ppu = Ppu::new();
        let joypad = JoypadState::new();

        memory_bus.load_rom(&cartridge.data);

        Hardware {
            cpu,
            ppu,
            memory_bus,
            joypad,
        }
    }

    pub fn run(&mut self) -> [Color; 160 * 144] {
        let mut trace = false;
        let mut instructions = 10;

        loop {
            let (elapsed_cycles, _) = self.cpu.step(&mut self.memory_bus);

            self.joypad.update_keys_status(&mut self.memory_bus);

            let mut buffer: Option<[Color; 160 * 144]> = None;
            for _ in 0..(elapsed_cycles * 4) {
                if let Some(interrupt) = self.ppu.step(&self.memory_bus) {
                    Interrupts::dispatch_interrupt(interrupt, &mut self.memory_bus);

                    if let Interrupt::VBlank = interrupt {
                        buffer = Some(self.ppu.buffer);
                    }
                }
            }

            if self
                .memory_bus
                .io_registers
                .timer_step(elapsed_cycles as i8)
            {
                Interrupts::dispatch_interrupt(Interrupt::Timer, &mut self.memory_bus);
            }

            self.ppu.dma_transfer(&mut self.memory_bus);
            self.ppu.update_memory(&mut self.memory_bus);

            // println!("{}", self.cpu.registers);

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
        print_array_16(&self.cpu.last_pc, 16);
        println!();
        println!("{:?}", self.memory_bus.work_ram);
    }
}
