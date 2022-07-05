pub mod palette;
pub mod tiles;

use std::collections::{HashMap, HashSet};

use crate::cpu::interrupts::Interrupt;
use crate::lcd::LcdControl;
use crate::memory::Memory;
use crate::utils::performance::mesure_performance;

use self::palette::{Color, Palette};
use self::tiles::{Tile, TileWithColors};

const LCD_CONTROL_ADDRESS: u16 = 0xff40;
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
    pub buffer: [Color; 160 * 144],
    pub tile_map: [Color; 256 * 256],
    last_oam_transfer: u16,
    frames: u32,
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
            frames: 0,
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
                        self.frames += 1;
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
                    self.render_line(ram);
                    self.mode = PpuMode::HBlank;
                }
                self.dots += 1;
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

    fn render_line(&mut self, ram: &Memory) {
        let lcd_control = LcdControl::from(ram.read(LCD_CONTROL_ADDRESS));

        let tile_map_area = *lcd_control.get_background_tile_map_area().start();
        let palette = Palette::background(ram);

        let tile_map_row = ram.read_bytes(tile_map_area + 32 * (self.scanline as u16 / 8), 32);
        let tiles_in_line: HashMap<_, _> =
            Ppu::get_tiles_in_tile_map_row(&tile_map_row, ram, &palette);

        for i in 0..160 {
            let tile_id = tile_map_row[i as usize / 8];
            let tile = tiles_in_line.get(&tile_id).unwrap();
            let color = tile.get_color(i % 8, self.scanline as usize % 8);
            self.buffer[i + self.scanline as usize * 160] = color.clone();
        }
    }

    fn get_tiles_in_tile_map_row<'a>(
        tile_map_row: &[u8],
        ram: &Memory,
        palette: &'a Palette,
    ) -> HashMap<u8, TileWithColors<'a>> {
        tile_map_row
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
            .iter()
            .map(|x| {
                let tile = Tile::read_from(ram, 0x8000 + *x as u16 * 16);
                let tile = tile.to_tile_with_colors(&palette);
                (*x, tile)
            })
            .collect()
    }
}

pub fn print_array(array: &[u8], width: usize, start_from: u16) {
    for i in 0..array.len() {
        if i % width == 0 {
            println!();
            print!("{:04x}: ", start_from + i as u16);
        }
        if i % width < 200 {
            print!("{:02x} ", array[i]);
        }
    }
}
