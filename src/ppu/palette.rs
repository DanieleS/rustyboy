use crate::memory::Memory;

const BG_PALETTE_ADDRESS: u16 = 0xff47;
const OBP0_PALETTE_ADDRESS: u16 = 0xff48;
const OBP1_PALETTE_ADDRESS: u16 = 0xff49;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Transparent = -1,
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

impl std::convert::From<u8> for Color {
    fn from(value: u8) -> Color {
        match value & 0x03 {
            0x00 => Color::White,
            0x01 => Color::LightGray,
            0x02 => Color::DarkGray,
            0x03 => Color::Black,
            _ => panic!("Invalid color"),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::White => write!(f, " "),
            Color::LightGray => write!(f, "░"),
            Color::DarkGray => write!(f, "▒"),
            Color::Black => write!(f, "█"),
            Color::Transparent => write!(f, " "),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PaletteType {
    Background,
    Sprite,
}

#[derive(Debug)]
pub enum SpritePalette {
    OBP0,
    OBP1,
}

#[derive(Debug, Clone)]
pub struct Palette {
    colors: [Color; 4],
}

impl Palette {
    pub fn from_u8(value: u8, palette_type: PaletteType) -> Palette {
        let colors = [
            match palette_type {
                PaletteType::Background => Color::from(value),
                PaletteType::Sprite => Color::Transparent,
            },
            Color::from(value >> 2),
            Color::from(value >> 4),
            Color::from(value >> 6),
        ];

        Palette { colors }
    }

    pub fn background(memory_bus: &Memory) -> Palette {
        let value = memory_bus.read(BG_PALETTE_ADDRESS);
        Palette::from_u8(value, PaletteType::Background)
    }

    pub fn obp0(memory_bus: &Memory) -> Palette {
        let value = memory_bus.read(OBP0_PALETTE_ADDRESS);
        Palette::from_u8(value, PaletteType::Sprite)
    }

    pub fn obp1(memory_bus: &Memory) -> Palette {
        let value = memory_bus.read(OBP1_PALETTE_ADDRESS);
        Palette::from_u8(value, PaletteType::Sprite)
    }

    pub fn color_from_bits(&self, first_bit: bool, last_bit: bool) -> &Color {
        let color = match (first_bit, last_bit) {
            (true, true) => 0b11,
            (true, false) => 0b10,
            (false, true) => 0b01,
            (false, false) => 0b00,
        };

        &self.colors[color as usize]
    }
}

impl std::default::Default for Palette {
    fn default() -> Self {
        Palette {
            colors: [
                Color::White,
                Color::LightGray,
                Color::DarkGray,
                Color::Black,
            ],
        }
    }
}
