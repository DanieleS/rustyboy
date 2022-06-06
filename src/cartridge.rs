use anyhow::Result;
use std::{fs::File, io::Read};

#[derive(Clone, Debug)]
struct CartridgeInfo {
    pub title: String,
}

impl CartridgeInfo {
    pub fn from_path(path: String) -> Result<Self> {
        let mut cartridge_file = File::open(path)?;
        let mut cartridge_data = vec![];
        cartridge_file.read_to_end(&mut cartridge_data)?;

        CartridgeInfo::from_data(cartridge_data)
    }

    pub fn from_data(data: Vec<u8>) -> Result<Self> {
        if data.len() < 0x8000 || data.len() % 0x4000 != 0 {
            return Err(anyhow::anyhow!("Invalid cartridge size"));
        }

        panic!("Not implemented");
    }
}
