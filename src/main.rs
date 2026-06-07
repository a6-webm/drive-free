use std::time;

use drive_free::{DeviceType, KeyId, MouseButton, RawEvent, RawInputManager, State};

fn main() {
    //print_raw_device_list(devices.clone());
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::All);

    let start_time = time::Instant::now();
    let mut current_time = time::Duration::ZERO;
    while current_time.as_secs_f64() < 10f64 {
        while let Some(event) = manager.get_event() {
            match event {
                RawEvent::MouseButtonEvent(id, MouseButton::Left, State::Pressed) => {
                    println!("Mouse {:?} Left Button Down", id)
                }
                RawEvent::MouseButtonEvent(id, MouseButton::Left, State::Released) => {
                    println!("Mouse {:?} Left Button Up", id)
                }
                RawEvent::MouseButtonEvent(id, MouseButton::Right, State::Pressed) => {
                    println!("Mouse {:?} Right Button Down", id)
                }
                RawEvent::MouseButtonEvent(id, MouseButton::Right, State::Released) => {
                    println!("Mouse {:?} Right Button Up", id)
                }
                RawEvent::MouseButtonEvent(id, MouseButton::Button4, State::Pressed) => {
                    println!("Mouse {:?} Button 4 Down", id)
                }
                RawEvent::MouseButtonEvent(id, MouseButton::Button4, State::Released) => {
                    println!("Mouse {:?} Button 4 Up", id)
                }
                RawEvent::MouseButtonEvent(id, MouseButton::Button5, State::Pressed) => {
                    println!("Mouse {:?} Button 5 Down", id)
                }
                RawEvent::MouseButtonEvent(id, MouseButton::Button5, State::Released) => {
                    println!("Mouse {:?} Button 5 Up", id)
                }
                RawEvent::MouseMoveEvent(id, move_x, move_y) => {
                    println!("Mouse {:?}  Moved {:?} {:?}", id, move_x, move_y)
                }
                RawEvent::MouseWheelEvent(id, data) => {
                    println!("Mouse {:?} Wheel Data {:?}", id, data)
                }
                RawEvent::KeyboardEvent(id, KeyId::Escape, State::Pressed) => {
                    println!("Keyboard {:?} Escape Pressed", id)
                }
                RawEvent::KeyboardEvent(id, KeyId::Escape, State::Released) => {
                    println!("Keyboard {:?} Escape Released", id)
                }
                RawEvent::KeyboardEvent(id, KeyId::Return, State::Pressed) => {
                    println!("Keyboard {:?} Return Pressed", id)
                }
                RawEvent::KeyboardEvent(id, KeyId::Return, State::Released) => {
                    println!("Keyboard {:?} Return Released", id)
                }
                _ => (),
            }
        }
        current_time = time::Instant::now() - start_time;
    }
}
