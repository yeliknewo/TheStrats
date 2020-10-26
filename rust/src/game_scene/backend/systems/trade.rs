use legion::{systems::ParallelRunnable, *};

use super::{take_province_stocks, give_province_stocks, super::components::{Province, Stocks}};

pub fn make() -> System {
    SystemBuilder::new("trade")
    .with_query(<Read<Province>>::query())
    .write_component::<Option<Stocks>>()
    .build(|command_buffer, world, (), query_province| {
        let mut province_stocks_map = take_province_stocks(world, query_province);
   
        

        give_province_stocks(world, province_stocks_map);
    })
}
