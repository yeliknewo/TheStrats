use specs::prelude::*;

use super::Stocks;

pub struct GlobalJob {
    name: String,
    base_inputs: Stocks,
    base_outputs: Stocks,
}

impl Component for GlobalJob {
    type Storage = VecStorage<Self>;
}

impl GlobalJob {
    pub fn new(
        name: String,
        base_inputs: Stocks,
        base_outputs: Stocks
    ) -> GlobalJob {
        GlobalJob {
            name,
            base_inputs,
            base_outputs,
        }
    }

    pub fn get_base_inputs(&self) -> &Stocks {
        &self.base_inputs
    }

    pub fn get_base_outputs(&self) -> &Stocks {
        &self.base_outputs
    }
}