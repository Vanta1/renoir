use std::time::Instant;

pub struct DeltaTime {
    delta_time: f32,
    prev_time: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self {
            delta_time: 0.,
            prev_time: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.delta_time = self.prev_time.elapsed().as_secs_f32();
        self.prev_time = Instant::now();
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
}

impl Default for DeltaTime {
    fn default() -> Self {
        Self::new()
    }
}
