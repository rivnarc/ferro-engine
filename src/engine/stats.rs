use std::time::Instant;

pub struct FpsCounter {
    pub fps: f32,
    frame_count: u32,
    last_update: Instant,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            fps: 0.0,
            frame_count: 0,
            last_update: Instant::now(),
        }
    }
}

impl FpsCounter {
    pub fn update(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_update.elapsed().as_secs_f32();
        if elapsed >= 1.0 {
            self.fps = self.frame_count as f32 / elapsed;
            self.frame_count = 0;
            self.last_update = Instant::now();
        }
    }
}
