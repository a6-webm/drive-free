use std::mem::*;
use winapi::um::winuser;

use crate::{DevId, RawEvent, event::PressState};

#[derive(Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

pub fn process_mouse_data(raw_data: &winuser::RAWMOUSE, id: DevId) -> Vec<RawEvent> {
    let cursor = (raw_data.lLastX, raw_data.lLastY);
    let buttons = raw_data.usButtonFlags;
    let mut output: Vec<RawEvent> = Vec::new();
    if buttons & winuser::RI_MOUSE_LEFT_BUTTON_DOWN != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Left,
            PressState::Press,
        ));
    }
    if buttons & winuser::RI_MOUSE_LEFT_BUTTON_UP != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Left,
            PressState::Release,
        ));
    }
    if buttons & winuser::RI_MOUSE_RIGHT_BUTTON_DOWN != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Right,
            PressState::Press,
        ));
    }
    if buttons & winuser::RI_MOUSE_RIGHT_BUTTON_UP != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Right,
            PressState::Release,
        ));
    }
    if buttons & winuser::RI_MOUSE_MIDDLE_BUTTON_DOWN != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Middle,
            PressState::Press,
        ));
    }
    if buttons & winuser::RI_MOUSE_MIDDLE_BUTTON_UP != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Middle,
            PressState::Release,
        ));
    }
    if buttons & 0x0040 != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Button4,
            PressState::Press,
        ));
    }
    if buttons & 0x0080 != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Button4,
            PressState::Release,
        ));
    }
    if buttons & 0x0100 != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Button5,
            PressState::Press,
        ));
    }
    if buttons & 0x0200 != 0 {
        output.push(RawEvent::MouseButtonEvent(
            id,
            MouseButton::Button5,
            PressState::Release,
        ));
    }
    if buttons & winuser::RI_MOUSE_WHEEL != 0 {
        let wheel_data = raw_data.usButtonData;
        let wheel_value = unsafe { (transmute_copy::<u16, i16>(&wheel_data) as f32) / 120f32 };
        output.push(RawEvent::MouseWheelEvent(id, wheel_value));
    }
    if (cursor.0 != 0) || (cursor.1 != 0) {
        output.push(RawEvent::MouseMoveEvent(id, cursor.0, cursor.1));
    }
    output
}
