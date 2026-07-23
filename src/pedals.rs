use drive_free::event::*;
use winapi::um::winuser::*;

const THROTTLE_US_KEYS: &[Key] = &[
    Key::K(VK_SHIFT),
    Key::K(VK_Z),
    Key::K(VK_X),
    Key::K(VK_C),
    Key::K(VK_V),
    Key::K(VK_B),
    Key::K(VK_N),
    Key::K(VK_M),
    Key::K(VK_OEM_COMMA),
    Key::K(VK_OEM_PERIOD),
    Key::K(VK_OEM_2),
];

const BRAKE_US_KEYS: &[Key] = &[
    Key::K(VK_CONTROL),
    Key::K(VK_A),
    Key::K(VK_S),
    Key::K(VK_D),
    Key::K(VK_F),
    Key::K(VK_G),
    Key::K(VK_H),
    Key::K(VK_J),
    Key::K(VK_K),
    Key::K(VK_L),
    Key::K(VK_OEM_1),
    Key::K(VK_OEM_7),
    Key::K(VK_RETURN),
];

const CLUTCH_US_KEYS: &[Key] = &[
    Key::K(VK_OEM_3),
    Key::K(VK_1),
    Key::K(VK_2),
    Key::K(VK_3),
    Key::K(VK_4),
    Key::K(VK_5),
    Key::K(VK_6),
    Key::K(VK_7),
    Key::K(VK_8),
    Key::K(VK_9),
    Key::K(VK_0),
    Key::K(VK_OEM_MINUS),
    Key::K(VK_OEM_PLUS),
    Key::K(VK_BACK),
];

#[derive(Clone, Debug)]
pub enum Key {
    K(Vk),
    KAndPos(Vk, KeyPos),
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::K(l0), Self::K(r0)) => l0 == r0,
            (Self::KAndPos(l0, l1), Self::KAndPos(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::KAndPos(l0, _l1), Self::K(r0)) => l0 == r0,
            (Self::K(l0), Self::KAndPos(r0, _r1)) => l0 == r0,
        }
    }
}

struct Pedal {
    axis: f32,
    keys: Vec<Key>,
    pressed: Vec<bool>,
    pos: isize,
}

impl Pedal {
    fn update(&mut self, key: Vk, key_pos: KeyPos, press: PressState) {
        self.update_pos(key, key_pos, press);
        self.axis = self.calc_axis();
    }

    fn update_pos(&mut self, key: Vk, key_pos: KeyPos, press: PressState) {
        let key = Key::KAndPos(key, key_pos);
        let Some(press_i) = self.keys.iter().cloned().position(|k| k == key) else {
            return;
        };
        let new_press = press == PressState::Press;
        if self.pressed[press_i] == new_press {
            return;
        }
        self.pressed[press_i] = new_press;
        if (press_i as isize) < self.pos {
            return;
        }
        if (press_i as isize) > self.pos {
            self.pos = press_i as isize;
            return;
        }
        if press_i == 0 && !new_press {
            self.pos = -1;
            return;
        }
        for i in (0..=press_i).rev() {
            if self.pressed[i] {
                self.pos = i as isize;
                return;
            }
        }
        self.pos = -1;
    }

    fn calc_axis(&self) -> f32 {
        if self.pos == self.keys.len() as isize - 1 {
            return 0.0;
        }
        let origin = 0.0;
        let index = (self.pos + 1) as f32;
        let step_size = self.keys.len() as f32;
        return origin + index / step_size;
    }
}

pub struct PedalsState {
    throttle: Pedal,
    brake: Pedal,
    clutch: Pedal,
}

impl PedalsState {
    pub fn new() -> Self {
        Self {
            throttle: Pedal {
                axis: 0.0,
                keys: THROTTLE_US_KEYS.to_owned(),
                pressed: vec![false; THROTTLE_US_KEYS.len()],
                pos: -1,
            },
            brake: Pedal {
                axis: 0.0,
                keys: BRAKE_US_KEYS.to_owned(),
                pressed: vec![false; BRAKE_US_KEYS.len()],
                pos: -1,
            },
            clutch: Pedal {
                axis: 0.0,
                keys: CLUTCH_US_KEYS.to_owned(),
                pressed: vec![false; CLUTCH_US_KEYS.len()],
                pos: -1,
            },
        }
    }

    pub fn get_clutch_axis(&self) -> f32 {
        self.clutch.axis
    }

    pub fn get_brake_axis(&self) -> f32 {
        self.brake.axis
    }

    pub fn get_throttle_axis(&self) -> f32 {
        self.throttle.axis
    }

    pub fn update(&mut self, key: Vk, key_pos: KeyPos, press: PressState) {
        self.throttle.update(key, key_pos, press);
        self.brake.update(key, key_pos, press);
        self.clutch.update(key, key_pos, press);
        if self.brake.axis > 0.0 {
            self.throttle.axis = 0.0; // if braking, ignore throttle
        }
    }

    pub fn dbg(&self) {
        println!(
            "c:{} b:{} a:{}",
            self.clutch.axis, self.brake.axis, self.throttle.axis
        )
    }
}
