use std::collections::HashMap;
use legion::prelude::*;

use crate::typedef::ResourceCount;

type StocksMap = HashMap<Entity, ResourceCount>;

#[derive(Clone, Default, Debug)]
pub struct Stocks {
    stocks: StocksMap,
}

impl Stocks {
    pub fn new(
        stocks: StocksMap,
    ) -> Stocks {
        Stocks {
            stocks,
        }
    }

    pub fn get_stocks(&self) -> &StocksMap {
        &self.stocks
    }

    pub fn get_stocks_mut(&mut self) -> &mut StocksMap {
        &mut self.stocks
    }

    pub fn math_stocks<F>(&mut self, other: &Stocks, f: &F) where F: Fn(Option<&ResourceCount>, ResourceCount) -> Option<ResourceCount> {
        for (entity_resource, your_count) in other.get_stocks() {
            let my_count_opt = self.get_stocks().get(entity_resource);
            if let Some(out_count) = f(my_count_opt, *your_count) {
                self.get_stocks_mut().insert(*entity_resource, out_count);
            }
        }
    }

    pub fn multiply_from(&self, multiplier: ResourceCount) -> Stocks {
        let mut new = self.get_stocks().clone();
        for (resource_entity, resource_count) in self.get_stocks() {
            new.insert(*resource_entity, resource_count * multiplier);
        }
        Stocks::new(new)
    }

    pub fn subtract_stocks(&mut self, subtrahend: &Stocks) {
        self.math_stocks(subtrahend, &|my_count_opt: Option<&ResourceCount>, your_count: ResourceCount| match my_count_opt {
            Some(my_count) => Some(my_count - your_count),
            None => Some(-your_count),
        })
    }

    pub fn add_stocks(&mut self, addend: &Stocks) {
        self.math_stocks(addend, &|my_count_opt: Option<&ResourceCount>, your_count: ResourceCount| match my_count_opt {
            Some(my_count) => Some(my_count + your_count),
            None => Some(your_count)
        })
    }
}