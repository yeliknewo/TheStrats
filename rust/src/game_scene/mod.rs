mod game_manager;
mod backend;

pub fn init(handle: gdnative::init::InitHandle) {
    game_manager::init(handle);
}
