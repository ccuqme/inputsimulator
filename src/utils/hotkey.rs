use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

use cosmic::iced::{keyboard::Key, core::SmolStr};
use evdev_rs::enums::EventCode;
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::error::Result;

use crate::{
    simulator::simulate_keys,
    config::GlobalHotkey,
    utils::{key_to_device_keycode, from_cosmic_key, keycode_to_evkey},
    constants::{HOTKEY_TOGGLE_DELAY_MS, LISTENER_SLEEP_MS},
};

// Add log throttling state
lazy_static::lazy_static! {
    static ref LAST_LOG_TIME: HashMap<&'static str, AtomicU64> = {
        let mut m = HashMap::new();
        m.insert("hotkey_parse_error", AtomicU64::new(0));
        m
    };
}

const LOG_THROTTLE_MS: u64 = 1000; // Only log once per second

fn should_log(key: &'static str) -> bool {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    
    if let Some(last_time) = LAST_LOG_TIME.get(key) {
        let last = last_time.load(Ordering::Relaxed);
        if now - last < LOG_THROTTLE_MS {
            return false;
        }
        last_time.store(now, Ordering::Relaxed);
    }
    true
}

/// Returns the default fallback hotkey with appropriate logging
fn fallback_hotkey() -> Keycode {
    log::warn!("Using fallback hotkey: F8");
    Keycode::F8
}

pub fn start_global_hotkey_listener(
    running: Arc<Mutex<bool>>,
    interval_ms: Arc<Mutex<u64>>,
    selected_keys: Arc<Mutex<Vec<EventCode>>>,
    modifier_mode: Arc<Mutex<crate::config::KeyBehaviorMode>>,
    previous_state: Arc<Mutex<bool>>,
    last_toggle: Arc<Mutex<Option<Instant>>>,
    app_data: Arc<Mutex<crate::config::AppData>>,
) {
    thread::spawn(move || {
        let device_state = DeviceState::new();
        log::info!("Started global hotkey listener");

        loop {
            let keys: Vec<Keycode> = device_state.get_keys();
            
            // Get the hotkey configuration
            let (hotkey, global_keybind) = {
                let app_data_guard = app_data.lock().unwrap();
                let hotkey = validate_hotkey(&app_data_guard);
                (hotkey, app_data_guard.global_keybind.clone())
            };

            // Save default config if needed
            if global_keybind.key.is_empty() {
                let mut app_data_guard = app_data.lock().unwrap();
                app_data_guard.global_keybind = GlobalHotkey::default();
                if let Err(e) = save_config(&app_data_guard) {
                    log::error!("Failed to save default config: {}", e);
                }
            }

            // Check key and modifier state independently
            let key_pressed = keys.contains(&hotkey);
            
            // Check modifier state (true if modifier is not required OR if it's pressed)
            let ctrl_match = !global_keybind.modifiers.ctrl || 
                (keys.contains(&Keycode::LControl) || keys.contains(&Keycode::RControl));
            let alt_match = !global_keybind.modifiers.alt || 
                (keys.contains(&Keycode::LAlt) || keys.contains(&Keycode::RAlt));
            let shift_match = !global_keybind.modifiers.shift || 
                (keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift));
            let super_match = !global_keybind.modifiers.super_key || 
                (keys.contains(&Keycode::LMeta) || keys.contains(&Keycode::RMeta));

            // All conditions must be true for the hotkey to be considered pressed
            let is_hotkey_pressed = key_pressed && ctrl_match && alt_match && shift_match && super_match;

            // Handle hotkey state
            let mut prev_state = previous_state.lock().unwrap();
            if is_hotkey_pressed && !*prev_state {
                let now = Instant::now();
                let mut last_toggle_time = last_toggle.lock().unwrap();

                if last_toggle_time.map_or(true, |last| 
                    now.duration_since(last).as_millis() > HOTKEY_TOGGLE_DELAY_MS as u128) {
                    let mut is_running = running.lock().unwrap();
                    *is_running = !*is_running;
                    
                    if *is_running {
                        log::info!("Hotkey pressed, starting simulation");
                        initialize_simulation(&app_data, &selected_keys, &modifier_mode);
                        start_simulation(
                            Arc::clone(&running),
                            Arc::clone(&interval_ms),
                            Arc::clone(&selected_keys),
                            Arc::clone(&modifier_mode),
                        );
                    } else {
                        log::info!("Hotkey pressed, stopping simulation");
                    }
                    
                    *last_toggle_time = Some(now);
                }
            }

            *prev_state = is_hotkey_pressed;
            thread::sleep(Duration::from_millis(LISTENER_SLEEP_MS));
        }
    });
}

