use super::Stocks;

pub struct GlobalJob {
    name: String,
    base_inputs: Stocks,
    base_outputs: Stocks,
}

impl GlobalJob {
    pub fn new(
        name: String,
        base_inputs: Stocks,
        base_outputs: Stocks
    ) -> GlobalJob {
        GlobalJob {
            name: name,
            base_inputs: base_inputs,
            base_outputs: base_outputs,
        }
    }
}