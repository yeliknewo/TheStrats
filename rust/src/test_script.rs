use gdnative::*;

#[derive(NativeClass)]
#[inherit(Control)]
pub struct TestScript;

#[methods]
impl TestScript {
    fn _init(_owner: Control) -> Self {
        TestScript
    }

    #[export]
    fn _ready(&self, _owner: gdnative::Control) {
        godot_print!("Hello, world!")
    }
}

pub fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<TestScript>();
}