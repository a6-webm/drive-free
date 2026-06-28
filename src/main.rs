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
    deadzone: i32,
    max_rot: f32,
    pos: (i32, i32),
    half_rot_index: i32,
    axis: i16,
    dbg: u32,
}

impl WheelState {
    fn new(deadzone: i32, max_rot: f32) -> Self {
        Self {
            deadzone,
            max_rot,
            ..Default::default()
        }
    }

    fn update(&mut self, delta: (i32, i32)) {
        self.pos.0 += delta.0;
        self.pos.1 -= delta.1;
        let x = self.pos.0;
        let y = self.pos.1;
        if x >= -self.deadzone && x <= self.deadzone && y >= -self.deadzone && y <= self.deadzone {
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
        let angle = angle.clamp(-self.max_rot * 2.0 * PI, self.max_rot * 2.0 * PI);
        self.axis = (i16::MAX as f32 * angle / (self.max_rot * 2.0 * PI)) as i16;
        if self.dbg > 5 {
            dbg!(self.axis);
            self.dbg = 0;
        } else {
            self.dbg += 1;
        }
    }
}

type GearMap = Vec<Vec<Option<GearCell>>>;

#[derive(Clone)]
struct GearCell {
    gear: i32,
    can_u: bool,
    can_d: bool,
    can_r: bool,
    can_l: bool,
    can_l_special: bool,
}

struct GearstickState {
    map: GearMap,
    mouse_pos: (i32, i32),
    gear_pos: (usize, usize),
    dist: i32,
    special: bool,
}

impl GearstickState {
    fn new_6_speed(dist: i32) -> Self {
        Self {
            map: Self::gear_map_6_speed(),
            dist,
            gear_pos: (2, 1),
            mouse_pos: (0, 0),
            special: false,
        }
    }

    fn new_5_speed(dist: i32) -> Self {
        Self {
            map: Self::gear_map_5_speed(),
            dist,
            gear_pos: (2, 1),
            mouse_pos: (0, 0),
            special: false,
        }
    }

    fn get_gear(&self) -> i32 {
        self.map[self.gear_pos.1][self.gear_pos.0]
            .as_ref()
            .unwrap()
            .gear
    }

    fn update(&mut self, delta: (i32, i32)) {
        let cell = self.map[self.gear_pos.1][self.gear_pos.0].as_ref().unwrap();
        self.mouse_pos.0 += delta.0;
        self.mouse_pos.1 += delta.1;
        if !cell.can_u && self.mouse_pos.1 < 0 || !cell.can_d && self.mouse_pos.1 > 0 {
            self.mouse_pos.1 = 0;
        }
        if !(cell.can_l || cell.can_l_special && self.special) && self.mouse_pos.0 < 0
            || !cell.can_r && self.mouse_pos.0 > 0
        {
            self.mouse_pos.0 = 0;
        }
        if self.mouse_pos.0 > self.dist {
            self.gear_pos.0 += 1;
            self.mouse_pos.0 -= 2 * self.dist;
        }
        if self.mouse_pos.0 < -self.dist {
            self.gear_pos.0 -= 1;
            self.mouse_pos.0 += 2 * self.dist;
        }
        if self.mouse_pos.1 > self.dist {
            self.gear_pos.1 += 1;
            self.mouse_pos.1 -= 2 * self.dist;
        }
        if self.mouse_pos.1 < -self.dist {
            self.gear_pos.1 -= 1;
            self.mouse_pos.1 += 2 * self.dist;
        }
    }

    fn gear_map_6_speed() -> GearMap {
        let width = 4;
        let height = 3;
        let mut map: GearMap = vec![vec![None; width]; height];
        map[1][0] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: false,
            can_r: true,
            can_l: false,
            can_l_special: false,
        });
        map[1][1] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: true,
            can_r: true,
            can_l: false,
            can_l_special: true,
        });
        map[1][2] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: true,
            can_r: true,
            can_l: true,
            can_l_special: false,
        });
        map[1][3] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: true,
            can_r: false,
            can_l: true,
            can_l_special: false,
        });
        map[0][0] = Some(GearCell {
            gear: -1,
            can_u: false,
            can_d: true,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[0][1] = Some(GearCell {
            gear: 1,
            can_u: false,
            can_d: true,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[0][2] = Some(GearCell {
            gear: 3,
            can_u: false,
            can_d: true,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[0][3] = Some(GearCell {
            gear: 5,
            can_u: false,
            can_d: true,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[2][1] = Some(GearCell {
            gear: 2,
            can_u: true,
            can_d: false,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[2][2] = Some(GearCell {
            gear: 4,
            can_u: true,
            can_d: false,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[2][3] = Some(GearCell {
            gear: 6,
            can_u: true,
            can_d: false,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map
    }

    fn gear_map_5_speed() -> GearMap {
        let mut map = Self::gear_map_6_speed();
        map[1][3] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: false,
            can_r: false,
            can_l: true,
            can_l_special: false,
        });
        map[2][3] = None;
        map
    }

    fn print_gear_map(&self) {
        let s = |b| if b { "1" } else { "0" };
        for row in self.map.iter() {
            for cell in row {
                if let Some(cell) = cell {
                    print!(
                        "{}{}{}{}{} ",
                        cell.gear,
                        s(cell.can_u),
                        s(cell.can_r),
                        s(cell.can_d),
                        if cell.can_l_special {
                            "s"
                        } else {
                            s(cell.can_l)
                        }
                    )
                } else {
                    print!("      ")
                }
            }
            println!()
        }
    }
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
    let mut wheel_state = WheelState::new(100, 1.5);
    let mut gearstick_state = GearstickState::new_6_speed(500);

    let mut dbg_gear = 0i32;
    loop {
        match manager.get_event() {
            Some(RawEvent::MouseMoveEvent(id, dx, dy)) if id == wheel_dev_id => {
                wheel_state.update((dx, dy));
                // println!("{:?}", wheel_state.axis); // debug
            }
            Some(RawEvent::MouseButtonEvent(id, MouseButton::Left, PressState::Press))
                if id == wheel_dev_id =>
            {
                return;
            }
            Some(RawEvent::MouseMoveEvent(id, dx, dy)) if id == gearstick_dev_id => {
                gearstick_state.update((dx, dy));
                // println!("{:?}", gearstick_state.mouse_pos); // debug
            }
            Some(RawEvent::MouseButtonEvent(id, MouseButton::Right, press))
                if id == gearstick_dev_id =>
            {
                gearstick_state.special = press == PressState::Press;
            }
            Some(_) | None => (),
        }
        {
            // debug
            let current = gearstick_state.get_gear();
            if dbg_gear != current {
                dbg_gear = current;
                dbg!(dbg_gear);
            }
        }
    }
}
