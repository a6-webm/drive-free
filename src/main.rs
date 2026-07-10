mod gearstick;
mod pedals;
mod wheel;

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

fn main() {
    RawInputManager::init();
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::All);
    let Ok((wheel_dev_id, gearstick_dev_id, pedals_dev_id)) =
        ask_user_to_select_devices(&mut manager)
    else {
        return;
    };
    let mut wheel_state = wheel::WheelState::new(10.0);
    let mut gearstick_state = gearstick::GearstickState::new_6_speed(500);
    let mut pedals_state = pedals::PedalsState::new();

    let mut dbg_gear = 0i32;
    loop {
        match manager.get_event() {
            Some(RawEvent::MouseMoveEvent(id, dx, dy)) if id == wheel_dev_id => {
                wheel_state.update((dx, dy));
                // println!("{:?}", wheel_state.axis); // dbg
            }
            Some(RawEvent::MouseButtonEvent(id, MouseButton::Left, PressState::Press))
                if id == wheel_dev_id =>
            {
                return;
            }
            Some(RawEvent::MouseMoveEvent(id, dx, dy)) if id == gearstick_dev_id => {
                gearstick_state.update((dx, dy));
                // println!("{:?}", gearstick_state.mouse_pos); // dbg
            }
            Some(RawEvent::MouseButtonEvent(id, MouseButton::Right, press))
                if id == gearstick_dev_id =>
            {
                gearstick_state.special = press == PressState::Press;
            }
            Some(RawEvent::KeyboardEvent(id, key, press, _key_pos)) if id == pedals_dev_id => {
                pedals_state.update(key, press);
                pedals_state.dbg();
            }
            Some(_) | None => (),
        }
        {
            // dbg
            let current = gearstick_state.get_gear();
            if dbg_gear != current {
                dbg_gear = current;
                dbg!(dbg_gear);
            }
        }
    }
}
