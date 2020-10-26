use legion::*;

use crate::typedef::ResourceCount;

pub struct LocalJob {
    entity_demographic: Entity,
    entity_global_job: Entity,
    input_efficency: ResourceCount,
    throughput_efficiency: ResourceCount,
    output_efficiency: ResourceCount,
}

impl LocalJob {
    pub fn new(
        entity_demographic: Entity,
        entity_global_job: Entity,
        input_efficency: ResourceCount,
        throughput_efficiency: ResourceCount,
        output_efficiency: ResourceCount
    ) -> LocalJob {
        LocalJob {
            entity_demographic,
            entity_global_job,
            input_efficency,
            throughput_efficiency,
            output_efficiency,
        }
    }

    pub fn get_entity_demographic(&self) -> Entity {
        self.entity_demographic
    }

    pub fn get_entity_global_job(&self) -> Entity {
        self.entity_global_job
    }

    pub fn get_input_efficiency(&self) -> ResourceCount {
        self.input_efficency
    }

    pub fn get_throughput_efficiency(&self) -> ResourceCount {
        self.throughput_efficiency
    }

    pub fn get_output_efficiency(&self) -> ResourceCount {
        self.output_efficiency
    }
}