use anyhow::Result;
use cartridge::CartridgeInfo;
use std::env;

mod cartridge;
mod cpu;
mod ram;

fn main() -> Result<()> {
    let rom_path: String = env::args().nth(1).expect("No ROM path provided");
    let cartridge = CartridgeInfo::from_path(rom_path)?;
    println!("{:?}", cartridge);

    Ok(())
}
