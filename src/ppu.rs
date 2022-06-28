pub mod palette;
pub mod tiles;

use crate::cpu::interrupts::Interrupt;
use crate::memory::Memory;

const LCD_STAT_ADDRESS: u16 = 0xff41;
const LY_ADDRESS: u16 = 0xff44;
const LYC_ADDRESS: u16 = 0xff45;

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
    last_oam_transfer: u16,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: PpuMode::HBlank,
            scanline: 0,
            dots: 0,
            last_oam_transfer: 0,
        }
    }

    pub fn step(&mut self) -> Option<Interrupt> {
        match self.mode {
            PpuMode::HBlank => {
                if self.dots == 456 {
                    self.scanline += 1;
                    self.dots = 0;
                    if self.scanline == 143 {
                        self.mode = PpuMode::VBlank;
                        return Some(Interrupt::VBlank);
                    } else {
                        self.mode = PpuMode::OamSearch;
                        return None;
                    }
                } else {
                    self.dots += 1;
                    return None;
                }
            }
            PpuMode::VBlank => {
                if self.dots == 456 {
                    self.scanline += 1;
                    self.dots = 0;
                    if self.scanline == 154 {
                        self.scanline = 0;
                        self.mode = PpuMode::OamSearch;
                    }
                } else {
                    self.dots += 1;
                }
                return None;
            }
            PpuMode::OamSearch => {
                if self.dots == 80 {
                    self.mode = PpuMode::PixelTransfer;
                } else {
                    self.dots += 1;
                }
                return None;
            }
            PpuMode::PixelTransfer => {
                if self.dots == 152 {
                    self.mode = PpuMode::HBlank;
                } else {
                    self.dots += 1;
                }
                return None;
            }
        }
    }

    pub fn update_memory(&mut self, ram: &mut Memory) {
        ram.write(LY_ADDRESS, self.scanline);
        ram.write(LCD_STAT_ADDRESS, self.create_stat_byte(ram))
    }

    pub fn dma_transfer(&mut self, ram: &mut Memory) {
        let mut address = ram.read(0xff46) as u16;

        if self.last_oam_transfer != address {
            address <<= 8;

            for i in 0..0xa0 {
                let byte = ram.read(address as u16 + i as u16);
                ram.write(0xfe00 + i as u16, byte);
            }

            self.last_oam_transfer = address;
        }
    }

    fn create_stat_byte(&self, ram: &Memory) -> u8 {
        let ly = self.scanline;
        let lyc = ram.read(LYC_ADDRESS);
        let state: u8 = match self.mode {
            PpuMode::HBlank => 0b00,
            PpuMode::VBlank => 0b01,
            PpuMode::OamSearch => 0b10,
            PpuMode::PixelTransfer => 0b11,
        };

        let ly_eq_lyc = if ly == lyc { 1 } else { 0 };

        (ly_eq_lyc << 7) | state
    }
}
