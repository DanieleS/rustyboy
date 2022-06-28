pub mod palette;
pub mod tiles;

use crate::cpu::interrupts::Interrupt;
use crate::memory::Memory;

use self::palette::{Color, Palette};
use self::tiles::Tile;

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
    buffer: [Color; 160 * 144],
    tile_map: [Color; 256 * 256],
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: PpuMode::HBlank,
            scanline: 0,
            dots: 0,
            last_oam_transfer: 0,
            buffer: [Color::White; 160 * 144],
            tile_map: [Color::White; 256 * 256],
        }
    }

    pub fn step(&mut self, ram: &Memory) -> Option<Interrupt> {
        match self.mode {
            PpuMode::HBlank => {
                if self.dots == 456 {
                    self.scanline += 1;
                    self.dots = 0;
                    if self.scanline == 143 {
                        self.mode = PpuMode::VBlank;
                        self.buffer = [Color::White; 160 * 144];
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
                        self.render_background_map(ram);
                        print_array(&self.tile_map, 256);
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
                if self.dots == 240 {
                    self.mode = PpuMode::HBlank;
                } else {
                    self.render_pixel();
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

    fn render_pixel(&mut self) {
        let x = (self.dots - 80) as u16;
        let y = self.scanline as u16;

        self.buffer[x as usize + y as usize * 160] = self.tile_map[x as usize + y as usize * 160];
    }

    fn render_background_map(&mut self, ram: &Memory) {
        let palette = Palette::background(ram);
        let mut tile_map = [Color::White; 256 * 256];
        for y in 0..256 {
            for x in 0..256 {
                let address = 0x9800 + (x / 8 + (y / 8) * 32);
                let tile_index = ram.read(address);
                let tile = Tile::read_from(ram, 0x8000 + tile_index as u16 * 16);
                let tile = tile.to_tile_with_colors(&palette);

                let tile_x = x % 8;
                let tile_y = y % 8;

                let color = tile.get_color(tile_x.into(), tile_y.into());
                tile_map[x as usize + y as usize * 256] = color.clone();
            }
        }

        self.tile_map = tile_map;
    }
}

fn print_array<const C: usize, T>(array: &[T; C], width: usize)
where
    T: std::fmt::Display,
{
    for i in 0..(C - 28672) {
        if i % width == 0 {
            println!();
        }
        if i % width < 200 {
            print!("{}", array[i]);
        }
    }
}
