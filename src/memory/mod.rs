use crate::{cartridge::Cartridge, utils::zipper::Zipper};

use self::io::{IOMemoryBank, DMA_ADDRESS};

mod io;
mod timer;

const OAM_BASE_ADDRESS: u16 = 0xfe00;

trait MemoryBank {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub struct GeneralPourposeMemoryBank<const C: usize> {
    data: [u8; C],
    offset: usize,
}

impl<const C: usize> std::fmt::Debug for GeneralPourposeMemoryBank<C> {
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

impl<const C: usize> GeneralPourposeMemoryBank<C> {
    fn new(offset: usize) -> Self {
        GeneralPourposeMemoryBank {
            data: [0; C],
            offset,
        }
    }
}

impl<const C: usize> MemoryBank for GeneralPourposeMemoryBank<C> {
    fn read(&self, address: u16) -> u8 {
        self.data[address as usize - self.offset]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize - self.offset] = value;
    }
}

impl MemoryBank for u8 {
    fn read(&self, _address: u16) -> u8 {
        *self
    }

    fn write(&mut self, _address: u16, value: u8) {
        *self = value;
    }
}

pub struct Memory {
    cartridge: Cartridge,
    vram: GeneralPourposeMemoryBank<0x2000>,
    external_ram: Zipper<GeneralPourposeMemoryBank<0x2000>>,
    pub work_ram: GeneralPourposeMemoryBank<0x1000>,
    work_ram_1_n: Zipper<GeneralPourposeMemoryBank<0x1000>>,
    oam: GeneralPourposeMemoryBank<0x100>,
    pub io_registers: IOMemoryBank,
    hram: GeneralPourposeMemoryBank<0x7f>,
    interrupt_enable: u8,
}

impl Memory {
    pub fn new(cartridge: Cartridge) -> Memory {
        Memory {
            cartridge,
            vram: GeneralPourposeMemoryBank::new(0x8000),
            external_ram: Zipper::new(vec![GeneralPourposeMemoryBank::new(0xa000)]).unwrap(),
            work_ram: GeneralPourposeMemoryBank::new(0xC000),
            work_ram_1_n: Zipper::new(vec![GeneralPourposeMemoryBank::new(0xd000)]).unwrap(),
            oam: GeneralPourposeMemoryBank::new(0xFE00),
            io_registers: IOMemoryBank::new(),
            hram: GeneralPourposeMemoryBank::new(0xFF80),
            interrupt_enable: 0,
        }
    }

    #[inline(always)]
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7fff => self.cartridge.read(address),
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

    pub fn read_bytes<const C: usize>(&self, start_address: u16) -> [u8; C] {
        let mut bytes = [0; C];
        for i in 0..C {
            bytes[i] = self.read(start_address + i as u16);
        }
        bytes
    }

    #[inline(always)]
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7fff => self.cartridge.write(address, value),
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

        if self.io_registers.dma_transfer_requested {
            self.io_registers.dma_transfer_requested = false;
            dma_transfer(self);
        }
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write(address, (value & 0xFF) as u8);
        self.write(address + 1, (value >> 8) as u8);
    }
}

fn dma_transfer(memory_bus: &mut Memory) {
    let mut address = memory_bus.read(DMA_ADDRESS) as u16;
    address <<= 8;

    let byte = memory_bus.read_bytes::<0xa0>(address);
    for i in 0..0xa0 {
        memory_bus.write(OAM_BASE_ADDRESS + i as u16, byte[i]);
    }
}
