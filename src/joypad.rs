use std::collections::HashSet;

use crate::memory::Memory;

const JOYPAD_STATE_ADDRESS: u16 = 0xff00;

pub struct JoypadState {
    keys: HashSet<JoypadKey>,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

        let new_state = if !joypad_state & 0b0010_0000 != 0 {
            // Action buttons
            let mut pressed_keys = 0b0000_1111;
            if self.keys.contains(&JoypadKey::A) {
                pressed_keys &= 0b1110;
            }
            if self.keys.contains(&JoypadKey::B) {
                pressed_keys &= 0b1101;
            }
            if self.keys.contains(&JoypadKey::Select) {
                pressed_keys &= 0b1011;
            }
            if self.keys.contains(&JoypadKey::Start) {
                pressed_keys &= 0b0111;
            }

            pressed_keys
        } else {
            let mut pressed_keys = 0b0000_1111;
            if self.keys.contains(&JoypadKey::Right) {
                pressed_keys &= 0b1110;
            }
            if self.keys.contains(&JoypadKey::Left) {
                pressed_keys &= 0b1101;
            }
            if self.keys.contains(&JoypadKey::Up) {
                pressed_keys &= 0b1011;
            }
            if self.keys.contains(&JoypadKey::Down) {
                pressed_keys &= 0b0111;
            }

            pressed_keys
        };

        // if self.keys.len() > 0 {
        //     println!("Joypad State: {:b}", joypad_state);
        //     println!("Pressed keys: {:?}", self.keys);
        //     println!("New state: {:b}", new_state);
        // }

        ram.write(JOYPAD_STATE_ADDRESS, new_state)
    }
}
