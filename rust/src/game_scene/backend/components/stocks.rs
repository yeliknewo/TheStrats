use std::collections::HashMap;
use legion::prelude::*;

use crate::typedef::ResourceCount;

pub struct Stocks {
    stocks: HashMap<Entity, ResourceCount>,
}

impl Stocks {
    pub fn new(
        stocks: HashMap<Entity, ResourceCount>,
    ) -> Stocks {
        Stocks {
            stocks: stocks,
        }
    }
}