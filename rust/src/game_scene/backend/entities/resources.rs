use specs::prelude::*;
use std::collections::HashMap;

use super::super::components::Resource;

#[derive(Hash, Clone, Eq, PartialEq)]
pub enum ResourceType {
    Food,
    Transportation,
}

fn make_resource(resource_type: ResourceType) -> (ResourceType, Resource) {
    match resource_type {
        ResourceType::Food => (resource_type, Resource::new("Food".into(), 1f64)),
        ResourceType::Transportation => (resource_type, Resource::new("Transportation".into(), 1f64)),
    }
}

pub fn make(world: &mut World) -> HashMap<ResourceType, Entity> {
    vec!(ResourceType::Food, ResourceType::Transportation).drain(..).map(|resource_type| {
        let (resource_type, resource) = make_resource(resource_type);
        (resource_type, world.create_entity().with(resource).build())
    }).collect()
}