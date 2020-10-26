use gdnative::prelude::*;

#[macro_use]
pub mod macros;
mod test_script;
mod main_menu;
mod game_scene;

mod typedef {
    pub type ProvinceId = u64;
    pub type DemographicCount = u64;
    pub type ResourceCount = f64;
}

pub mod log {
    use gdnative::*;

    const LOG_LEVEL: LogLevel = LogLevel::Full;

    #[derive(Eq, PartialEq)]
    enum LogLevel {
        Full,
        Small,
        Empty,
    }

    fn should_log(log_level: LogLevel) -> bool {
        match LOG_LEVEL {
            LogLevel::Full => true,
            LogLevel::Small => log_level == LogLevel::Small || log_level == LogLevel::Empty,
            LogLevel::Empty => log_level == LogLevel::Empty,
        }
    }

    fn log(log_level: LogLevel, message: String) {
        if should_log(log_level) {
            godot_print!("{}", message);
        }
    }

    fn error(log_level: LogLevel, message: String) {
        if should_log(log_level) {
            godot_error!("{}", message);
        }
    }

    pub fn full(message: String) {
        log(LogLevel::Full, message);
    }

    pub fn full_error(message: String) {
        error(LogLevel::Full, message);
    }

    pub fn small(message: String) {
        log(LogLevel::Small, message);
    }

    pub fn small_error(message: String) {
        error(LogLevel::Small, message);
    }

    pub fn empty(message: String) {
        log(LogLevel::Empty, message);
    }

    pub fn empty_error(message: String) {
        error(LogLevel::Empty, message);
    }
}

fn init(handle: nativescript::init::InitHandle) {
    test_script::init(handle);
    main_menu::init(handle);
    game_scene::init(handle);
}

godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();