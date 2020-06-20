use crate::typedef::DemographicCount;

#[derive(Clone, Debug, PartialEq)]
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
}