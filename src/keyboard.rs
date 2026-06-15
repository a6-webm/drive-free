use winapi::um::winuser;

use crate::{RawEvent, State, Vk};

#[derive(Clone)]
pub enum KeyPos {
    Left,
    Right,
}

const VK_0: Vk = 0x30;
const VK_1: Vk = 0x31;
const VK_2: Vk = 0x32;
const VK_3: Vk = 0x33;
const VK_4: Vk = 0x34;
const VK_5: Vk = 0x35;
const VK_6: Vk = 0x36;
const VK_7: Vk = 0x37;
const VK_8: Vk = 0x38;
const VK_9: Vk = 0x39;
const VK_A: Vk = 0x41;
const VK_B: Vk = 0x42;
const VK_C: Vk = 0x43;
const VK_D: Vk = 0x44;
const VK_E: Vk = 0x45;
const VK_F: Vk = 0x46;
const VK_G: Vk = 0x47;
const VK_H: Vk = 0x48;
const VK_I: Vk = 0x49;
const VK_J: Vk = 0x4A;
const VK_K: Vk = 0x4B;
const VK_L: Vk = 0x4C;
const VK_M: Vk = 0x4D;
const VK_N: Vk = 0x4E;
const VK_O: Vk = 0x4F;
const VK_P: Vk = 0x50;
const VK_Q: Vk = 0x51;
const VK_R: Vk = 0x52;
const VK_S: Vk = 0x53;
const VK_T: Vk = 0x54;
const VK_U: Vk = 0x55;
const VK_V: Vk = 0x56;
const VK_W: Vk = 0x57;
const VK_X: Vk = 0x58;
const VK_Y: Vk = 0x59;
const VK_Z: Vk = 0x5A;

pub fn process_keyboard_data(raw_data: &winuser::RAWKEYBOARD, id: usize) -> Vec<RawEvent> {
    let mut output: Vec<RawEvent> = Vec::new();
    let flags = raw_data.Flags as u32;
    let key = raw_data.VKey as i32;
    let key_state = if flags & winuser::RI_KEY_BREAK != 0 {
        State::Released
    } else {
        State::Pressed
    };
    let key_pos = if flags & winuser::RI_KEY_E0 != 0 {
        KeyPos::Left
    } else {
        KeyPos::Right
    };
    output.push(RawEvent::KeyboardEvent(id, key, key_state, key_pos));
    output
}
