use crate::memory::Memory;

pub const INTERRUPT_ENABLED_ADDRESS: u16 = 0xffff;
pub const INTERRUPT_FLAG_ADDRESS: u16 = 0xff0f;

#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    VBlank = 0x01,
    LCDStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
}

impl std::ops::BitAnd<u8> for Interrupt {
    type Output = u8;

    fn bitand(self, rhs: u8) -> u8 {
        match self {
            Interrupt::VBlank => rhs & 0x01,
            Interrupt::LCDStat => rhs & 0x02,
            Interrupt::Timer => rhs & 0x04,
            Interrupt::Serial => rhs & 0x08,
            Interrupt::Joypad => rhs & 0x10,
        }
    }
}

impl std::ops::BitAnd<Interrupt> for u8 {
    type Output = u8;

    fn bitand(self, rhs: Interrupt) -> u8 {
        match rhs {
            Interrupt::VBlank => self & 0x01,
            Interrupt::LCDStat => self & 0x02,
            Interrupt::Timer => self & 0x04,
            Interrupt::Serial => self & 0x08,
            Interrupt::Joypad => self & 0x10,
        }
    }
}

pub struct InterruptStatus {
    enabled: bool,
    flag: bool,
}

impl InterruptStatus {
    pub fn is_active(&self) -> bool {
        self.enabled && self.flag
    }
}

pub struct Interrupts {
    pub vblank: InterruptStatus,
    pub lcd_stat: InterruptStatus,
    pub timer: InterruptStatus,
    pub serial: InterruptStatus,
    pub joypad: InterruptStatus,
}

impl std::convert::From<(u8, u8)> for Interrupts {
    fn from((enabled, flag): (u8, u8)) -> Interrupts {
        Interrupts {
            vblank: InterruptStatus {
                enabled: enabled & Interrupt::VBlank == Interrupt::VBlank as u8,
                flag: flag & Interrupt::VBlank == Interrupt::VBlank as u8,
            },
            lcd_stat: InterruptStatus {
                enabled: enabled & Interrupt::LCDStat == Interrupt::LCDStat as u8,
                flag: flag & Interrupt::LCDStat == Interrupt::LCDStat as u8,
            },
            timer: InterruptStatus {
                enabled: enabled & Interrupt::Timer == Interrupt::Timer as u8,
                flag: flag & Interrupt::Timer == Interrupt::Timer as u8,
            },
            serial: InterruptStatus {
                enabled: enabled & Interrupt::Serial == Interrupt::Serial as u8,
                flag: flag & Interrupt::Serial == Interrupt::Serial as u8,
            },
            joypad: InterruptStatus {
                enabled: enabled & Interrupt::Joypad == Interrupt::Joypad as u8,
                flag: flag & Interrupt::Joypad == Interrupt::Joypad as u8,
            },
        }
    }
}

impl std::convert::From<&mut Interrupts> for (u8, u8) {
    fn from(interrupts: &mut Interrupts) -> (u8, u8) {
        (
            (interrupts.vblank.enabled as u8) << 0
                | (interrupts.lcd_stat.enabled as u8) << 1
                | (interrupts.timer.enabled as u8) << 2
                | (interrupts.serial.enabled as u8) << 3
                | (interrupts.joypad.enabled as u8) << 4,
            (interrupts.vblank.flag as u8) << 0
                | (interrupts.lcd_stat.flag as u8) << 1
                | (interrupts.timer.flag as u8) << 2
                | (interrupts.serial.flag as u8) << 3
                | (interrupts.joypad.flag as u8) << 4,
        )
    }
}

impl Interrupts {
    pub fn get_interrupts(memory_bus: &Memory) -> Self {
        let enabled = memory_bus.read(INTERRUPT_ENABLED_ADDRESS);
        let flag = memory_bus.read(INTERRUPT_FLAG_ADDRESS);

        Interrupts::from((enabled, flag))
    }

    pub fn dispatch_interrupt(interrupt: Interrupt, memory_bus: &mut Memory) {
        let mut interrupts = Interrupts::get_interrupts(memory_bus);
        match interrupt {
            Interrupt::VBlank => {
                interrupts.vblank.flag = true;
            }
            Interrupt::LCDStat => {
                interrupts.lcd_stat.flag = true;
            }
            Interrupt::Timer => {
                interrupts.timer.flag = true;
            }
            Interrupt::Serial => {
                interrupts.serial.flag = true;
            }
            Interrupt::Joypad => {
                interrupts.joypad.flag = true;
            }
        }

        let (_, flag) = <(u8, u8)>::from(&mut interrupts);
        memory_bus.write(INTERRUPT_FLAG_ADDRESS, flag);
    }

    pub fn get_highest_priority_interrupt(&self) -> Option<Interrupt> {
        let mut highest_priority = None;
        if self.vblank.is_active() {
            highest_priority = Some(Interrupt::VBlank);
        }
        if self.lcd_stat.is_active() {
            highest_priority = Some(Interrupt::LCDStat);
        }
        if self.timer.is_active() {
            highest_priority = Some(Interrupt::Timer);
        }
        if self.serial.is_active() {
            highest_priority = Some(Interrupt::Serial);
        }
        if self.joypad.is_active() {
            highest_priority = Some(Interrupt::Joypad);
        }
        highest_priority
    }

    pub fn ack_interrupt(&mut self, interrupt: Interrupt, memory_bus: &mut Memory) {
        match interrupt {
            Interrupt::VBlank => self.vblank.flag = false,
            Interrupt::LCDStat => self.lcd_stat.flag = false,
            Interrupt::Timer => self.timer.flag = false,
            Interrupt::Serial => self.serial.flag = false,
            Interrupt::Joypad => self.joypad.flag = false,
        }

        let (_, flag) = <(u8, u8)>::from(self);
        memory_bus.write(INTERRUPT_FLAG_ADDRESS, flag);
    }

    pub fn is_empty(&self) -> bool {
        !self.vblank.is_active()
            && !self.lcd_stat.is_active()
            && !self.timer.is_active()
            && !self.serial.is_active()
            && !self.joypad.is_active()
    }
}
