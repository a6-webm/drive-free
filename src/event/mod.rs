pub mod keyboard;
pub mod mouse;

pub use keyboard::KeyPos;
pub use mouse::MouseButton;

pub type DevId = usize;
pub type Vk = i32;

pub const VK_0: Vk = 0x30;
pub const VK_1: Vk = 0x31;
pub const VK_2: Vk = 0x32;
pub const VK_3: Vk = 0x33;
pub const VK_4: Vk = 0x34;
pub const VK_5: Vk = 0x35;
pub const VK_6: Vk = 0x36;
pub const VK_7: Vk = 0x37;
pub const VK_8: Vk = 0x38;
pub const VK_9: Vk = 0x39;
pub const VK_A: Vk = 0x41;
pub const VK_B: Vk = 0x42;
pub const VK_C: Vk = 0x43;
pub const VK_D: Vk = 0x44;
pub const VK_E: Vk = 0x45;
pub const VK_F: Vk = 0x46;
pub const VK_G: Vk = 0x47;
pub const VK_H: Vk = 0x48;
pub const VK_I: Vk = 0x49;
pub const VK_J: Vk = 0x4A;
pub const VK_K: Vk = 0x4B;
pub const VK_L: Vk = 0x4C;
pub const VK_M: Vk = 0x4D;
pub const VK_N: Vk = 0x4E;
pub const VK_O: Vk = 0x4F;
pub const VK_P: Vk = 0x50;
pub const VK_Q: Vk = 0x51;
pub const VK_R: Vk = 0x52;
pub const VK_S: Vk = 0x53;
pub const VK_T: Vk = 0x54;
pub const VK_U: Vk = 0x55;
pub const VK_V: Vk = 0x56;
pub const VK_W: Vk = 0x57;
pub const VK_X: Vk = 0x58;
pub const VK_Y: Vk = 0x59;
pub const VK_Z: Vk = 0x5A;

#[derive(Clone)]
pub enum PressState {
    Press,
    Release,
}

#[derive(Clone)]
pub enum RawEvent {
    MouseButtonEvent(DevId, MouseButton, PressState),
    MouseMoveEvent(DevId, i32, i32),
    MouseWheelEvent(DevId, f32),
    KeyboardEvent(DevId, Vk, PressState, KeyPos),
}
