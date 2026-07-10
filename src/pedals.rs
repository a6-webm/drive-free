use drive_free::event::*;
use winapi::um::winuser::*;

const THROTTLE_US_KEYS: &[Vk] = &[
    VK_LSHIFT,
    VK_Z,
    VK_X,
    VK_C,
    VK_V,
    VK_B,
    VK_N,
    VK_M,
    VK_OEM_COMMA,
    VK_OEM_PERIOD,
    VK_OEM_2,
    VK_RSHIFT,
];

const BRAKE_US_KEYS: &[Vk] = &[
    VK_CAPITAL, VK_A, VK_S, VK_D, VK_F, VK_G, VK_H, VK_J, VK_K, VK_L, VK_OEM_1, VK_OEM_7, VK_RETURN,
];

const CLUTCH_US_KEYS: &[Vk] = &[
    VK_OEM_3,
    VK_1,
    VK_2,
    VK_3,
    VK_4,
    VK_5,
    VK_6,
    VK_7,
    VK_8,
    VK_9,
    VK_0,
    VK_OEM_MINUS,
    VK_OEM_PLUS,
    VK_BACK,
];

struct Pedal {
    axis: i16,
    keys: Vec<Vk>,
    pressed: Vec<bool>,
    pos: isize,
}

impl Pedal {
    fn update(&mut self, key: Vk, press: PressState) {
        self.update_pos(key, press);
        self.axis = self.calc_axis();
    }

    fn update_pos(&mut self, key: Vk, press: PressState) {
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

    fn calc_axis(&self) -> i16 {
        if self.pos == self.keys.len() as isize - 1 {
            return i16::MAX;
        }
        let origin = i16::MIN as f32;
        let index = (self.pos + 1) as f32;
        let step_size = i16::MAX as f32 * 2.0 / self.keys.len() as f32;
        (origin + index * step_size) as i16
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
                axis: i16::MIN,
                keys: THROTTLE_US_KEYS.to_owned(),
                pressed: vec![false; THROTTLE_US_KEYS.len()],
                pos: -1,
            },
            brake: Pedal {
                axis: i16::MIN,
                keys: BRAKE_US_KEYS.to_owned(),
                pressed: vec![false; BRAKE_US_KEYS.len()],
                pos: -1,
            },
            clutch: Pedal {
                axis: i16::MIN,
                keys: CLUTCH_US_KEYS.to_owned(),
                pressed: vec![false; CLUTCH_US_KEYS.len()],
                pos: -1,
            },
        }
    }

    pub fn update(&mut self, key: Vk, press: PressState) {
        self.throttle.update(key, press);
        self.brake.update(key, press);
        self.clutch.update(key, press);

        // if braking, ignore throttle
        if self.brake.axis > i16::MIN {
            self.throttle.axis = i16::MIN;
        }
    }

    pub fn dbg(&self) {
        println!(
            "c:{} b:{} a:{}",
            self.clutch.axis, self.brake.axis, self.throttle.axis
        )
    }
}
