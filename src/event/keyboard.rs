use winapi::um::winuser;

use crate::{DevId, RawEvent, event::PressState};

#[derive(Clone)]
pub enum KeyPos {
    Left,
    Right,
}

pub fn process_keyboard_data(raw_data: &winuser::RAWKEYBOARD, id: DevId) -> Vec<RawEvent> {
    let mut output: Vec<RawEvent> = Vec::new();
    let flags = raw_data.Flags as u32;
    let key = raw_data.VKey as i32;
    let key_state = if flags & winuser::RI_KEY_BREAK != 0 {
        PressState::Release
    } else {
        PressState::Press
    };
    let key_pos = if flags & winuser::RI_KEY_E0 != 0 {
        KeyPos::Left
    } else {
        KeyPos::Right
    };
    output.push(RawEvent::KeyboardEvent(id, key, key_state, key_pos));
    output
}
