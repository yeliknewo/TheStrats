use gdnative::*;

#[derive(NativeClass)]
#[inherit(Button)]
pub struct MainMenuPlayButton;

const GAME_SCENE_PATH: &str = "res://game_scene.tscn";

#[methods]
impl MainMenuPlayButton {
    fn _init(_owner: Button) -> Self {
        MainMenuPlayButton
    }

    #[export]
    unsafe fn _pressed(&self, owner: Button) {
        if let Some(tree) = &mut owner.get_tree() {
            tree.change_scene(GodotString::from(GAME_SCENE_PATH)).expect("Game Scene could not be loaded");
        }
    }
}

pub fn init(handle: init::InitHandle) {
    handle.add_class::<MainMenuPlayButton>();
}