use crate::typedef::DemographicCount;

#[derive(Clone)]
pub struct Demographic {
    count: DemographicCount
}

impl Demographic {
    pub fn new(
        count: DemographicCount
    ) -> Demographic {
        Demographic {
            count: count,
        }
    }

    pub fn get_count(&self) -> DemographicCount {
        self.count
    }
}