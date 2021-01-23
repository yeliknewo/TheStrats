pub struct Delta {
    delta: f64,
}

impl Delta {
    pub fn new(delta: f64) -> Delta {
        Delta {
            delta,
        }
    }

    pub fn get_delta(&self) -> f64 {
        self.delta
    }
}