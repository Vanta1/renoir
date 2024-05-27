use std::time::Instant;

pub struct DeltaTime {
    delta_time: f64,
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
        self.delta_time = self.prev_time.elapsed().as_secs_f64();
        self.prev_time = Instant::now();
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }
}
