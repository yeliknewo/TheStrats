use legion::prelude::*;

use crate::typedef::CommuneId;
use super::Stocks;

#[derive(Clone)]
pub struct Commune {
    id: CommuneId,
    local_jobs: Vec<Entity>,
    neighbor_tiles: Vec<Entity>,
    stocks: Option<Stocks>,
}

impl Commune {
    pub fn new(
        id: CommuneId,
        local_jobs: Vec<Entity>,
        neighbor_tiles: Vec<Entity>
    ) -> Commune {
        Commune {
            id,
            local_jobs,
            neighbor_tiles,
            stocks: Some(Default::default()),
        }
    }

    pub fn get_local_jobs(&self) -> &Vec<Entity> {
        &self.local_jobs
    }

    pub fn take_stocks(&mut self) -> Option<Stocks> {
        self.stocks.take()
    }

    pub fn give_stocks(&mut self, stocks: Stocks) {
        self.stocks = Some(stocks);
    }
}