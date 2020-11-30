use gdnative::*;

mod game_manager;
mod backend;

pub fn init(handle: init::InitHandle) {
    game_manager::init(handle);
}
