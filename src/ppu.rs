pub mod palette;
pub mod renderer;
pub mod tiles;

use crate::cpu::interrupts::{Interrupt, Interrupts};
use crate::lcd::LcdStat;
use crate::memory::Memory;

use self::palette::Color;

const LCD_STAT_ADDRESS: u16 = 0xff41;
const LY_ADDRESS: u16 = 0xff44;
const LYC_ADDRESS: u16 = 0xff45;

#[derive(Clone, Copy)]
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
        let lcd_stat = LcdStat::from(memory_bus.read(LCD_STAT_ADDRESS));

        if lcd_stat.lyc_eq_ly_interrupt_source && lcd_stat.lyc_eq_ly {
            Interrupts::dispatch_interrupt(Interrupt::LcdStat, memory_bus);
        }

        match self.mode {
            PpuMode::HBlank => {
                if self.dots == 456 {
                    self.scanline += 1;
                    self.dots = 0;
                    if self.scanline == 143 {
                        self.mode = PpuMode::VBlank;
                        Interrupts::dispatch_interrupt(Interrupt::VBlank, memory_bus);
                        if lcd_stat.mode_1_interrupt_source {
                            Interrupts::dispatch_interrupt(Interrupt::LcdStat, memory_bus);
                        }
                        return true;
                    } else {
                        self.mode = PpuMode::OamSearch;
                        if lcd_stat.mode_2_interrupt_source {
                            Interrupts::dispatch_interrupt(Interrupt::LcdStat, memory_bus);
                        }
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
                        if lcd_stat.mode_2_interrupt_source {
                            Interrupts::dispatch_interrupt(Interrupt::LcdStat, memory_bus);
                        }
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
                    if lcd_stat.mode_0_interrupt_source {
                        Interrupts::dispatch_interrupt(Interrupt::LcdStat, memory_bus);
                    }
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
        let mut lcd_stat = LcdStat::from(memory_bus.read(LCD_STAT_ADDRESS));
        let lyc = memory_bus.read(LYC_ADDRESS);
        lcd_stat.lyc_eq_ly = self.scanline == lyc;
        lcd_stat.mode = self.mode;

        u8::from(lcd_stat)
    }
}
