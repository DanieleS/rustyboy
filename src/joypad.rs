use std::collections::HashSet;

use crate::memory::Memory;

const JOYPAD_STATE_ADDRESS: u16 = 0xff00;

pub struct JoypadState {
    keys: HashSet<JoypadKey>,
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
pub enum JoypadKey {
    A,
    B,
    Select,
    Start,
    Up,
    Down,
    Left,
    Right,
}

impl JoypadState {
    pub fn new() -> JoypadState {
        JoypadState {
            keys: HashSet::new(),
        }
    }

    pub fn set_key_pressed(&mut self, key: JoypadKey) {
        self.keys.insert(key);
    }

    pub fn set_key_released(&mut self, key: JoypadKey) {
        self.keys.retain(|&k| k != key);
    }

    pub fn update_keys_status(&self, ram: &mut Memory) {
        let joypad_state = ram.read(JOYPAD_STATE_ADDRESS);

        let new_state = if joypad_state & 0b0010_0000 == 0 {
            // Action buttons
            let mut pressed_keys = 0b1101_1111;
            if self.keys.contains(&JoypadKey::A) {
                pressed_keys &= 0b111_1110;
            }
            if self.keys.contains(&JoypadKey::B) {
                pressed_keys &= 0b111_1101;
            }
            if self.keys.contains(&JoypadKey::Select) {
                pressed_keys &= 0b111_1011;
            }
            if self.keys.contains(&JoypadKey::Start) {
                pressed_keys &= 0b111_0111;
            }

            pressed_keys
        } else {
            let mut pressed_keys = 0b1110_1111;
            if self.keys.contains(&JoypadKey::Right) {
                pressed_keys &= 0b111_1110;
            }
            if self.keys.contains(&JoypadKey::Left) {
                pressed_keys &= 0b111_1101;
            }
            if self.keys.contains(&JoypadKey::Up) {
                pressed_keys &= 0b111_1011;
            }
            if self.keys.contains(&JoypadKey::Down) {
                pressed_keys &= 0b111_0111;
            }

            pressed_keys
        };

        ram.write(JOYPAD_STATE_ADDRESS, new_state)
    }
}
