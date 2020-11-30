use specs::prelude::*;

use crate::typedef::ResourceCount;

pub struct Resource {
    name: String,
    transportation_cost: ResourceCount,
}

impl Component for Resource {
    type Storage = VecStorage<Self>;
}

impl Resource {
    pub fn new(
        name: String,
        transportation_cost: ResourceCount,
    ) -> Resource {
        Resource {
            name,
            transportation_cost,
        }
    }
}