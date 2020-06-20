use legion::prelude::*;

use crate::typedef::ResourceCount;

pub struct LocalJob {
    demographic_entity: Entity,
    global_job_entity: Entity,
    input_efficency: ResourceCount,
    throughput_efficiency: ResourceCount,
    output_efficiency: ResourceCount,
}

impl LocalJob {
    pub fn new(
        demographic_entity: Entity,
        global_job_entity: Entity,
        input_efficency: ResourceCount,
        throughput_efficiency: ResourceCount,
        output_efficiency: ResourceCount
    ) -> LocalJob {
        LocalJob {
            demographic_entity: demographic_entity,
            global_job_entity: global_job_entity,
            input_efficency: input_efficency,
            throughput_efficiency: throughput_efficiency,
            output_efficiency: output_efficiency,
        }
    }
}