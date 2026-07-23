const CONST_SENSE: f32 = 1.0 / 160000.0;

pub struct WheelState {
    sense: f32,
    pos: i32,
    pub axis: f32,
}

impl WheelState {
    pub fn new(sense: f32) -> Self {
        Self {
            sense,
            pos: 0,
            axis: 0.5,
        }
    }

    pub fn update(&mut self, delta: (i32, i32)) {
        self.pos += delta.0;
        self.axis = (self.pos as f32 * self.sense * CONST_SENSE + 0.5).clamp(0.0, 1.0);
    }
}