fn save_config(app_data: &crate::config::AppData) -> Result<()> {
    let json = serde_json::to_string(app_data)?;
    std::fs::write("app_data.json", json)?;
    log::debug!("Config saved successfully");
    Ok(())
}

fn initialize_simulation(
    app_data: &Arc<Mutex<crate::config::AppData>>,
    selected_keys: &Arc<Mutex<Vec<EventCode>>>,
    modifier_mode: &Arc<Mutex<crate::config::KeyBehaviorMode>>,
) {
    let app_data = app_data.lock().unwrap();
    let mut keys = selected_keys.lock().unwrap();
    keys.clear();
    *modifier_mode.lock().unwrap() = app_data.modifier_mode;

    log::debug!("Initializing simulation with keys: {:?}", app_data.selected_keys);

    for key in &app_data.selected_keys {
        if let Key::Character(key_str) = key {
            let clean_key = key_str.replace("KEY_KEY_", "KEY_");
            match from_cosmic_key(Key::Character(SmolStr::from(clean_key.clone()))) {
                Some(device_key) => {
                    if let Some(ev_key) = keycode_to_evkey(device_key) {
                        keys.push(EventCode::EV_KEY(ev_key));
                        log::debug!("Added key: {:?}", ev_key);
                    }
                }
                None => log::warn!("Failed to convert cosmic key: {}", clean_key),
            }
        }
    }

    if keys.is_empty() {
        log::warn!("No valid keys initialized for simulation");
    } else {
        log::info!("Simulation initialized with {} keys", keys.len());
    }
}

fn start_simulation(
    running: Arc<Mutex<bool>>,
    interval_ms: Arc<Mutex<u64>>,
    selected_keys: Arc<Mutex<Vec<EventCode>>>,
    modifier_mode: Arc<Mutex<crate::config::KeyBehaviorMode>>,
) {
    thread::spawn(move || {
        if let Err(e) = simulate_keys(running, interval_ms, selected_keys, modifier_mode) {
            log::error!("Failed to simulate keys: {}", e);
        }
    });
}

