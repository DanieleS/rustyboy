use crate::memory::Memory;

use super::palette::{Color, Palette, SpritePalette};

#[derive(Debug)]
pub struct Tile {
    pub data: [u8; 16],
}

#[derive(Debug)]
pub struct TileWithColors {
    pixels: [Color; 64],
}

#[derive(Debug)]
pub struct Sprite {
    pub tile: TileWithColors,
    pub x: u8,
    pub y: u8,
    pub sprite_flags: SpriteFlags,
}

#[derive(Debug)]
pub struct SpriteFlags {
    pub bg_and_window_over: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub palette_number: SpritePalette,
}

impl Tile {
    pub fn read_from(ram: &Memory, address: u16) -> Tile {
        let mut data = [0; 16];
        for i in 0..16 {
            data[i] = ram.read(address + i as u16);
        }
        Tile { data }
    }

    pub fn to_tile_with_colors(&self, palette: &Palette) -> TileWithColors {
        let mut pixels = [Color::Black; 64];
        let palette = palette.clone();

        let even_bytes: Vec<_> = self
            .data
            .iter()
            .enumerate()
            .filter(|(i, _)| *i % 2 == 0)
            .map(|(_, v)| v)
            .collect();

        let odd_bytes: Vec<_> = self
            .data
            .iter()
            .enumerate()
            .filter(|(i, _)| *i % 2 == 1)
            .map(|(_, v)| v)
            .collect();

        let bytes_zip: Vec<_> = odd_bytes.iter().zip(even_bytes.iter()).collect();

        for i in 0..64 {
            let first_bit = ((*bytes_zip[i / 8].0) & (1 << (7 - (i % 8)))) != 0;
            let last_bit = ((*bytes_zip[i / 8].1) & (1 << (7 - (i % 8)))) != 0;

            let color = palette.color_from_bits(first_bit, last_bit).clone();

            pixels[i] = color;
        }

        TileWithColors { pixels }
    }
}

impl TileWithColors {
    pub fn get_color(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y * 8 + x]
    }
}

impl std::fmt::Display for TileWithColors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                write!(f, "{}", self.pixels[y * 8 + x])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Sprite {
    pub fn new(tile: TileWithColors, x: u8, y: u8, sprite_flags: SpriteFlags) -> Self {
        Sprite {
            tile: tile,
            x,
            y,
            sprite_flags,
        }
    }

    pub fn new_from_bytes(
        tile_data: [u8; 4],
        tile: Tile,
        palette_obp0: &Palette,
        palette_obp1: &Palette,
    ) -> Self {
        let y = tile_data[0];
        let x = tile_data[1];
        let sprite_flags = SpriteFlags::from(tile_data[3]);

        let tile_with_colors = tile.to_tile_with_colors(match sprite_flags.palette_number {
            SpritePalette::OBP0 => palette_obp0,
            SpritePalette::OBP1 => palette_obp1,
        });

        Sprite {
            tile: tile_with_colors,
            x,
            y,
            sprite_flags,
        }
    }
}

impl Sprite {
    pub fn get_color(&self, x: usize, y: usize) -> &Color {
        let x = if self.sprite_flags.x_flip { 7 - x } else { x };
        let y = if self.sprite_flags.y_flip { 7 - y } else { y };

        self.tile.get_color(x, y)
    }
}

impl std::convert::From<u8> for SpriteFlags {
    fn from(value: u8) -> Self {
        SpriteFlags {
            bg_and_window_over: (value & 0x80) != 0,
            y_flip: (value & 0x40) != 0,
            x_flip: (value & 0x20) != 0,
            palette_number: match value & 0x10 >> 4 {
                0 => SpritePalette::OBP0,
                1 => SpritePalette::OBP1,
                _ => panic!("Invalid sprite palette"),
            },
        }
    }
}
