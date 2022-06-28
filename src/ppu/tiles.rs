use crate::memory::Memory;

use super::palette::{Color, Palette};

#[derive(Debug)]
pub struct Tile {
    pub data: [u8; 16],
}

pub struct TileWithColors<'a> {
    pixels: [&'a Color; 64],
}

impl Tile {
    pub fn read_from(ram: &Memory, address: u16) -> Tile {
        let mut data = [0; 16];
        for i in 0..16 {
            data[i] = ram.read(address + i as u16);
        }
        Tile { data }
    }

    pub fn to_tile_with_colors<'a>(&self, palette: &'a Palette) -> TileWithColors<'a> {
        let mut pixels = [&Color::Black; 64];

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

            let color = palette.color_from_bits(first_bit, last_bit);

            pixels[i] = color;
        }

        TileWithColors { pixels }
    }
}

impl<'a> TileWithColors<'a> {
    pub fn get_color(&self, x: usize, y: usize) -> &'a Color {
        &self.pixels[y * 8 + x]
    }
}

impl std::fmt::Display for TileWithColors<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..8 {
            for x in 0..8 {
                write!(f, "{}", self.pixels[y * 8 + x])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
