#[derive(Default)]
pub struct Time {
    time: f64,
}

impl Time {
    pub fn new() -> Time {
        Time {
            time: 0f64,
        }
    }

    pub fn add_delta(&mut self, delta: f64) {
        self.time += delta;
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }
}