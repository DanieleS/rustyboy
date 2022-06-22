#[derive(Debug)]
pub enum Color {
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
        }
    }
}

#[derive(Debug)]
pub enum PaletteType {
    Background,
    Sprite,
}

#[derive(Debug)]
pub struct Palette {
    colors: [Color; 4],
    palette_type: PaletteType,
}

impl Palette {
    pub fn from_u8(value: u8, palette_type: PaletteType) -> Palette {
        let colors = [
            Color::from(value),
            Color::from(value >> 2),
            Color::from(value >> 4),
            Color::from(value >> 6),
        ];

        Palette {
            colors,
            palette_type,
        }
    }
}
