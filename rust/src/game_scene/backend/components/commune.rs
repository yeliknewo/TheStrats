use legion::prelude::*;

use crate::typedef::CommuneId;

#[derive(Clone, Debug, PartialEq)]
pub struct Commune {
    id: CommuneId,
    local_jobs: Vec<Entity>,
    neighbor_tiles: Vec<Entity>,
}

impl Commune {
    pub fn new(
        id: CommuneId,
        local_jobs: Vec<Entity>,
        neighbor_tiles: Vec<Entity>
    ) -> Commune {
        Commune {
            id: id,
            local_jobs: local_jobs,
            neighbor_tiles: neighbor_tiles,
        }
    }
}