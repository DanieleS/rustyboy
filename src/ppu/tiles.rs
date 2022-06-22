use crate::memory::Memory;

#[derive(Debug)]
pub struct Tile {
    pub data: [u8; 16],
}

impl Tile {
    pub fn read_from(ram: &Memory, address: u16) -> Tile {
        let mut data = [0; 16];
        for i in 0..16 {
            data[i] = ram.read(address + i as u16);
        }
        Tile { data }
    }
}
