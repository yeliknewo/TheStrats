use gdnative::{prelude::*, api::*, nativescript::init};

mod game_manager;
mod backend;

pub fn init(handle: init::InitHandle) {
    game_manager::init(handle);
}
