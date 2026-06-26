use std::io::{Write, stdout};

use drive_free::{
    DeviceType, RawInputManager,
    event::{DevId, MouseButton, PressState, RawEvent},
};
use winapi::um::winuser::{self, VK_SPACE};

fn user_select_mouse(manager: &mut RawInputManager) -> DevId {
    loop {
        if let Some(RawEvent::MouseButtonEvent(id, MouseButton::Left, PressState::Press)) =
            manager.get_event()
        {
            return id;
        }
    }
}

fn user_select_keyboard(manager: &mut RawInputManager) -> DevId {
    loop {
        if let Some(RawEvent::KeyboardEvent(id, VK_SPACE, PressState::Press, _)) =
            manager.get_event()
        {
            return id;
        }
    }
}

fn ask_user_to_select_devices(manager: &mut RawInputManager) -> Result<(DevId, DevId, DevId), ()> {
    print!("Press LEFT CLICK on the mouse you would like to be the STEERING WHEEL:");
    stdout().flush().unwrap();
    let wheel_dev_id = user_select_mouse(manager);
    println!("\nDevice ID: {}\n", wheel_dev_id);

    print!("Press LEFT CLICK on the mouse you would like to be the GEARSTICK:");
    stdout().flush().unwrap();
    let gearstick_dev_id = user_select_mouse(manager);
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
    let pedals_dev_id = user_select_keyboard(manager);
    println!("Device ID: {}", pedals_dev_id);

    println!();
    Ok((wheel_dev_id, gearstick_dev_id, pedals_dev_id))
}

fn update_wheel(dx: i32, dy: i32) {
    dbg!(dx, dy);
}

fn main() {
    RawInputManager::init();
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::All);
    let Ok((wheel_dev_id, gearstick_dev_id, pedals_dev_id)) =
        ask_user_to_select_devices(&mut manager)
    else {
        return;
    };

    loop {
        match manager.get_event() {
            Some(RawEvent::MouseMoveEvent(wheel_dev_id, dx, dy)) => update_wheel(dx, dy), // TODO it says this variable isn't being used, wat
            Some(RawEvent::MouseButtonEvent(
                wheel_dev_id,
                MouseButton::Right,
                PressState::Press,
            )) => return,
            Some(_) | None => (),
        }
    }
}
