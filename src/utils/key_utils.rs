use device_query::Keycode;
use evdev_rs::enums::EV_KEY;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    pub(crate) static ref KEY_MAPPINGS: HashMap<&'static str, (Keycode, EV_KEY)> = {
        let mut m = HashMap::new();
        // Mouse buttons (Using F13-F15 for device_query compatibility)
        m.insert("BTN_LEFT", (Keycode::F13, EV_KEY::BTN_LEFT));
        m.insert("BTN_MIDDLE", (Keycode::F14, EV_KEY::BTN_MIDDLE));
        m.insert("BTN_RIGHT", (Keycode::F15, EV_KEY::BTN_RIGHT));

        // Letter keys
        m.insert("A", (Keycode::A, EV_KEY::KEY_A));
        m.insert("B", (Keycode::B, EV_KEY::KEY_B));
        m.insert("C", (Keycode::C, EV_KEY::KEY_C));
        m.insert("D", (Keycode::D, EV_KEY::KEY_D));
        m.insert("E", (Keycode::E, EV_KEY::KEY_E));
        m.insert("F", (Keycode::F, EV_KEY::KEY_F));
        m.insert("G", (Keycode::G, EV_KEY::KEY_G));
        m.insert("H", (Keycode::H, EV_KEY::KEY_H));
        m.insert("I", (Keycode::I, EV_KEY::KEY_I));
        m.insert("J", (Keycode::J, EV_KEY::KEY_J));
        m.insert("K", (Keycode::K, EV_KEY::KEY_K));
        m.insert("L", (Keycode::L, EV_KEY::KEY_L));
        m.insert("M", (Keycode::M, EV_KEY::KEY_M));
        m.insert("N", (Keycode::N, EV_KEY::KEY_N));
        m.insert("O", (Keycode::O, EV_KEY::KEY_O));
        m.insert("P", (Keycode::P, EV_KEY::KEY_P));
        m.insert("Q", (Keycode::Q, EV_KEY::KEY_Q));
        m.insert("R", (Keycode::R, EV_KEY::KEY_R));
        m.insert("S", (Keycode::S, EV_KEY::KEY_S));
        m.insert("T", (Keycode::T, EV_KEY::KEY_T));
        m.insert("U", (Keycode::U, EV_KEY::KEY_U));
        m.insert("V", (Keycode::V, EV_KEY::KEY_V));
        m.insert("W", (Keycode::W, EV_KEY::KEY_W));
        m.insert("X", (Keycode::X, EV_KEY::KEY_X));
        m.insert("Y", (Keycode::Y, EV_KEY::KEY_Y));
        m.insert("Z", (Keycode::Z, EV_KEY::KEY_Z));
        m.insert("æ", (Keycode::F16, EV_KEY::KEY_APOSTROPHE));
        m.insert("ø", (Keycode::F17, EV_KEY::KEY_SEMICOLON));
        m.insert("å", (Keycode::F18, EV_KEY::KEY_LEFTBRACE));
        
        // Numpad
        m.insert("KP0", (Keycode::Numpad0, EV_KEY::KEY_KP0));
        m.insert("KP1", (Keycode::Numpad1, EV_KEY::KEY_KP1));
        m.insert("KP2", (Keycode::Numpad2, EV_KEY::KEY_KP2));
        m.insert("KP3", (Keycode::Numpad3, EV_KEY::KEY_KP3));
        m.insert("KP4", (Keycode::Numpad4, EV_KEY::KEY_KP4));
        m.insert("KP5", (Keycode::Numpad5, EV_KEY::KEY_KP5));
        m.insert("KP6", (Keycode::Numpad6, EV_KEY::KEY_KP6));
        m.insert("KP7", (Keycode::Numpad7, EV_KEY::KEY_KP7));
        m.insert("KP8", (Keycode::Numpad8, EV_KEY::KEY_KP8));
        m.insert("KP9", (Keycode::Numpad9, EV_KEY::KEY_KP9));

        // Add function keys
        m.insert("F1", (Keycode::F1, EV_KEY::KEY_F1));
        m.insert("F2", (Keycode::F2, EV_KEY::KEY_F2));
        m.insert("F3", (Keycode::F3, EV_KEY::KEY_F3));
        m.insert("F4", (Keycode::F4, EV_KEY::KEY_F4));
        m.insert("F5", (Keycode::F5, EV_KEY::KEY_F5));
        m.insert("F6", (Keycode::F6, EV_KEY::KEY_F6));
        m.insert("F7", (Keycode::F7, EV_KEY::KEY_F7));
        m.insert("F8", (Keycode::F8, EV_KEY::KEY_F8));
        m.insert("F9", (Keycode::F9, EV_KEY::KEY_F9));
        m.insert("F10", (Keycode::F10, EV_KEY::KEY_F10));
        m.insert("F11", (Keycode::F11, EV_KEY::KEY_F11));
        m.insert("F12", (Keycode::F12, EV_KEY::KEY_F12));

        // Numbers
        m.insert("1", (Keycode::Key1, EV_KEY::KEY_1));
        m.insert("2", (Keycode::Key2, EV_KEY::KEY_2));
        m.insert("3", (Keycode::Key3, EV_KEY::KEY_3));
        m.insert("4", (Keycode::Key4, EV_KEY::KEY_4));
        m.insert("5", (Keycode::Key5, EV_KEY::KEY_5));
        m.insert("6", (Keycode::Key6, EV_KEY::KEY_6));
        m.insert("7", (Keycode::Key7, EV_KEY::KEY_7));
        m.insert("8", (Keycode::Key8, EV_KEY::KEY_8));
        m.insert("9", (Keycode::Key9, EV_KEY::KEY_9));
        m.insert("0", (Keycode::Key0, EV_KEY::KEY_0));

        // Modifiers
        m.insert("Control", (Keycode::LControl, EV_KEY::KEY_LEFTCTRL));
        m.insert("Shift", (Keycode::LShift, EV_KEY::KEY_LEFTSHIFT));
        m.insert("Alt", (Keycode::LAlt, EV_KEY::KEY_LEFTALT));
        m.insert("AltGraph", (Keycode::RAlt, EV_KEY::KEY_RIGHTALT));        
        m.insert("Super", (Keycode::LMeta, EV_KEY::KEY_LEFTMETA));

        // Add special keys
        m.insert("Space", (Keycode::Space, EV_KEY::KEY_SPACE));
        m.insert("Backspace", (Keycode::Backspace, EV_KEY::KEY_BACKSPACE));
        m.insert("Tab", (Keycode::Tab, EV_KEY::KEY_TAB));
        m.insert("Enter", (Keycode::Enter, EV_KEY::KEY_ENTER));
        m.insert("Escape", (Keycode::Escape, EV_KEY::KEY_ESC));
        m.insert(",", (Keycode::Comma, EV_KEY::KEY_COMMA));
        m.insert(".", (Keycode::Dot, EV_KEY::KEY_DOT));
        m.insert("/", (Keycode::Minus, EV_KEY::KEY_SLASH));
        m.insert(";", (Keycode::Semicolon, EV_KEY::KEY_SEMICOLON));
        m.insert("'", (Keycode::Apostrophe, EV_KEY::KEY_APOSTROPHE));
        m.insert("[", (Keycode::LeftBracket, EV_KEY::KEY_LEFTBRACE));
        m.insert("]", (Keycode::RightBracket, EV_KEY::KEY_RIGHTBRACE));
        m.insert("<", (Keycode::F19 , EV_KEY::KEY_102ND));
        m.insert("-", (Keycode::Minus, EV_KEY::KEY_MINUS));
        m.insert("=", (Keycode::Equal, EV_KEY::KEY_EQUAL));
        m.insert("\\\\", (Keycode::BackSlash, EV_KEY::KEY_BACKSLASH));
        m.insert("`", (Keycode::Grave, EV_KEY::KEY_GRAVE));
        m.insert("CapsLock", (Keycode::CapsLock, EV_KEY::KEY_CAPSLOCK));
        m.insert("Insert", (Keycode::Insert, EV_KEY::KEY_INSERT));
        m.insert("Home", (Keycode::Home, EV_KEY::KEY_HOME));
        m.insert("PageUp", (Keycode::PageUp, EV_KEY::KEY_PAGEUP));
        m.insert("Delete", (Keycode::Delete, EV_KEY::KEY_DELETE));
        m.insert("End", (Keycode::End, EV_KEY::KEY_END));
        m.insert("PageDown", (Keycode::PageDown, EV_KEY::KEY_PAGEDOWN));
        m.insert("ArrowRight", (Keycode::Right, EV_KEY::KEY_RIGHT));
        m.insert("ArrowLeft", (Keycode::Left, EV_KEY::KEY_LEFT));
        m.insert("ArrowDown", (Keycode::Down, EV_KEY::KEY_DOWN));
        m.insert("ArrowUp", (Keycode::Up, EV_KEY::KEY_UP));
        
        m
    };
}

