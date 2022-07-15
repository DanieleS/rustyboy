use anyhow::Result;
use std::{fs::File, io::Read};

const MBC_TYPE_ADDRESS: usize = 0x147;

#[derive(Clone, Debug)]
struct Mbc1State {
    selected_rom_bank: u8,
}

#[derive(Clone, Debug)]
enum Mbc {
    NoMbc,
    Mbc1(Mbc1State),
}

#[derive(Clone, Debug)]
pub struct CartridgeHeader {
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub data: Vec<u8>,
    mbc: Mbc,
}

impl Cartridge {
    pub fn from_path(path: String) -> Result<Self> {
        let mut cartridge_file = File::open(path)?;
        let mut cartridge_data = vec![];
        cartridge_file.read_to_end(&mut cartridge_data)?;

        Cartridge::from_data(cartridge_data)
    }

    pub fn from_data(data: Vec<u8>) -> Result<Self> {
        if data.len() < 0x8000 || data.len() % 0x4000 != 0 {
            return Err(anyhow::anyhow!("Invalid cartridge size"));
        }

        let title = Cartridge::parse_title(&data)?;
        let header = CartridgeHeader { title };
        let mbc = Cartridge::parse_mbc(&data)?;

        Ok(Cartridge { header, data, mbc })
    }

    fn is_new_cartridge(data: &[u8]) -> bool {
        data[0x14b] == 0x33
    }

    fn parse_title(data: &[u8]) -> Result<String> {
        let title_range = if Cartridge::is_new_cartridge(data) {
            0x134..0x13f
        } else {
            0x134..0x13c
        };
        let title_data = &data[title_range];
        let title = String::from_utf8(title_data.to_vec())
            .map_err(|_| anyhow::anyhow!("Invalid cartridge title"))?;
        Ok(title.trim_end_matches('\0').to_string())
    }

    fn parse_mbc(data: &[u8]) -> Result<Mbc> {
        let mbc_type = data[MBC_TYPE_ADDRESS];
        let mbc_state = match mbc_type {
            0x00 => Mbc::NoMbc,
            0x01 => Mbc::Mbc1(Mbc1State {
                selected_rom_bank: 1,
            }),
            _ => return Err(anyhow::anyhow!("Invalid cartridge MBC type")),
        };
        Ok(mbc_state)
    }
}

impl Cartridge {
    #[inline(always)]
    pub fn read(&self, address: u16) -> u8 {
        match &self.mbc {
            Mbc::NoMbc => self.data[address as usize],
            Mbc::Mbc1(state) => match address {
                0x0000..=0x3fff => self.data[address as usize],
                _ => {
                    self.data[address as usize + ((state.selected_rom_bank as usize) - 1) * 0x4000]
                }
            },
        }
    }

    #[inline(always)]
    pub fn write(&mut self, address: u16, value: u8) {
        match &mut self.mbc {
            Mbc::NoMbc => (),
            Mbc::Mbc1(state) => match address {
                0x2000..=0x3fff => {
                    let value = if value == 0 { 1 } else { value & 0x1f };
                    state.selected_rom_bank = value;
                }
                _ => (),
            },
        }
    }
}
