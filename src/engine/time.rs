use std::time::Instant;

pub struct Time {
    last_frame: Instant,
    pub delta: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            delta: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = (now - self.last_frame).as_secs_f32();
        self.last_frame = now;
    }
}
