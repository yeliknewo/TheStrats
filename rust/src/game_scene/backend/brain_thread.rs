use legion::prelude::*;
use std::collections::HashMap;

use crate::typedef::ResourceCount;
use super::{BrainError, brain_channel::{FrontToBrain, BrainChannel}, components::{Commune, GlobalJob, Demographic, Resource, Stocks, LocalJob}};

pub fn start(mut brain_channel: BrainChannel) {
    let mut running = true;
    let universe = Universe::new();
    let mut world = universe.create_world();
    while running {
        match brain_channel.get_event() {
            Ok(event) => match event {
                FrontToBrain::Init => {
                    
                    let food = Resource::new("Food".to_string());

                    let food_entity = world.insert((), vec!((food,)))[0];

                    let demographic = Demographic::new(10);

                    let demographic_entity = world.insert((), vec!((demographic,)))[0];

                    let farmer_base_inputs = Stocks::new({
                        let mut map = HashMap::<Entity, ResourceCount>::new();

                        map.insert(food_entity, 0.0);

                        map
                    });

                    let farmer_base_outputs = Stocks::new({
                        let mut map = HashMap::<Entity, ResourceCount>::new();

                        map.insert(food_entity, 1.0);

                        map
                    });

                    let farmer_global_job = GlobalJob::new("Farmer".to_string(), farmer_base_inputs, farmer_base_outputs);

                    let farmer_global_job_entity = world.insert((), vec!((farmer_global_job,)))[0];

                    let farmer_local_job = LocalJob::new(demographic_entity, farmer_global_job_entity, 1.0, 1.0, 1.0);

                    let farmer_local_job_entity = world.insert((), vec!((farmer_local_job,)))[0];

                    let commune = Commune::new(0, vec!(farmer_global_job_entity), vec!());

                    let commune_entity = world.insert((), vec!((commune,)))[0];

                }
                FrontToBrain::Exit => running = false,
            },
            Err(error) => crate::log::empty_error(format!("{}", BrainError::BrainError(Box::new(error), stack!()))),
        }
    }
}