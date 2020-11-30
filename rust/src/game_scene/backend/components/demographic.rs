use specs::prelude::*;

use crate::typedef::DemographicCount;

#[derive(Clone)]
pub struct Demographic {
    count: DemographicCount
}

impl Component for Demographic {
    type Storage = VecStorage<Self>;
}

impl Demographic {
    pub fn new(
        count: DemographicCount
    ) -> Demographic {
        Demographic {
            count,
        }
    }

    pub fn get_count(&self) -> DemographicCount {
        self.count
    }
}