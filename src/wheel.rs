#[derive(Default)]
pub struct WheelState {
    sense: f32,
    pos: i32,
    pub axis: i16,
}

impl WheelState {
    pub fn new(sense: f32) -> Self {
        Self {
            sense,
            ..Default::default()
        }
    }

    pub fn update(&mut self, delta: (i32, i32)) {
        self.pos += delta.0;
        self.axis = (self.pos as f32 * self.sense).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
    }
}
