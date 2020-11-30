SystemBuilder::new("trade")
    .with_query(<Read<Province>>::query())
    .write_component::<Option<Stocks>>()
    .build(|command_buffer, world, (), query_province| {
        let mut province_stocks_map = {    
            let mut province_stocks_map = HashMap::new();
            let mut entity_provinces = vec!();
            for (entity_province, _province) in query_province.iter_entities_mut(world) {
                entity_provinces.push(entity_province);
            }
            for entity_province in entity_provinces {
                province_stocks_map.insert(entity_province, world.get_component_mut::<Option<Stocks>>(entity_province).unwrap_or_else(|| {
                    crate::log::empty_error(stack!());
                    panic!(stack!());
                }).take().unwrap());
            }
            province_stocks_map
        };
   
        

        for (entity_province, province_stocks) in province_stocks_map.drain() {
            crate::log::full(format!("Stocks 1: {:?}", province_stocks));
            if let Some(_old_province_stocks) = world.get_component_mut::<Option<Stocks>>(entity_province).unwrap_or_else(|| {
                crate::log::empty_error(stack!());
                panic!(stack!())
            }).replace(province_stocks) {
                crate::log::empty_error(stack!());
                panic!(stack!())
            }
        }
    })