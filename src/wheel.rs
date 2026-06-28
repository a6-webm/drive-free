use std::f32::consts::PI;

#[derive(Default)]
pub struct WheelState {
    deadzone: i32,
    max_rot: f32,
    pos: (i32, i32),
    half_rot_index: i32,
    axis: i16,
    dbg: u32,
}

impl WheelState {
    pub fn new(deadzone: i32, max_rot: f32) -> Self {
        Self {
            deadzone,
            max_rot,
            ..Default::default()
        }
    }

    pub fn update(&mut self, delta: (i32, i32)) {
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
