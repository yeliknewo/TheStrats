use legion::*;

use super::{
    systems,
    brain_channel::{
        FrontToBrain, BrainChannel
    }, 
    components::{
        Province, Demographic, Stocks, LocalJob
    }, 
    entities::{self, global_jobs::GlobalJobType},
};

pub fn start(mut brain_channel: BrainChannel) {
    let mut running = true;
    let mut world = World::default();
    let mut schedule = Schedule::builder()
    .add_system(systems::production::make())
    .add_system(systems::trade::make())
    .build();
    let mut resources = Resources::default();
    while running {
        for event in brain_channel.try_drain() {
             match event {
                FrontToBrain::Init => {

                    crate::log::full("Init Recieved".into());

                    let resources = entities::resources::make(&mut world);

                    let demographic = Demographic::new(10);

                    let demographic_entity = world.push((demographic,));

                    let global_jobs = entities::global_jobs::make(&mut world, &resources);

                    let farmer_global_job_entity = world.push((*global_jobs.get(&GlobalJobType::Farmer).unwrap(),));

                    let farmer_local_job = LocalJob::new(demographic_entity, farmer_global_job_entity, 1.0, 1.0, 1.0);

                    let farmer_local_job_entity = world.push((farmer_local_job,));

                    let province = Province::new(0, vec!(farmer_local_job_entity), vec!(), None);

                    let province_entity = world.push((province, Some(Stocks::default())));

                    crate::log::full("Exit Init".into());

                }
                FrontToBrain::Exit => running = false,
            }
        }
        schedule.execute(&mut world, &mut resources);
    }
}