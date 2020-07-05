use legion::prelude::*;
use std::collections::HashMap;

use super::super::components::{Commune, Stocks, GlobalJob, Demographic, LocalJob};
use crate::typedef::ResourceCount;

pub fn make_production_system() -> Box<dyn Schedulable> {
    SystemBuilder::new("production")
    .with_query(<Read<Commune>>::query())
    .read_component::<GlobalJob>()
    .read_component::<LocalJob>()
    .read_component::<Demographic>()
    .read_component::<Commune>()
    .write_component::<Stocks>()
    .write_component::<Option<Stocks>>()
    .build(|_, world, (), query_commune| {
        let mut commune_stocks_map = {
            let mut commune_stocks_map = HashMap::new();
            let mut entity_communes = vec!();
            for (entity_commune, _commune) in query_commune.iter_entities(world) {
                entity_communes.push(entity_commune);
            }
            for entity_commune in entity_communes {
                commune_stocks_map.insert(entity_commune, world.get_component_mut::<Option<Stocks>>(entity_commune).unwrap_or_else(|| {
                    crate::log::empty_error(stack!());
                    panic!(stack!());
                }).take().unwrap());
            }
            commune_stocks_map
        };

        for (entity_commune, commune_stocks) in &mut commune_stocks_map {
            let commune = world.get_component::<Commune>(*entity_commune).unwrap_or_else(|| {
                crate::log::empty_error(stack!());
                panic!(stack!())
            });
            for entity_local_job in commune.get_local_jobs() {
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
                        let available_input = commune_stocks.get_stocks().get(&resource_entity).unwrap_or(&0f64);
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
                commune_stocks.subtract_stocks(&actual_input);
                commune_stocks.add_stocks(&actual_output);
            }
        }

        for (entity_commune, commune_stocks) in commune_stocks_map.drain() {
            crate::log::full(format!("Stocks 1: {:?}", commune_stocks));
            if let Some(_old_commune_stocks) = world.get_component_mut::<Option<Stocks>>(entity_commune).unwrap_or_else(|| {
                crate::log::empty_error(stack!());
                panic!(stack!())
            }).replace(commune_stocks) {
                crate::log::empty_error(stack!());
                panic!(stack!())
            }
        }

    })
}