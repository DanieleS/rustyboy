use crate::utils::zipper::Zipper;

trait Memo {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct MemoryBank<const C: usize> {
    data: [u8; C],
    offset: usize,
}

impl<const C: usize> std::fmt::Debug for MemoryBank<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rows = self.data.chunks(16);
        for (i, row) in rows.enumerate() {
            let row_str = row
                .iter()
                .map(|&x| format!("{:02x}", x))
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(f, "{:04X} {}", self.offset + i * 0x10, row_str)?;
        }

        Ok(())
    }
}

impl<const C: usize> MemoryBank<C> {
    fn new(offset: usize) -> Self {
        MemoryBank {
            data: [0; C],
            offset,
        }
    }

    fn new_from_data(data: [u8; C], offset: usize) -> Self {
        MemoryBank { data, offset }
    }
}

impl<const C: usize> Memo for MemoryBank<C> {
    fn read(&self, address: u16) -> u8 {
        self.data[address as usize - self.offset]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize - self.offset] = value;
    }
}

impl Memo for u8 {
    fn read(&self, _address: u16) -> u8 {
        *self
    }

    fn write(&mut self, _address: u16, value: u8) {
        *self = value;
    }
}

pub struct Memory {
    cartridge_bank_0: MemoryBank<0x4000>,
    cartridge_banks_1_n: Zipper<MemoryBank<0x4000>>,
    vram: MemoryBank<0x2000>,
    external_ram: Zipper<MemoryBank<0x2000>>,
    work_ram: MemoryBank<0x1000>,
    work_ram_1_n: Zipper<MemoryBank<0x1000>>,
    oam: MemoryBank<0x100>,
    pub io_registers: MemoryBank<0x80>,
    hram: MemoryBank<0x7f>,
    interrupt_enable: u8,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            cartridge_bank_0: MemoryBank::new(0x0),
            cartridge_banks_1_n: Zipper::new(vec![MemoryBank::new(0x4000)]).unwrap(),
            vram: MemoryBank::new(0x8000),
            external_ram: Zipper::new(vec![MemoryBank::new(0xa000)]).unwrap(),
            work_ram: MemoryBank::new(0xC000),
            work_ram_1_n: Zipper::new(vec![MemoryBank::new(0xd000)]).unwrap(),
            oam: MemoryBank::new(0xFE00),
            io_registers: MemoryBank::new(0xFF00),
            hram: MemoryBank::new(0xFF80),
            interrupt_enable: 0,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, byte) in rom.iter().enumerate() {
            self.write(i as u16, *byte);
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3fff => self.cartridge_bank_0.read(address),
            0x4000..=0x7fff => self.cartridge_banks_1_n.get().read(address),
            0x8000..=0x9fff => self.vram.read(address),
            0xa000..=0xbfff => self.external_ram.get().read(address),
            0xc000..=0xcfff => self.work_ram.read(address),
            0xd000..=0xdfff => self.work_ram_1_n.get().read(address),
            0xe000..=0xfdff => {
                let new_addr = address - 0x2000;
                self.read(new_addr)
            }
            0xfe00..=0xfeff => self.oam.read(address),
            0xff00..=0xff7f => self.io_registers.read(address),
            0xff80..=0xfffe => self.hram.read(address),
            0xffff => self.interrupt_enable.read(address),
        }
    }

    pub fn read_signed(&self, address: u16) -> i8 {
        self.read(address) as i8
    }

    pub fn read16(&self, address: u16) -> u16 {
        self.read(address) as u16 | ((self.read(address + 1) as u16) << 8)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3fff => self.cartridge_bank_0.write(address, value),
            0x4000..=0x7fff => self.cartridge_banks_1_n.get_mut().write(address, value),
            0x8000..=0x9fff => self.vram.write(address, value),
            0xa000..=0xbfff => self.external_ram.get_mut().write(address, value),
            0xc000..=0xcfff => self.work_ram.write(address, value),
            0xd000..=0xdfff => self.work_ram_1_n.get_mut().write(address, value),
            0xe000..=0xfdff => self.work_ram.write(address, value),
            0xfe00..=0xfeff => self.oam.write(address, value),
            0xff00..=0xff7f => self.io_registers.write(address, value),
            0xff80..=0xfffe => self.hram.write(address, value),
            0xffff => self.interrupt_enable.write(address, value),
        }
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write(address, (value & 0xFF) as u8);
        self.write(address + 1, (value >> 8) as u8);
    }
}
