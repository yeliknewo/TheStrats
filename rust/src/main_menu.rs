use gdnative::prelude::*;
use gdnative::api::*;
use gdnative::nativescript::init;

#[derive(NativeClass)]
#[inherit(Button)]
pub struct MainMenuPlayButton;

const GAME_SCENE_PATH: &str = "res://game_scene.tscn";

#[methods]
impl MainMenuPlayButton {
    fn new(_owner: &Button) -> Self {
        MainMenuPlayButton
    }

    #[export]
    fn _pressed(&self, owner: &Button) {
        if let Some(tree) = owner.get_tree() {
            unsafe {tree.assume_safe()}.change_scene(GodotString::from(GAME_SCENE_PATH)).expect("Game Scene could not be loaded");
        }
    }
}

pub fn init(handle: init::InitHandle) {
    handle.add_class::<MainMenuPlayButton>();
}