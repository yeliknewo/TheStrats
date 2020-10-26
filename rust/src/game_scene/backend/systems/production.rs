use legion::{systems::ParallelRunnable, *};

use super::{take_province_stocks, give_province_stocks, super::components::{Province, Stocks, GlobalJob, Demographic, LocalJob}};
use crate::typedef::ResourceCount;

pub fn make() -> System {
    SystemBuilder::new("production")
    .with_query(<Read<Province>>::query())
    .read_component::<GlobalJob>()
    .read_component::<LocalJob>()
    .read_component::<Demographic>()
    .read_component::<Province>()
    .write_component::<Option<Stocks>>()
    .build(|_, world, (), query_province| {

        let q: String = query_province;
        
        let mut province_stocks_map = take_province_stocks(world, query_province);

        for (entity_province, province_stocks) in &mut province_stocks_map {
            let province = world.get_component::<Province>(*entity_province).unwrap_or_else(|| {
                crate::log::empty_error(stack!());
                panic!(stack!())
            });
            for entity_local_job in province.get_local_jobs() {
                let local_job = world.get_component::<LocalJob>(*entity_local_job).unwrap_or_else(|| {
                    crate::log::empty_error(stack!());
                    panic!(stack!())
                });
                let demographic = world.get_component::<Demographic>(local_job.get_entity_demographic()).unwrap_or_else(|| {
                    crate::log::empty_error(stack!());
                    panic!(stack!())
                });
                let global_job = world.get_component::<GlobalJob>(local_job.get_entity_global_job()).unwrap_or_else(|| {
                    crate::log::empty_error(stack!());
                    panic!(stack!())
                });
                let input_efficiency = local_job.get_input_efficiency();
                let throughput_efficiency = local_job.get_throughput_efficiency();
                let throughput = demographic.get_count() as ResourceCount * throughput_efficiency;
                let input_ratio = {
                    let mut current_ratio = 1f64;
                    for (resource_entity, input_resource_count) in global_job.get_base_inputs().get_stocks() {
                        let requested_input = input_resource_count * input_efficiency * throughput;
                        let available_input = province_stocks.get_stocks().get(&resource_entity).unwrap_or(&0f64);
                        let ratio = requested_input / available_input;
                        if ratio < current_ratio {
                            current_ratio = ratio;
                        }
                    }
                    current_ratio
                };
                let input = input_ratio * throughput;
                let actual_input = global_job.get_base_inputs().multiply_from(input);
                let output_efficiency = local_job.get_output_efficiency();
                let output = input_ratio * throughput * output_efficiency;
                let actual_output = global_job.get_base_outputs().multiply_from(output);
                province_stocks.subtract_stocks(&actual_input);
                province_stocks.add_stocks(&actual_output);
            }
        }

        give_province_stocks(world, province_stocks_map);

    })
}