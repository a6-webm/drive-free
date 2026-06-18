use std::time;

use drive_free::{
    DeviceType, RawInputManager,
    event::{DevId, MouseButton, PressState, RawEvent},
};
use winapi::um::winuser::{self, VK_SPACE};

fn main() {
    dbg!(user_select_mouse());
    dbg!(user_select_keyboard());
}

fn user_select_mouse() -> DevId {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Mice);
    loop {
        if let Some(RawEvent::MouseButtonEvent(id, MouseButton::Left, PressState::Press)) =
            manager.get_event()
        {
            return id;
        }
    }
}

fn user_select_keyboard() -> DevId {
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::Keyboards);
    loop {
        if let Some(RawEvent::KeyboardEvent(id, VK_SPACE, PressState::Press, _)) =
            manager.get_event()
        {
            return id;
        }
    }
}

// fn main() {
//     //print_raw_device_list(devices.clone());
//     let mut manager = RawInputManager::new().unwrap();
//     manager.register_devices(DeviceType::All);

//     let start_time = time::Instant::now();
//     let mut current_time = time::Duration::ZERO;
//     while current_time.as_secs_f64() < 10f64 {
//         while let Some(event) = manager.get_event() {
//             match event {
//                 RawEvent::MouseButtonEvent(id, MouseButton::Left, PressState::Press) => {
//                     println!("Mouse {:?} Left Button Down", id)
//                 }
//                 RawEvent::MouseButtonEvent(id, MouseButton::Left, PressState::Release) => {
//                     println!("Mouse {:?} Left Button Up", id)
//                 }
//                 RawEvent::MouseButtonEvent(id, MouseButton::Right, PressState::Press) => {
//                     println!("Mouse {:?} Right Button Down", id)
//                 }
//                 RawEvent::MouseButtonEvent(id, MouseButton::Right, PressState::Release) => {
//                     println!("Mouse {:?} Right Button Up", id)
//                 }
//                 RawEvent::MouseButtonEvent(id, MouseButton::Button4, PressState::Press) => {
//                     println!("Mouse {:?} Button 4 Down", id)
//                 }
//                 RawEvent::MouseButtonEvent(id, MouseButton::Button4, PressState::Release) => {
//                     println!("Mouse {:?} Button 4 Up", id)
//                 }
//                 RawEvent::MouseButtonEvent(id, MouseButton::Button5, PressState::Press) => {
//                     println!("Mouse {:?} Button 5 Down", id)
//                 }
//                 RawEvent::MouseButtonEvent(id, MouseButton::Button5, PressState::Release) => {
//                     println!("Mouse {:?} Button 5 Up", id)
//                 }
//                 RawEvent::MouseMoveEvent(id, move_x, move_y) => {
//                     println!("Mouse {:?}  Moved {:?} {:?}", id, move_x, move_y)
//                 }
//                 RawEvent::MouseWheelEvent(id, data) => {
//                     println!("Mouse {:?} Wheel Data {:?}", id, data)
//                 }
//                 RawEvent::KeyboardEvent(id, winuser::VK_ESCAPE, PressState::Press, _) => {
//                     println!("Keyboard {:?} Escape Pressed", id)
//                 }
//                 RawEvent::KeyboardEvent(id, winuser::VK_ESCAPE, PressState::Release, _) => {
//                     println!("Keyboard {:?} Escape Released", id)
//                 }
//                 RawEvent::KeyboardEvent(id, winuser::VK_RETURN, PressState::Press, _) => {
//                     println!("Keyboard {:?} Return Pressed", id)
//                 }
//                 RawEvent::KeyboardEvent(id, winuser::VK_RETURN, PressState::Release, _) => {
//                     println!("Keyboard {:?} Return Released", id)
//                 }
//                 _ => (),
//             }
//         }
//         current_time = time::Instant::now() - start_time;
//     }
// }
