use anyhow::Result;
use cartridge::Cartridge;
use std::env;

use crate::hadrware::Hardware;

mod cartridge;
mod cpu;
mod hadrware;
mod joypad;
mod lcd;
mod memory;
mod ppu;
mod utils;

fn main() -> Result<()> {
    let rom_path: String = env::args().nth(1).expect("No ROM path provided");
    let cartridge = Cartridge::from_path(rom_path)?;
    println!("Running {}", cartridge.title);

    let mut hardware = Hardware::new(cartridge);
    hardware.run();

    Ok(())
}
