use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use evdev_rs::enums::EventCode;
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::{
    config::GlobalHotkey,
    constants::{LISTENER_SLEEP_MS},
};

use crate::utils::persistence::save_app_data;

// New helper function extracting hotkey matching logic.
fn is_hotkey_active(keys: &Vec<device_query::Keycode>, hotkey: device_query::Keycode, global_keybind: &crate::config::GlobalHotkey) -> bool {
    // Check primary hotkey
    let key_pressed = keys.contains(&hotkey);
    // Check modifiers: only required if flagged true.
    let ctrl_match = !global_keybind.modifiers.ctrl || (keys.contains(&device_query::Keycode::LControl) || keys.contains(&device_query::Keycode::RControl));
    let alt_match = !global_keybind.modifiers.alt || (keys.contains(&device_query::Keycode::LAlt) || keys.contains(&device_query::Keycode::RAlt));
    let shift_match = !global_keybind.modifiers.shift || (keys.contains(&device_query::Keycode::LShift) || keys.contains(&device_query::Keycode::RShift));
    let super_match = !global_keybind.modifiers.super_key || (keys.contains(&device_query::Keycode::LMeta) || keys.contains(&device_query::Keycode::RMeta));
    key_pressed && ctrl_match && alt_match && shift_match && super_match
}

pub fn start_global_hotkey_listener(
    _running: Arc<Mutex<bool>>,
    _interval_ms: Arc<Mutex<u64>>,
    _selected_keys: Arc<Mutex<Vec<EventCode>>>,
    _key_behavior: Arc<Mutex<crate::config::KeyBehaviorMode>>,
    previous_state: Arc<Mutex<bool>>,
    _last_toggle: Arc<Mutex<Option<Instant>>>,
    app_data: Arc<Mutex<crate::config::AppData>>,
    on_hotkey: Arc<dyn Fn() + Send + Sync>, // new callback parameter
) {
    thread::spawn(move || {
        let device_state = DeviceState::new();
        log::info!("Started global hotkey listener");

        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
                
            // Cache hotkey configuration.
            let (hotkey, global_keybind) = {
                let mut app_data_guard = app_data.lock().unwrap();
                if app_data_guard.global_keybind.key.is_empty() {
                    app_data_guard.global_keybind = GlobalHotkey::default();
                    if let Err(e) = save_app_data(&mut app_data_guard) {
                        log::error!("Failed to save default config: {}", e);
                    }
                }
                let hotkey = crate::utils::key_utils::validate_hotkey(&app_data_guard);
                let global_keybind = app_data_guard.global_keybind.clone();
                (hotkey, global_keybind)
            };

            // Use helper function for hotkey matching.
            let is_hotkey_pressed = is_hotkey_active(&keys, hotkey, &global_keybind);

            // Handle hotkey state.
            let mut prev_state = previous_state.lock().unwrap();
            if is_hotkey_pressed && !*prev_state {
                (on_hotkey)();
            }
            *prev_state = is_hotkey_pressed;
            thread::sleep(Duration::from_millis(LISTENER_SLEEP_MS));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppData;

    fn create_test_app_data(key: &str) -> AppData {
        let mut app_data = AppData::default();
        app_data.global_keybind.key = key.to_string();
        app_data
    }

    #[test]
    fn test_validate_hotkey() {
        // Test empty hotkey
        let app_data = create_test_app_data("");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);

        // Test valid named key with different cases
        let app_data = create_test_app_data("Named(F8)");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);
        let app_data = create_test_app_data("NAMED(F8)");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);
        let app_data = create_test_app_data("named(f8)");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);

        // Test single letters (should be handled by key_to_device_keycode)
        let app_data = create_test_app_data("A");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::A);
        let app_data = create_test_app_data("a");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::A);

        // Test invalid key format
        let app_data = create_test_app_data("INVALID_KEY");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);

        // Test valid key with KEY_ prefix in different cases
        let app_data = create_test_app_data("KEY_F9");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F9);
        let app_data = create_test_app_data("key_f9");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F9);

        // Test prefixed character keys
        let app_data = create_test_app_data("Key_A");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::A);
        let app_data = create_test_app_data("KEY_A");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::A);

        // Test invalid configurations
        let app_data = create_test_app_data(" ");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);
        let app_data = create_test_app_data("KEY_");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);
        let app_data = create_test_app_data("Named()");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);

        // Test special characters
        let app_data = create_test_app_data("#");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);

        // Test numbers (should be handled by key_to_device_keycode)
        let app_data = create_test_app_data("1");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::Key1);

        // Test Character format
        let app_data = create_test_app_data(r#"Character("K")"#);
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::K);
        
        // Test numpad keys
        let app_data = create_test_app_data(r#"Character("KP4")"#);
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::Numpad4);
        
        let app_data = create_test_app_data(r#"Character("KP0")"#);
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::Numpad0);
        
        let app_data = create_test_app_data(r#"Character("KP9")"#);
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::Numpad9);
    }

    #[test]
    fn test_hotkey_whitespace_handling() {
        let app_data = create_test_app_data("  F8  ");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);
        
        let app_data = create_test_app_data("  KEY_F8  ");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);
        
        let app_data = create_test_app_data("  Named(F8)  ");
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::F8);
    }

    #[test]
    fn test_character_key_handling() {
        // Test character keys with modifiers
        let app_data = create_test_app_data(r#"Character("K")"#);
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::K);
        
        // Test lowercase character keys
        let app_data = create_test_app_data(r#"Character("k")"#);
        assert_eq!(crate::utils::key_utils::validate_hotkey(&app_data), Keycode::K);
    }
}
