use specs::prelude::*;

use crate::typedef::ProvinceId;

#[derive(Clone)]
pub struct Province {
    id: ProvinceId,
    local_jobs: Vec<Entity>,
    neighbor_tiles: Vec<Entity>,
    upstream: Option<Entity>,
}

impl Component for Province {
    type Storage = DenseVecStorage<Self>;
}

impl Province {
    pub fn new(
        id: ProvinceId,
        local_jobs: Vec<Entity>,
        neighbor_tiles: Vec<Entity>,
        upstream: Option<Entity>
    ) -> Province {
        Province {
            id,
            local_jobs,
            neighbor_tiles,
            upstream,
        }
    }

    pub fn get_local_jobs(&self) -> &Vec<Entity> {
        &self.local_jobs
    }
}