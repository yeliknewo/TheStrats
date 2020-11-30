use specs::prelude::*;

use super::{super::{components::{Province}}};

struct ProductionSystem;

#[derive(SystemData)]
struct ProductionSystemData<'a> {
    province: WriteStorage<'a, Province>,
}

impl<'a> System<'a> for ProductionSystem {
    type SystemData = ProductionSystemData<'a>;

    fn run(&mut self, data: ProductionSystemData) {
    //     SystemBuilder::new("production")
    // .with_query(<Read<Province>>::query())
    // .read_component::<GlobalJob>()
    // .read_component::<LocalJob>()
    // .read_component::<Demographic>()
    // .read_component::<Province>()
    // .write_component::<Option<Stocks>>()
    // .build(|_, world, (), query_province| {
        
    //     let mut province_stocks_map = {    
    //         let mut province_stocks_map = HashMap::new();
    //         let mut entity_provinces = vec!();
    //         for (entity_province, _province) in query_province.iter_entities(world) {
    //             entity_provinces.push(entity_province);
    //         }
    //         for entity_province in entity_provinces {
    //             province_stocks_map.insert(entity_province, world.get_component_mut::<Option<Stocks>>(entity_province).unwrap_or_else(|| {
    //                 crate::log::empty_error(stack!());
    //                 panic!(stack!());
    //             }).take().unwrap());
    //         }
    //         province_stocks_map
    //     };

    //     for (entity_province, province_stocks) in &mut province_stocks_map {
    //         let province = world.get_component::<Province>(*entity_province).unwrap_or_else(|| {
    //             crate::log::empty_error(stack!());
    //             panic!(stack!())
    //         });
    //         for entity_local_job in province.get_local_jobs() {
    //             let local_job = world.get_component::<LocalJob>(*entity_local_job).unwrap_or_else(|| {
    //                 crate::log::empty_error(stack!());
    //                 panic!(stack!())
    //             });
    //             let demographic = world.get_component::<Demographic>(local_job.get_entity_demographic()).unwrap_or_else(|| {
    //                 crate::log::empty_error(stack!());
    //                 panic!(stack!())
    //             });
    //             let global_job = world.get_component::<GlobalJob>(local_job.get_entity_global_job()).unwrap_or_else(|| {
    //                 crate::log::empty_error(stack!());
    //                 panic!(stack!())
    //             });
    //             let input_efficiency = local_job.get_input_efficiency();
    //             let throughput_efficiency = local_job.get_throughput_efficiency();
    //             let throughput = demographic.get_count() as ResourceCount * throughput_efficiency;
    //             let input_ratio = {
    //                 let mut current_ratio = 1f64;
    //                 for (resource_entity, input_resource_count) in global_job.get_base_inputs().get_stocks() {
    //                     let requested_input = input_resource_count * input_efficiency * throughput;
    //                     let available_input = province_stocks.get_stocks().get(&resource_entity).unwrap_or(&0f64);
    //                     let ratio = requested_input / available_input;
    //                     if ratio < current_ratio {
    //                         current_ratio = ratio;
    //                     }
    //                 }
    //                 current_ratio
    //             };
    //             let input = input_ratio * throughput;
    //             let actual_input = global_job.get_base_inputs().multiply_from(input);
    //             let output_efficiency = local_job.get_output_efficiency();
    //             let output = input_ratio * throughput * output_efficiency;
    //             let actual_output = global_job.get_base_outputs().multiply_from(output);
    //             province_stocks.subtract_stocks(&actual_input);
    //             province_stocks.add_stocks(&actual_output);
    //         }
    //     }

    //     for (entity_province, province_stocks) in province_stocks_map.drain() {
    //         crate::log::full(format!("Stocks 1: {:?}", province_stocks));
    //         if let Some(_old_province_stocks) = world.get_component_mut::<Option<Stocks>>(entity_province).unwrap_or_else(|| {
    //             crate::log::empty_error(stack!());
    //             panic!(stack!())
    //         }).replace(province_stocks) {
    //             crate::log::empty_error(stack!());
    //             panic!(stack!())
    //         }
    //     }

    // })
    }
}
