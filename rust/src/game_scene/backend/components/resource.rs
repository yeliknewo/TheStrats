use crate::typedef::ResourceCount;

pub struct Resource {
    name: String,
    transportation_cost: ResourceCount,
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