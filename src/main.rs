mod gearstick;
mod pedals;
mod wheel;

use std::{
    env,
    io::{Write, stdout},
    net,
};

use drive_free::{
    DeviceType, RawInputManager,
    event::{DevId, MouseButton, PressState, RawEvent, VK_Q},
};
use winapi::um::winuser::{VK_ESCAPE, VK_SPACE};

fn user_select_mouse(manager: &mut RawInputManager) -> DevId {
    loop {
        if let RawEvent::MouseButtonEvent(id, MouseButton::Left, PressState::Press) =
            manager.get_event()
        {
            return id;
        }
    }
}

fn user_select_keyboard(manager: &mut RawInputManager) -> DevId {
    loop {
        if let RawEvent::KeyboardEvent(id, VK_SPACE, PressState::Press, _) = manager.get_event() {
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

fn dbg_mode(manager: &mut RawInputManager) {
    let mut exit_counter = 0;
    loop {
        match manager.get_event() {
            ev @ RawEvent::MouseButtonEvent(_, _, _) => {
                dbg!(ev);
            }
            ev @ RawEvent::KeyboardEvent(_, key, press, _)
                if (key == VK_ESCAPE || key == VK_Q) && press == PressState::Press =>
            {
                exit_counter += 1;
                dbg!(ev);
            }
            ev @ RawEvent::KeyboardEvent(_, _, _, _) => {
                dbg!(ev);
            }
            _ => (),
        }
        if exit_counter > 5 {
            return;
        }
    }
}

fn main() {
    RawInputManager::init();
    let mut manager = RawInputManager::new().unwrap();
    manager.register_devices(DeviceType::All);

    let args: Vec<String> = env::args().collect();
    if args.get(1).map_or(false, |a| a == "dbg") {
        dbg_mode(&mut manager);
        return;
    }

    let Ok((wheel_dev_id, gearstick_dev_id, pedals_dev_id)) =
        ask_user_to_select_devices(&mut manager)
    else {
        return;
    };
    let mut wheel_state = wheel::WheelState::new(10.0);
    let mut gearstick_state = gearstick::GearstickState::new_6_speed(500);
    let mut pedals_state = pedals::PedalsState::new();

    let socket = net::UdpSocket::bind("127.0.0.1:55555").unwrap();
    socket.connect("127.0.0.1:55555").unwrap();
    loop {
        match manager.get_event() {
            RawEvent::MouseMoveEvent(id, dx, dy) if id == wheel_dev_id => {
                wheel_state.update((dx, dy));
                // println!("{:?}", wheel_state.axis); // dbg
            }
            RawEvent::MouseButtonEvent(id, MouseButton::Left, PressState::Press)
                if id == wheel_dev_id =>
            {
                return;
            }
            RawEvent::MouseMoveEvent(id, dx, dy) if id == gearstick_dev_id => {
                gearstick_state.update((dx, dy));
                // println!("{:?}", gearstick_state.mouse_pos); // dbg
            }
            RawEvent::MouseButtonEvent(id, MouseButton::Right, press) if id == gearstick_dev_id => {
                gearstick_state.special = press == PressState::Press;
            }
            RawEvent::KeyboardEvent(id, key, press, key_pos) if id == pedals_dev_id => {
                pedals_state.update(key, key_pos, press);
                // pedals_state.dbg();
                // TODO remove dbg ----------
                let buf = format!(
                    "{}|{}|{}|{}|{}",
                    wheel_state.axis,
                    pedals_state.get_clutch_axis(),
                    pedals_state.get_brake_axis(),
                    pedals_state.get_throttle_axis(),
                    gearstick_state.get_gear()
                );
                dbg!(buf);
                // --------------------------
            }
            _ => (),
        }
        let mut buf = format!(
            "{}|{}|{}|{}|{}",
            wheel_state.axis,
            pedals_state.get_clutch_axis(),
            pedals_state.get_brake_axis(),
            pedals_state.get_throttle_axis(),
            gearstick_state.get_gear()
        )
        .into_bytes();
        buf.reverse();
        if let Err(e) = socket.send(&buf) {
            println!("{}", e);
        }
        // dbg
        // let current = gearstick_state.get_gear();
        // if dbg_gear != current {
        //     dbg_gear = current;
        //     dbg!(dbg_gear);
        // }
    }
}