/// Validates and returns the appropriate `Keycode` for the global hotkey.
///
/// # Arguments
///
/// * `app_data` - Reference to the app's configuration data containing the global hotkey settings
///
/// # Returns
///
/// A valid `Keycode` for the global hotkey. Falls back to `Keycode::F8` if validation fails.
///
/// # Details
///
/// The function handles several key formats:
/// - Empty keys (falls back to default)
/// - Named keys (e.g., "Named(F8)")
/// - Single character keys (e.g., "A", "a")
/// - Prefixed keys (e.g., "KEY_F8")
///
/// Case is handled insensitively for better user experience.
fn validate_hotkey(app_data: &crate::config::AppData) -> Keycode {
    log::debug!("Validating hotkey configuration: {}", app_data.global_keybind.key);
    let default_hotkey = GlobalHotkey::default();
    
    if app_data.global_keybind.key.trim().is_empty() {
        log::warn!("Empty or whitespace-only hotkey configured, falling back to default");
        return key_to_device_keycode(&default_hotkey.key)
            .unwrap_or_else(fallback_hotkey);
    }

    // First, handle Character format (e.g., "Character("K")")
    if app_data.global_keybind.key.starts_with("Character(") {
        let clean_key = app_data.global_keybind.key
            .trim_start_matches("Character(")
            .trim_end_matches(")")
            .trim_matches('"')
            .trim_matches('\'')
            .to_uppercase();
            
        log::debug!("Processing character key: {}", clean_key);
        
        // Use existing key_to_device_keycode from key_utils.rs
        if let Some(keycode) = key_to_device_keycode(&clean_key) {
            log::debug!("Successfully mapped character '{}' to {:?}", clean_key, keycode);
            return keycode;
        }
        
        if should_log("hotkey_parse_error") {
            log::warn!("Failed to map character key: {}", clean_key);
        }
        return fallback_hotkey();
    }

    // Handle Named format and other cases
    let clean_key = app_data.global_keybind.key
        .trim_start_matches("Named(")
        .trim_end_matches(")")
        .trim_start_matches("KEY_")
        .trim()
        .to_uppercase();

    if clean_key.is_empty() {
        log::warn!("Hotkey contains only prefixes, falling back to default");
        return fallback_hotkey();
    }

    // Try direct conversion using existing mapping
    if let Some(keycode) = key_to_device_keycode(&clean_key) {
        log::debug!("Successfully mapped '{}' to {:?}", clean_key, keycode);
        return keycode;
    }

    log::warn!(
        "Hotkey validation failed for '{}', falling back to default",
        app_data.global_keybind.key
    );
    fallback_hotkey()
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
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);

        // Test valid named key with different cases
        let app_data = create_test_app_data("Named(F8)");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);
        let app_data = create_test_app_data("NAMED(F8)");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);
        let app_data = create_test_app_data("named(f8)");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);

        // Test single letters (should be handled by key_to_device_keycode)
        let app_data = create_test_app_data("A");
        assert_eq!(validate_hotkey(&app_data), Keycode::A);
        let app_data = create_test_app_data("a");
        assert_eq!(validate_hotkey(&app_data), Keycode::A);

        // Test invalid key format
        let app_data = create_test_app_data("INVALID_KEY");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);

        // Test valid key with KEY_ prefix in different cases
        let app_data = create_test_app_data("KEY_F9");
        assert_eq!(validate_hotkey(&app_data), Keycode::F9);
        let app_data = create_test_app_data("key_f9");
        assert_eq!(validate_hotkey(&app_data), Keycode::F9);

        // Test prefixed character keys
        let app_data = create_test_app_data("Key_A");
        assert_eq!(validate_hotkey(&app_data), Keycode::A);
        let app_data = create_test_app_data("KEY_A");
        assert_eq!(validate_hotkey(&app_data), Keycode::A);

        // Test invalid configurations
        let app_data = create_test_app_data(" ");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);
        let app_data = create_test_app_data("KEY_");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);
        let app_data = create_test_app_data("Named()");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);

        // Test special characters
        let app_data = create_test_app_data("#");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);

        // Test numbers (should be handled by key_to_device_keycode)
        let app_data = create_test_app_data("1");
        assert_eq!(validate_hotkey(&app_data), Keycode::Key1);

        // Test Character format
        let app_data = create_test_app_data(r#"Character("K")"#);
        assert_eq!(validate_hotkey(&app_data), Keycode::K);
        
        // Test numpad keys
        let app_data = create_test_app_data(r#"Character("KP4")"#);
        assert_eq!(validate_hotkey(&app_data), Keycode::Numpad4);
        
        let app_data = create_test_app_data(r#"Character("KP0")"#);
        assert_eq!(validate_hotkey(&app_data), Keycode::Numpad0);
        
        let app_data = create_test_app_data(r#"Character("KP9")"#);
        assert_eq!(validate_hotkey(&app_data), Keycode::Numpad9);
    }

    #[test]
    fn test_hotkey_whitespace_handling() {
        let app_data = create_test_app_data("  F8  ");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);
        
        let app_data = create_test_app_data("  KEY_F8  ");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);
        
        let app_data = create_test_app_data("  Named(F8)  ");
        assert_eq!(validate_hotkey(&app_data), Keycode::F8);
    }

    #[test]
    fn test_character_key_handling() {
        // Test character keys with modifiers
        let app_data = create_test_app_data(r#"Character("K")"#);
        assert_eq!(validate_hotkey(&app_data), Keycode::K);
        
        // Test lowercase character keys
        let app_data = create_test_app_data(r#"Character("k")"#);
        assert_eq!(validate_hotkey(&app_data), Keycode::K);
    }
}
