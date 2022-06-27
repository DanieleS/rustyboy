use anyhow::Result;
use std::{fs::File, io::Read};

#[derive(Clone, Debug)]
pub struct Cartridge {
    pub title: String,
    pub data: Vec<u8>,
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

        Ok(Cartridge { title, data })
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
}
