use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct TestScript;

#[methods]
impl TestScript {
    fn _init(_owner: Control) -> Self {
        TestScript
    }

    #[export]
    fn _ready(&self, _owner: Control) {
        godot_print!("Hello, world!")
    }
}

pub fn init(handle: nativescript::init::InitHandle) {
    handle.add_class::<TestScript>();
}