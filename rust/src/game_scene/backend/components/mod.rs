use specs::prelude::*;

mod global_job;
mod local_job;
mod resource;
mod stocks;
mod demographic;
mod province;

pub use province::Province;
pub use demographic::Demographic;
pub use global_job::GlobalJob;
pub use local_job::LocalJob;
pub use resource::Resource;
pub use stocks::{Stocks, StocksOpt};

pub fn register_entities(world: &mut World) {
    world.register::<Province>();
    world.register::<Demographic>();
    world.register::<GlobalJob>();
    world.register::<LocalJob>();
    world.register::<Resource>();
    world.register::<Stocks>();
    world.register::<StocksOpt>();
}