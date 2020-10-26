use legion::*;
use std::collections::HashMap;

use super::{resources::ResourceType, super::components::{GlobalJob, Stocks}};

#[derive(Hash, Clone, Eq, PartialEq)]
pub enum GlobalJobType {
    Farmer,
    Rancher,
}

fn make_global_job(global_job_type: GlobalJobType, resources: &HashMap<ResourceType, Entity>) -> (GlobalJobType, GlobalJob) {
    match global_job_type {
        GlobalJobType::Farmer => (global_job_type, 
            GlobalJob::new(
                "Farmer".into(), 
                Stocks::new({
                    let mut farmer_base_inputs = HashMap::new();

                    farmer_base_inputs
                }), 
                Stocks::new({
                    let mut farmer_base_outputs = HashMap::new();

                    farmer_base_outputs.insert(*resources.get(&ResourceType::Food).unwrap(), 1f64);

                    farmer_base_outputs
                }),
            )
        ),
        GlobalJobType::Rancher => (global_job_type, 
            GlobalJob::new(
                "Rancher".into(),
                Stocks::new({
                    let mut rancher_base_inputs = HashMap::new();

                    rancher_base_inputs.insert(*resources.get(&ResourceType::Food).unwrap(), 0.5f64);

                    rancher_base_inputs
                }),
                Stocks::new({
                    let mut rancher_base_outputs = HashMap::new();

                    rancher_base_outputs.insert(*resources.get(&ResourceType::Transportation).unwrap(), 1f64);

                    rancher_base_outputs
                }),
            )
        )
    }
}

pub fn make(world: &mut World, resources: &HashMap<ResourceType, Entity>) -> HashMap<GlobalJobType, Entity> {
    vec!(GlobalJobType::Farmer, GlobalJobType::Rancher).drain(..).map(|global_job_type| {
        let (global_job_type, global_job) = make_global_job(global_job_type, resources);
        (global_job_type, world.push((global_job, )))
    }).collect()
}