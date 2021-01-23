use gdnative::prelude::*;
use gdnative::api::*;
use gdnative::nativescript::init;

use super::backend::{FrontToBrain, BrainToFront, BrainBunch, start, BrainError, input::KeyCode};

type Owner = Control;

#[derive(NativeClass)]
#[inherit(Owner)]
struct GameManager {
    brain_bunch: BrainBunch,
}

#[methods]
impl GameManager {

    fn new(_owner: &Owner) -> Self {
        GameManager {
            brain_bunch: start(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Owner) {
        match self.brain_bunch.send_event(FrontToBrain::Init) {
            Ok(()) => (),
            Err(error) => crate::log::empty_error(format!("{}", BrainError::BrainError(Box::new(error), stack!()))),
        }
    }

    #[export]
    fn _notification(&mut self, _owner: &Owner, i: i64) {
        if i == Node::NOTIFICATION_WM_QUIT_REQUEST {
            self.stop_backend();
        }
    }

    #[export]
    fn _process(&mut self, _owner: &Owner, delta: f64) {
        for brain_to_front_event in self.brain_bunch.try_get_events() {
            match brain_to_front_event {
                BrainToFront::Log(msg) => crate::log::empty(msg),
            }
        }
        self.brain_bunch.send_event(FrontToBrain::Delta(delta)).ok();
        if Input::godot_singleton().is_action_just_pressed("ui_accept") {
            self.brain_bunch.send_event(FrontToBrain::KeyDown(KeyCode::UiAccept)).ok();
        }
        if Input::godot_singleton().is_action_just_released("ui_accept") {
            self.brain_bunch.send_event(FrontToBrain::KeyUp(KeyCode::UiAccept)).ok();
        }
    }

    fn stop_backend(&mut self) {
        for rs in self.brain_bunch.stop() {
            match rs {
                Ok(()) => (),
                Err(error) => crate::log::empty_error(format!("{}", BrainError::BrainError(Box::new(error), stack!()))),
            }
        }
    }

}

pub fn init(handle: init::InitHandle) {
    handle.add_class::<GameManager>();
}