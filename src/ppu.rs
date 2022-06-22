pub mod palette;
pub mod tiles;

use crate::memory::Memory;

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
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: PpuMode::HBlank,
            scanline: 0,
            dots: 0,
        }
    }

    pub fn step(&mut self) {
        match self.mode {
            PpuMode::HBlank => {
                if self.dots == 456 {
                    self.scanline += 1;
                    self.dots = 0;
                    if self.scanline == 143 {
                        self.mode = PpuMode::VBlank;
                    } else {
                        self.mode = PpuMode::OamSearch;
                    }
                } else {
                    self.dots += 1;
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
            }
            PpuMode::OamSearch => {
                if self.dots == 80 {
                    self.mode = PpuMode::PixelTransfer;
                } else {
                    self.dots += 1;
                }
            }
            PpuMode::PixelTransfer => {
                if self.dots == 152 {
                    self.mode = PpuMode::HBlank;
                } else {
                    self.dots += 1;
                }
            }
        }
    }

    pub fn update_memory(&mut self, ram: &mut Memory) {
        ram.write(0xff44, self.scanline);
    }
}
