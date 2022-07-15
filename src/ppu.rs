pub mod palette;
pub mod tiles;

use std::collections::{HashMap, HashSet};

use crate::cpu::interrupts::Interrupt;
use crate::lcd::{self, LcdControl};
use crate::memory::Memory;
use crate::utils::performance::mesure_performance;

use self::palette::{Color, Palette};
use self::tiles::{Sprite, Tile, TileWithColors};

const LCD_CONTROL_ADDRESS: u16 = 0xff40;
const LCD_STAT_ADDRESS: u16 = 0xff41;
const LY_ADDRESS: u16 = 0xff44;
const LYC_ADDRESS: u16 = 0xff45;
const SCY_ADDRESS: u16 = 0xff42;
const SCX_ADDRESS: u16 = 0xff43;
const WY_ADDRESS: u16 = 0xff4a;
const WX_ADDRESS: u16 = 0xff4b;

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

    pub fn step(&mut self, memory_bus: &Memory) -> Option<Interrupt> {
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
                    self.render_line(memory_bus);
                    self.mode = PpuMode::HBlank;
                }
                self.dots += 1;
                return None;
            }
        }
    }

    pub fn update_memory(&mut self, memory_bus: &mut Memory) {
        memory_bus.write(LY_ADDRESS, self.scanline);
        memory_bus.write(LCD_STAT_ADDRESS, self.create_stat_byte(memory_bus))
    }

    fn create_stat_byte(&self, memory_bus: &Memory) -> u8 {
        let ly = self.scanline;
        let lyc = memory_bus.read(LYC_ADDRESS);
        let state: u8 = match self.mode {
            PpuMode::HBlank => 0b00,
            PpuMode::VBlank => 0b01,
            PpuMode::OamSearch => 0b10,
            PpuMode::PixelTransfer => 0b11,
        };

        let ly_eq_lyc = if ly == lyc { 1 } else { 0 };

        (ly_eq_lyc << 7) | state
    }

    fn render_line(&mut self, memory_bus: &Memory) {
        let lcd_control = LcdControl::from(memory_bus.read(LCD_CONTROL_ADDRESS));
        let x_scroll = memory_bus.read(SCX_ADDRESS);
        let y_scroll = memory_bus.read(SCY_ADDRESS);
        let window_x = memory_bus.read(WX_ADDRESS);
        let window_y = memory_bus.read(WY_ADDRESS);

        let bg_shifted_scanline = self.scanline.wrapping_add(y_scroll);

        let palette = Palette::background(memory_bus);

        let bg_tile_map_area = *lcd_control.get_background_tile_map_area().start();
        let bg_tile_map_row =
            memory_bus.read_bytes::<32>(bg_tile_map_area + 32 * (bg_shifted_scanline as u16 / 8));
        let bg_tiles_in_line: HashMap<_, _> =
            Ppu::get_bg_tiles_in_row(&bg_tile_map_row, memory_bus, &palette);

        let window_tile_map_area = *lcd_control.get_window_tile_map_area().start();

        let (window_tile_map_row, window_tiles_in_line) =
            if lcd_control.window_enabled && self.scanline >= window_y {
                let window_tile_map_row = memory_bus.read_bytes::<32>(
                    window_tile_map_area + 32 * ((self.scanline - window_y) as u16 / 8),
                );
                let window_tiles_in_line: HashMap<_, _> =
                    Ppu::get_bg_tiles_in_row(&window_tile_map_row, memory_bus, &palette);

                (window_tile_map_row, window_tiles_in_line)
            } else {
                ([0; 32], HashMap::new())
            };

        let sprites_in_row = self.get_sprite_tiles_in_row(memory_bus);

        for i in 0..160 {
            let bg_shifted_dot = (i as u8).wrapping_add(x_scroll);
            let bg_tile_id = bg_tile_map_row[bg_shifted_dot as usize / 8];
            let bg_tile = bg_tiles_in_line.get(&bg_tile_id).unwrap();
            let mut bg_color = bg_tile.get_color(
                bg_shifted_dot as usize % 8,
                bg_shifted_scanline as usize % 8,
            );

            if lcd_control.window_enabled && self.scanline >= window_y && (i as u8) + 7 >= window_x
            {
                let window_shifted_dot = (i as u8) + 7 - window_x;
                let window_tile_id = window_tile_map_row[window_shifted_dot as usize / 8];
                let window_tile = window_tiles_in_line.get(&window_tile_id).unwrap();
                let window_color = window_tile.get_color(
                    window_shifted_dot as usize % 8,
                    (self.scanline - window_y) as usize % 8,
                );

                bg_color = window_color;
            }

            let sprite_color = sprites_in_row
                .iter()
                .filter(|&sprite| (sprite.x..sprite.x + 8).contains(&(i as u8 + 8)))
                .map(|sprite| {
                    sprite.get_color(
                        (i - sprite.x as usize % 8) % 8,
                        (self.scanline as usize - sprite.y as usize % 8) % 8,
                    )
                })
                .find(|&color| match color {
                    Color::Transparent => false,
                    _ => true,
                })
                .and_then(|color| match color {
                    Color::Transparent => None,
                    color => Some(color),
                });

            self.buffer[i + self.scanline as usize * 160] = match sprite_color {
                Some(color) => color,
                None => bg_color,
            }
            .clone();
        }
    }

    fn get_bg_tiles_in_row(
        tile_map_row: &[u8],
        memory_bus: &Memory,
        palette: &Palette,
    ) -> HashMap<u8, TileWithColors> {
        let lcd_control = LcdControl::from(memory_bus.read(LCD_CONTROL_ADDRESS));

        tile_map_row
            .iter()
            .cloned()
            .collect::<HashSet<_>>()
            .iter()
            .map(|x| {
                let tile = Tile::read_from(
                    memory_bus,
                    lcd_control.get_background_window_tile_address(*x),
                );
                let tile = tile.to_tile_with_colors(&palette);
                (*x, tile)
            })
            .collect()
    }

    fn get_sprite_tiles_in_row(&self, memory_bus: &Memory) -> Vec<Sprite> {
        let lcd_control = LcdControl::from(memory_bus.read(LCD_CONTROL_ADDRESS));

        if !lcd_control.object_enable {
            return vec![];
        }

        let sprites = self.get_sprites(memory_bus);
        let mut row_sprites = vec![];

        for sprite in sprites {
            if (sprite.y..sprite.y + 8).contains(&(self.scanline + 16)) {
                row_sprites.push(sprite);
            }
        }

        row_sprites
    }

    fn get_sprites(&self, memory_bus: &Memory) -> Vec<Sprite> {
        let mut sprites = vec![];
        let obp0 = Palette::obp0(memory_bus);
        let obp1 = Palette::obp1(memory_bus);

        for i in (0xfe00..0xff00).step_by(4) {
            let sprite_data = memory_bus.read_bytes::<4>(i);
            let tile = Tile::read_from(memory_bus, 0x8000 + sprite_data[2] as u16 * 16);
            let sprite = Sprite::new_from_bytes(sprite_data, tile, &obp0, &obp1);
            sprites.push(sprite);
        }

        sprites
    }
}
