use gdnative::*;

use super::backend::{FrontToBrain, BrainBunch, start, BrainError};

type Owner = Control;

#[derive(NativeClass)]
#[inherit(Owner)]
struct GameManager {
    brain_bunch: BrainBunch,
}

#[methods]
impl GameManager {

    fn _init(_owner: Owner) -> Self {
        GameManager {
            brain_bunch: start(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: Owner) {
        match self.brain_bunch.send_event(FrontToBrain::Init) {
            Ok(()) => (),
            Err(error) => crate::log::empty_error(format!("{}", BrainError::BrainError(Box::new(error), stack!()))),
        }
    }

    #[export]
    fn _notification(&mut self, _owner: Owner, i: i64) {
        if i == Node::NOTIFICATION_WM_QUIT_REQUEST {
            self.stop_backend();
        }
    }

    #[export]
    fn _process(&mut self, _owner: Owner, _delta: f64) {
        for brain_to_front_event in self.brain_bunch.try_get_events() {
            match brain_to_front_event {

            }
        }
    }

    fn stop_backend(&mut self) {
        self.brain_bunch.stop();
    }

}

pub fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<GameManager>();
}