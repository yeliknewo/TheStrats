use std::collections::HashMap;

use super::super::input::KeyCode;

pub struct KeyManager {
    map: HashMap<KeyCode, bool>,
}

impl Default for KeyManager {
    fn default() -> Self {
        KeyManager::new()
    }
}

impl KeyManager {
    pub fn new() -> KeyManager {
        KeyManager {
            map: HashMap::new(),
        }
    }

    pub fn is_key_down(&self, key_code: KeyCode) -> bool {
        if let Some(key_value) = self.map.get(&key_code) {
            return *key_value;
        }
        false
    }

    pub fn set_key(&mut self, key_code: KeyCode, key_down: bool) {
        self.map.insert(key_code, key_down);
    }

    pub fn set_key_down(&mut self, key_code: KeyCode) {
        self.set_key(key_code, true);
    }

    pub fn set_key_up(&mut self, key_code: KeyCode) {
        self.set_key(key_code, false);
    }
}