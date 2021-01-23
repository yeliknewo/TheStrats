use gdnative::prelude::*;
use gdnative::api::*;
use gdnative::nativescript::init;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct TestScript;

#[methods]
impl TestScript {
    fn new(_owner: &Control) -> Self {
        TestScript
    }

    #[export]
    fn _ready(&self, _owner: &Control) {
        godot_print!("Hello, world!")
    }
}

pub fn init(handle: init::InitHandle) {
    handle.add_class::<TestScript>();
}