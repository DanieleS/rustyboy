use anyhow::Result;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

const MBC_TYPE_ADDRESS: usize = 0x147;

#[derive(Clone, Debug)]
pub struct Mbc1State {
    selected_rom_bank: u8,
}

#[derive(Clone, Debug)]
pub struct Mbc3State {
    selected_rom_bank: u8,
    selected_ram_bank: u8,
    ram: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct NoMbcState {
    ram: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum Mbc {
    NoMbc(NoMbcState),
    Mbc1(Mbc1State),
    Mbc3(Mbc3State),
}

#[derive(Clone, Debug)]
pub struct CartridgeHeader {
    pub title: String,
}

#[derive(Clone, Debug)]
pub struct Cartridge {
    path: String,
    pub header: CartridgeHeader,
    pub data: Vec<u8>,
    pub mbc: Mbc,
}

impl Cartridge {
    pub fn from_path(path: String) -> Result<Self> {
        let mut cartridge_file = File::open(&path)?;
        let mut cartridge_data = vec![];
        cartridge_file.read_to_end(&mut cartridge_data)?;

        Cartridge::from_data(path, cartridge_data)
    }

    fn from_data(path: String, data: Vec<u8>) -> Result<Self> {
        if data.len() < 0x8000 || data.len() % 0x4000 != 0 {
            return Err(anyhow::anyhow!("Invalid cartridge size"));
        }

        let title = Cartridge::parse_title(&data)?;
        let header = CartridgeHeader { title };
        let mbc = Cartridge::parse_mbc(&path, &data)?;

        Ok(Cartridge {
            path,
            header,
            data,
            mbc,
        })
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

    fn parse_mbc(path: &str, data: &[u8]) -> Result<Mbc> {
        let mbc_type = data[MBC_TYPE_ADDRESS];
        let mbc_state = match mbc_type {
            0x00 => Mbc::NoMbc(NoMbcState {
                ram: vec![0; 0x2000],
            }),
            0x01 => Mbc::Mbc1(Mbc1State {
                selected_rom_bank: 1,
            }),
            0x13 => Mbc::Mbc3(Mbc3State {
                selected_rom_bank: 1,
                selected_ram_bank: 0,
                ram: Cartridge::load_mbc_ram(path, 0x8000)?,
            }),
            _ => return Err(anyhow::anyhow!("Invalid cartridge MBC type")),
        };
        Ok(mbc_state)
    }

    fn get_save_file_path(path: &str) -> Option<String> {
        let mut save_path = PathBuf::from(path);
        save_path.set_extension("sav");

        let path = save_path.to_str()?;

        Some(String::from(path))
    }

    fn load_mbc_ram(path: &str, length: usize) -> Result<Vec<u8>> {
        if let Some(path) = Cartridge::get_save_file_path(path) {
            if let Ok(mut save_file) = File::open(path) {
                let mut save_data = vec![];
                save_file.read_to_end(&mut save_data)?;

                if save_data.len() != length {
                    return Ok(vec![0; length]);
                }

                Ok(save_data)
            } else {
                Ok(vec![0; length])
            }
        } else {
            Ok(vec![0; length])
        }
    }
}

impl Cartridge {
    #[inline(always)]
    pub fn read(&self, address: u16) -> u8 {
        match &self.mbc {
            Mbc::NoMbc(state) => match address {
                0xa000..=0xbfff => state.ram[address as usize - 0xa000],
                _ => self.data[address as usize],
            },
            Mbc::Mbc1(state) => match address {
                0x0000..=0x3fff => self.data[address as usize],
                _ => {
                    self.data[address as usize + ((state.selected_rom_bank as usize) - 1) * 0x4000]
                }
            },
            Mbc::Mbc3(state) => match address {
                0x0000..=0x3fff => self.data[address as usize],
                0xa000..=0xbfff => {
                    state.ram
                        [address as usize - 0xa000 + (state.selected_ram_bank as usize) * 0x2000]
                }
                _ => {
                    self.data[address as usize + ((state.selected_rom_bank as usize) - 1) * 0x4000]
                }
            },
        }
    }

    #[inline(always)]
    pub fn write(&mut self, address: u16, value: u8) {
        match &mut self.mbc {
            Mbc::NoMbc(state) => match address {
                0xa000..=0xbfff => {
                    state.ram[address as usize - 0xa000] = value;
                }
                _ => (),
            },
            Mbc::Mbc1(state) => match address {
                0x2000..=0x3fff => {
                    let value = if value == 0 { 1 } else { value & 0x1f };
                    state.selected_rom_bank = value;
                }
                _ => (),
            },
            Mbc::Mbc3(state) => match address {
                0 => {}
                0x2000..=0x3fff => {
                    let value = if value == 0 { 1 } else { value & 0x7f };
                    state.selected_rom_bank = value;
                }
                0x4000..=0x5fff => {
                    state.selected_ram_bank = value;
                }
                0x6000..=0x7fff => {
                    if value != 0 {
                        let mbc_ram = state.ram.clone();
                        if let Err(_) = self.save_ram(&mbc_ram) {
                            println!("Failed to save RAM");
                        }
                    }
                }
                0xa000..=0xbfff => {
                    state.ram
                        [address as usize - 0xa000 + (state.selected_ram_bank as usize) * 0x2000] =
                        value;
                }
                _ => {
                    println!("MBC3 Unhandled! Writing {:X} - Value {:X}", address, value);
                }
            },
        }
    }

    fn save_ram(&self, ram: &[u8]) -> Result<()> {
        if let Some(path) = Cartridge::get_save_file_path(&self.path) {
            let mut file = File::create(path)?;
            file.write_all(ram)?;

            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to get save file path"))
        }
    }
}

impl std::fmt::Display for Mbc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mbc::NoMbc(_) => write!(f, "No MBC"),
            Mbc::Mbc1(state) => write!(f, "MBC1 - ROM Bank: {:02X}", state.selected_rom_bank),
            Mbc::Mbc3(state) => write!(
                f,
                "MBC3 - ROM Bank: {:02X} - RAM Bank: {:02X}",
                state.selected_rom_bank, state.selected_ram_bank
            ),
        }
    }
}
