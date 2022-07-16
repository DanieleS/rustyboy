pub mod palette;
pub mod renderer;
pub mod tiles;

use crate::cpu::interrupts::{Interrupt, Interrupts};
use crate::memory::Memory;

use self::palette::Color;

const LCD_STAT_ADDRESS: u16 = 0xff41;
const LY_ADDRESS: u16 = 0xff44;

pub enum PpuMode {
    HBlank,
    VBlank,
    OamSearch,
    PixelTransfer,
}

pub struct Ppu {
    pub mode: PpuMode,
    pub scanline: u8,
    pub dots: u16,
    pub buffer: [Color; 160 * 144],
    pub tile_map: [Color; 256 * 256],
    frames: u32,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: PpuMode::HBlank,
            scanline: 0,
            dots: 0,
            buffer: [Color::White; 160 * 144],
            tile_map: [Color::White; 256 * 256],
            frames: 0,
        }
    }

    /// Step the PPU.
    ///
    /// Returns `true` if the Frame buffer is ready to be read.
    /// Returns `false` if the PPU is still in the middle of a frame.
    pub fn step(&mut self, memory_bus: &mut Memory) -> bool {
        match self.mode {
            PpuMode::HBlank => {
                if self.dots == 456 {
                    self.scanline += 1;
                    self.dots = 0;
                    if self.scanline == 143 {
                        self.mode = PpuMode::VBlank;
                        Interrupts::dispatch_interrupt(Interrupt::VBlank, memory_bus);
                        return true;
                    } else {
                        self.mode = PpuMode::OamSearch;
                        return false;
                    }
                } else {
                    self.dots += 1;
                    return false;
                }
            }
            PpuMode::VBlank => {
                if self.dots == 456 {
                    self.scanline += 1;
                    self.dots = 0;
                    if self.scanline == 154 {
                        self.frames += 1;
                        self.scanline = 0;
                        self.mode = PpuMode::OamSearch;
                    }
                } else {
                    self.dots += 1;
                }
                return false;
            }
            PpuMode::OamSearch => {
                if self.dots == 80 {
                    self.mode = PpuMode::PixelTransfer;
                } else {
                    self.dots += 1;
                }
                return false;
            }
            PpuMode::PixelTransfer => {
                if self.dots == 240 {
                    self.render_line(memory_bus);
                    self.mode = PpuMode::HBlank;
                }
                self.dots += 1;
                return false;
            }
        }
    }

    pub fn update_memory(&mut self, memory_bus: &mut Memory) {
        memory_bus.write(LY_ADDRESS, self.scanline);
        memory_bus.write(LCD_STAT_ADDRESS, self.create_stat_byte(memory_bus))
    }

    fn create_stat_byte(&self, memory_bus: &Memory) -> u8 {
        let lcd_stat = memory_bus.read(LCD_STAT_ADDRESS);

        let state: u8 = match self.mode {
            PpuMode::HBlank => 0b00,
            PpuMode::VBlank => 0b01,
            PpuMode::OamSearch => 0b10,
            PpuMode::PixelTransfer => 0b11,
        };

        lcd_stat | state
    }
}
