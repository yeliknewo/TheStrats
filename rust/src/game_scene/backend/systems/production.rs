use specs::prelude::*;
use std::collections::HashMap;

use super::{super::{components::{Province, Stocks, StocksOpt, LocalJob, Demographic, GlobalJob}}};
use crate::typedef::{ResourceCount};

pub struct ProductionSystem;

#[derive(SystemData)]
pub struct ProductionSystemData<'a> {
    provinces: WriteStorage<'a, Province>,
    entities: Entities<'a>,
    stocks_opts: WriteStorage<'a, StocksOpt>,
    local_jobs: ReadStorage<'a, LocalJob>,
    demographics: ReadStorage<'a, Demographic>,
    global_jobs: ReadStorage<'a, GlobalJob>,
}

impl<'a> System<'a> for ProductionSystem {
    type SystemData = ProductionSystemData<'a>;

    fn run(&mut self, mut data: ProductionSystemData) {
        crate::log::full("Enter Production Run".into());

        let mut province_stocks_map = {    
            let mut province_stocks_map = HashMap::new();
            for (entity_province, _province, stocks_opt) in (&data.entities, &data.provinces, &data.stocks_opts).join() {
                province_stocks_map.insert(entity_province, stocks_opt.get().clone().unwrap());
            }
            province_stocks_map
        };

        for (entity_province, province_stocks) in &mut province_stocks_map {
            let province = data.provinces.get(*entity_province).unwrap_or_else(|| {
                    crate::log::empty_error(stack!());
                    panic!(stack!())
            });
            for entity_local_job in province.get_local_jobs() {
                let local_job = data.local_jobs.get(*entity_local_job).unwrap_or_else(|| {
                    crate::log::empty_error(stack!());
                    panic!(stack!())
                });
                let demographic = data.demographics.get(local_job.get_entity_demographic()).unwrap_or_else(|| {
                    crate::log::empty_error(stack!());
                    panic!(stack!())
                });
                let global_job = data.global_jobs.get(local_job.get_entity_global_job()).unwrap_or_else(|| {
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

        for (entity_province, province_stocks) in province_stocks_map.drain() {
            crate::log::full(format!("Stocks 1: {:?}", province_stocks));
            data.stocks_opts.get_mut(entity_province).unwrap().set(Some(province_stocks));
        }
    }
}
