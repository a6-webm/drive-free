use crate::keyboard::KeyPos;

/// State of a Key or Button
#[derive(Clone)]
pub enum State {
    Pressed,
    Released,
}

pub type Vk = i32;

/// Mouse Buttons
#[derive(Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

/// Event types
///
/// The usize entry acts as a device ID unique to each DeviceType (Mouse, Keyboard, Hid)
#[derive(Clone)]
pub enum RawEvent {
    MouseButtonEvent(usize, MouseButton, State),
    MouseMoveEvent(usize, i32, i32),
    MouseWheelEvent(usize, f32),
    KeyboardEvent(usize, Vk, State, KeyPos),
}
