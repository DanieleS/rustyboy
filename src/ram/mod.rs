pub struct Ram {
    memory: [u8; 0xFFFF],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [0; 0xFFFF],
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, byte) in rom.iter().enumerate() {
            self.memory[i] = *byte;
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_signed(&self, address: u16) -> i8 {
        self.memory[address as usize] as i8
    }

    pub fn read16(&self, address: u16) -> u16 {
        (self.read(address) as u16) | ((self.read(address + 1) as u16) << 8)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write(address, (value & 0xFF) as u8);
        self.write(address + 1, (value >> 8) as u8);
    }
}
