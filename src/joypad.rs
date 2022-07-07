use crate::memory::Memory;

const JOYPAD_STATE_ADDRESS: u16 = 0xff00;

struct JoypadState {
    keys: Vec<Keys>,
}

pub enum Keys {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

pub fn update_keys_status(ram: &mut Memory) {
    let joypad_state = ram.read(JOYPAD_STATE_ADDRESS);

    let joypad_state = joypad_state | 0xff;

    ram.write(JOYPAD_STATE_ADDRESS, joypad_state);
}
