use std::thread::JoinHandle;

use super::{BrainError, brain_channel::{new_wombo, FrontChannel, BrainToFront, FrontToBrain}};

pub fn start() -> BrainBunch {
    BrainBunch::new()
}

pub struct BrainBunch {
    brain_thread_handles: Vec<JoinHandle<()>>,
    front_channel: FrontChannel,
}

impl BrainBunch {
    fn new() -> BrainBunch {
        let (front_channel, brain_channel) = new_wombo();
        BrainBunch {
            brain_thread_handles: vec!(std::thread::spawn(move || {
                super::brain_thread::start(brain_channel);
            })),
            front_channel: front_channel,
        }
    }

    pub fn send_event(&mut self, event: FrontToBrain) -> Result<(), BrainError> {
        self.front_channel.send(event)
    }

    pub fn try_get_events(&mut self) -> Vec<BrainToFront> {
        self.front_channel.try_drain()
    }

    pub fn stop(&mut self) -> Vec<Result<(), BrainError>> {
        let mut rs = vec!();

        for _ in 0..self.brain_thread_handles.len() {
            rs.push(self.send_event(FrontToBrain::Exit).map_err(|error| BrainError::BrainError(Box::new(error), stack!())))
        }
        
        for handle in self.brain_thread_handles.drain(..) {
            rs.push(handle.join().map_err(|error| BrainError::BoxError(error, stack!())));
        }

        rs
    }
}