pub fn normalize_key(raw: &str) -> String {
    let mut key = raw.trim().to_string();
    
    // Remove nested wrappers using regex.
    let re_char = Regex::new(r#"^Character\("?(?P<inner>.+?)"?\)$"#).unwrap();
    if let Some(caps) = re_char.captures(&key) {
        key = caps["inner"].to_string();
    }
    
    let re_named = Regex::new(r#"^Named\("?(?P<inner>.+?)"?\)$"#).unwrap();
    if let Some(caps) = re_named.captures(&key) {
        key = caps["inner"].to_string();
    }
    
    // Strip KEY_ prefixes.
    if key.starts_with("KEY_KEY_") {
        key = key.trim_start_matches("KEY_KEY_").to_string();
    } else if key.starts_with("KEY_") {
        key = key.trim_start_matches("KEY_").to_string();
    }
    
    key = key.trim().trim_matches('"').trim_matches('\'').to_string();
    if key.len() == 1 {
        key = key.to_uppercase();
    }
    key
}

pub fn keycode_to_evkey(keycode: Keycode) -> Option<EV_KEY> {
    for (_, (k, ev)) in KEY_MAPPINGS.iter() {
        if k == &keycode {
            return Some(*ev);
        }
    }
    log::warn!("No matching EV_KEY found for keycode: {:?}", keycode);
    None
}

pub fn key_to_device_keycode(key: &str) -> Option<Keycode> {
    // First try direct mapping
    if let Some((keycode, _)) = KEY_MAPPINGS.get(key) {
        return Some(keycode.clone());
    }

    // Then try with KEY_ prefix
    if let Some((keycode, _)) = KEY_MAPPINGS.get(key.trim_start_matches("KEY_")) {
        return Some(keycode.clone());
    }

    // Handle single letter keys
    if key.len() == 1 {
        let upper_key = key.to_uppercase();
        if let Some((keycode, _)) = KEY_MAPPINGS.get(upper_key.as_str()) {
            return Some(keycode.clone());
        }
    }

    log::warn!("Failed to convert key to device keycode: {}", key);
    None
}

/// Converts a raw key string (from JSON) into a device Keycode.
pub fn raw_key_to_device_keycode(raw: &String) -> Option<Keycode> {
    let key = normalize_key(raw);
    key_to_device_keycode(key.as_str())
}

// New functions added for global hotkey validation (moved from hotkey.rs)
fn fallback_hotkey() -> Keycode {
    log::warn!("Using fallback hotkey: F8");
    Keycode::F8
}

pub fn validate_hotkey(app_data: &crate::config::AppData) -> Keycode {
    log::debug!("Validating hotkey configuration: {}", app_data.global_keybind.key);
    let default_hotkey = crate::config::GlobalHotkey::default();
    
    let key_str = app_data.global_keybind.key.trim();
    if key_str.is_empty() {
        log::warn!("Empty or whitespace-only hotkey configured, falling back to default");
        return key_to_device_keycode(normalize_key(default_hotkey.key.as_str()).as_str())
            .unwrap_or_else(fallback_hotkey);
    }
    
    let normalized = normalize_key(key_str);
    if normalized.is_empty() {
        log::warn!("Hotkey normalization resulted in empty string, falling back to default");
        return fallback_hotkey();
    }
    
    if let Some(keycode) = key_to_device_keycode(normalized.as_str()) {
        log::debug!("Successfully mapped '{}' to {:?}", normalized, keycode);
        return keycode;
    }
    
    log::warn!("Hotkey validation failed for '{}', falling back to default", app_data.global_keybind.key);
    fallback_hotkey()
}

pub fn is_modifier_evcode(ec: &evdev_rs::enums::EventCode) -> bool {
    use evdev_rs::enums::EV_KEY;
    match ec {
        evdev_rs::enums::EventCode::EV_KEY(
            EV_KEY::KEY_LEFTSHIFT
            | EV_KEY::KEY_RIGHTSHIFT
            | EV_KEY::KEY_LEFTCTRL
            | EV_KEY::KEY_RIGHTCTRL
            | EV_KEY::KEY_LEFTALT
            | EV_KEY::KEY_RIGHTALT
            | EV_KEY::KEY_LEFTMETA
            | EV_KEY::KEY_RIGHTMETA
            | EV_KEY::KEY_CAPSLOCK
            | EV_KEY::KEY_NUMLOCK
            | EV_KEY::KEY_SCROLLLOCK,
        ) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_key;

    #[test]
    fn test_normalize_key_examples() {
        // Example 1: Input: Character("a") => Expected Output: A
        assert_eq!(normalize_key("Character(\"a\")"), "A");
        // Example 2: Input: "a" with stray whitespace and quote => Expected Output: A
        assert_eq!(normalize_key(" \"a\" "), "A");
        // Example 3: Input: Named(F8) => Expected Output: F8
        assert_eq!(normalize_key("Named(F8)"), "F8");
        // Example 4: Input: KEY_A => Expected Output: A
        assert_eq!(normalize_key("KEY_A"), "A");
        // Example 5: Input: Character("Named(X)") => Expected Output: X
        assert_eq!(normalize_key("Character(\"Named(X)\")"), "X");
    }
    
    // New tests for simulation keys
    #[test]
    fn test_normalize_key_altgraph() {
        // "Character(\"Named(AltGraph)\")" should become "AltGraph"
        assert_eq!(normalize_key("Character(\"Named(AltGraph)\")"), "AltGraph");
    }
    
    #[test]
    fn test_normalize_key_number() {
        // "Character(\"2\")" should become "2"
        assert_eq!(normalize_key("Character(\"2\")"), "2");
    }
}
