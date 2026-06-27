use std::{
    f32::consts::PI,
    io::{Write, stdout},
};

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

#[derive(Default)]
struct WheelState {
    pos: (i32, i32),
    half_rot_index: i32,
    axis: i16,
    dbg: u32,
}

impl WheelState {
    fn update(&mut self, delta: (i32, i32)) {
        const DEADZONE: i32 = 100;
        const MAX_ROT: f32 = 1.5;
        self.pos.0 += delta.0;
        self.pos.1 -= delta.1;
        let x = self.pos.0;
        let y = self.pos.1;
        if x >= -DEADZONE && x <= DEADZONE && y >= -DEADZONE && y <= DEADZONE {
            return;
        }
        if self.half_rot_index % 2 == 0 {
            if y < 0 {
                // pos was in top half, is now in bottom half
                if x < 0 {
                    self.half_rot_index -= 1;
                } else {
                    self.half_rot_index += 1;
                }
            }
        } else {
            if y >= 0 {
                // pos was in bottom half, is now in top half
                if x < 0 {
                    self.half_rot_index += 1;
                } else {
                    self.half_rot_index -= 1;
                }
            }
        }
        // 0 degrees at the top, increasing clockwise
        let angle = (x as f32 / y as f32).atan() + self.half_rot_index as f32 * PI;
        let angle = angle.clamp(-MAX_ROT * 2.0 * PI, MAX_ROT * 2.0 * PI);
        self.axis = (i16::MAX as f32 * angle / (MAX_ROT * 2.0 * PI)) as i16;
        if self.dbg > 5 {
            dbg!(self.axis);
            self.dbg = 0;
        } else {
            self.dbg += 1;
        }
    }
}

struct GearstickState {
    todo: i32,
}
struct PedalState {
    todo: i32,
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
    let mut wheel_state = WheelState::default();

    loop {
        match manager.get_event() {
            Some(RawEvent::MouseMoveEvent(id, dx, dy)) if id == wheel_dev_id => {
                wheel_state.update((dx, dy));
            }
            Some(RawEvent::MouseButtonEvent(_id, MouseButton::Left, PressState::Press)) => {
                return;
            }
            Some(_) | None => (),
        }
    }
}
