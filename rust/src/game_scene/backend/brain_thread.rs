use specs::prelude::*;
use std::collections::HashMap;

use super::{
    brain_channel::{
        FrontToBrain, BrainChannel, BrainToFront
    }, 
    components::{self, 
        Province, Demographic, StocksOpt, LocalJob, GlobalJob
    }, 
    entities::{self, global_jobs::GlobalJobType},
    systems::{production::ProductionSystem, trade::TradeSystem},
};

pub fn start(mut brain_channel: BrainChannel) {
    let mut running = true;
    let mut world = World::new();
    let mut dispatcher = DispatcherBuilder::new()
    .with(ProductionSystem, "ProductionSystem", &[])
    .with(TradeSystem, "TradeSystem", &["ProductionSystem"])
    .build();
    components::register_entities(&mut world);
    while running {
        crate::log::full("Entering run loop".into());
        for event in brain_channel.try_drain() {
             match event {
                FrontToBrain::Init => {

                    crate::log::full("Init Recieved".into());

                    let resources = entities::resources::make(&mut world);

                    let demographic = Demographic::new(10);

                    let demographic_entity = world.create_entity().with(demographic).build();


                    let global_jobs = entities::global_jobs::make(&mut world, &resources);

                    let farmer_global_job_entity = *global_jobs.get(&GlobalJobType::Farmer).unwrap();

                    let farmer_local_job = LocalJob::new(demographic_entity, farmer_global_job_entity, 1.0, 1.0, 1.0);

                    let farmer_local_job_entity = world.create_entity().with(farmer_local_job).build();

                    let province = Province::new(0, vec!(farmer_local_job_entity), vec!(), None);

                    let province_entity = world.create_entity().with(province).with(StocksOpt::default()).build();

                    crate::log::full("Exit Init".into());

                }
                FrontToBrain::Exit => running = false,
            }
        }
        crate::log::full("Starting dispatch".into());
        dispatcher.dispatch(&world);
        crate::log::full("End of run loop".into());
    }
}