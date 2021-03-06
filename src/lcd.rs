use std::ops::RangeInclusive;

use crate::ppu::PpuMode;

#[derive(Debug)]
pub enum ObjectSize {
    Size8x8,
    Size8x16,
}

#[derive(Debug)]
pub struct LcdControl {
    enabled: bool,
    window_tile_map_area: bool,
    pub window_enabled: bool,
    pub background_window_tile_data_area: bool,
    background_tile_map_area: bool,
    pub object_size: ObjectSize,
    pub object_enable: bool,
    pub background_enable: bool,
}

impl LcdControl {
    pub fn get_window_tile_map_area(&self) -> RangeInclusive<u16> {
        if self.window_tile_map_area {
            0x9C00..=0x9FFF
        } else {
            0x9800..=0x9BFF
        }
    }

    pub fn get_background_window_tile_data_area(&self) -> RangeInclusive<u16> {
        if self.background_window_tile_data_area {
            0x8000..=0x8FFF
        } else {
            0x8800..=0x97FF
        }
    }

    pub fn get_background_window_tile_address(&self, tile_index: u8) -> u16 {
        if !self.background_window_tile_data_area {
            let signed_index = tile_index as i8;
            (0x9000 + (signed_index as i32 * 16)) as u16
        } else {
            0x8000 + ((tile_index as u16) * 16)
        }
    }

    pub fn get_background_tile_map_area(&self) -> RangeInclusive<u16> {
        if self.background_tile_map_area {
            0x9C00..=0x9FFF
        } else {
            0x9800..=0x9BFF
        }
    }
}

impl std::convert::From<u8> for LcdControl {
    fn from(value: u8) -> LcdControl {
        LcdControl {
            enabled: value & 0x80 == 0x80,
            window_tile_map_area: value & 0x40 == 0x40,
            window_enabled: value & 0x20 == 0x20,
            background_window_tile_data_area: value & 0x10 == 0x10,
            background_tile_map_area: value & 0x08 == 0x08,
            object_size: match value & 0x04 {
                0x04 => ObjectSize::Size8x16,
                _ => ObjectSize::Size8x8,
            },
            object_enable: value & 0x02 == 0x02,
            background_enable: value & 0x01 == 0x01,
        }
    }
}

impl std::convert::From<&LcdControl> for u8 {
    fn from(value: &LcdControl) -> u8 {
        let mut result = 0;
        if value.enabled {
            result |= 0x80;
        }
        if value.window_tile_map_area {
            result |= 0x40;
        }
        if value.window_enabled {
            result |= 0x20;
        }
        if value.background_window_tile_data_area {
            result |= 0x10;
        }
        if value.background_tile_map_area {
            result |= 0x08;
        }
        if let ObjectSize::Size8x16 = value.object_size {
            result |= 0x04;
        }
        if value.object_enable {
            result |= 0x02;
        }
        if value.background_enable {
            result |= 0x01;
        }
        result
    }
}

impl ObjectSize {
    pub fn get_height(&self) -> u8 {
        match self {
            ObjectSize::Size8x8 => 8,
            ObjectSize::Size8x16 => 16,
        }
    }
}

pub struct LcdStat {
    pub lyc_eq_ly_interrupt_source: bool,
    pub mode_2_interrupt_source: bool,
    pub mode_1_interrupt_source: bool,
    pub mode_0_interrupt_source: bool,
    pub lyc_eq_ly: bool,
    pub mode: PpuMode,
}

impl std::convert::From<u8> for LcdStat {
    fn from(byte: u8) -> Self {
        LcdStat {
            lyc_eq_ly_interrupt_source: byte & 0x40 == 0x40,
            mode_2_interrupt_source: byte & 0x20 == 0x20,
            mode_1_interrupt_source: byte & 0x10 == 0x10,
            mode_0_interrupt_source: byte & 0x08 == 0x08,
            lyc_eq_ly: byte & 0x04 == 0x04,
            mode: match byte & 0x03 {
                0x00 => PpuMode::HBlank,
                0x01 => PpuMode::VBlank,
                0x02 => PpuMode::OamSearch,
                0x03 => PpuMode::PixelTransfer,
                _ => panic!("Invalid LCD stat mode"),
            },
        }
    }
}

impl std::convert::From<LcdStat> for u8 {
    fn from(value: LcdStat) -> u8 {
        let mut result = 0;
        if value.lyc_eq_ly_interrupt_source {
            result |= 0x40;
        }
        if value.mode_2_interrupt_source {
            result |= 0x20;
        }
        if value.mode_1_interrupt_source {
            result |= 0x10;
        }
        if value.mode_0_interrupt_source {
            result |= 0x08;
        }
        if value.lyc_eq_ly {
            result |= 0x04;
        }
        match value.mode {
            PpuMode::HBlank => result |= 0x00,
            PpuMode::VBlank => result |= 0x01,
            PpuMode::OamSearch => result |= 0x02,
            PpuMode::PixelTransfer => result |= 0x03,
        }
        result
    }
}
