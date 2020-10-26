use gdnative::prelude::*;

mod game_manager;
mod backend;

pub fn init(handle: nativescript::init::InitHandle) {
    game_manager::init(handle);
}
