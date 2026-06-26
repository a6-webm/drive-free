use std::io::{Write, stdout};

use drive_free::{
    DeviceType, RawInputManager,
    event::{DevId, MouseButton, PressState, RawEvent},
};
use winapi::um::winuser::{self, VK_SPACE};

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

fn ask_user_to_select_devices() -> Result<(DevId, DevId, DevId), ()> {
    print!("Press LEFT CLICK on the mouse you would like to be the STEERING WHEEL:");
    stdout().flush().unwrap();
    let wheel_dev_id = user_select_mouse();
    println!("\nDevice ID: {}\n", wheel_dev_id);

    print!("Press LEFT CLICK on the mouse you would like to be the GEARSTICK:");
    stdout().flush().unwrap();
    let gearstick_dev_id = user_select_mouse();
    if gearstick_dev_id == wheel_dev_id {
        println!(
            "\n\n---------- ERROR: Steering wheel and gearstick are the same mouse ----------"
        );
        println!("Make sure to have at least two mouses plugged in, and run the program again\n");
        return Err(());
    }
    println!("\nDevice ID: {}\n", gearstick_dev_id);

    println!("Press SPACEBAR on the keyboard you would like to be the PEDALS:");
    stdout().flush().unwrap();
    let pedals_dev_id = user_select_keyboard();
    println!("Device ID: {}", pedals_dev_id);

    println!();
    Ok((wheel_dev_id, gearstick_dev_id, pedals_dev_id))
}

fn main() {
    RawInputManager::init();
    let Ok((wheel_dev_id, gearstick_dev_id, pedals_dev_id)) = ask_user_to_select_devices() else {
        return;
    };
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
