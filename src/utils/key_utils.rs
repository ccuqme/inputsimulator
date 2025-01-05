use cosmic::iced::keyboard::Key;
use device_query::Keycode;
use evdev_rs::enums::EV_KEY;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub(crate) static ref KEY_MAPPINGS: HashMap<&'static str, (Keycode, evdev_rs::enums::EV_KEY)> = {
        let mut m = HashMap::new();
        // Mouse buttons (Using F13-F15 for device_query compatibility)
        m.insert("BTN_LEFT", (Keycode::F13, evdev_rs::enums::EV_KEY::BTN_LEFT));
        m.insert("BTN_MIDDLE", (Keycode::F14, evdev_rs::enums::EV_KEY::BTN_MIDDLE));
        m.insert("BTN_RIGHT", (Keycode::F15, evdev_rs::enums::EV_KEY::BTN_RIGHT));

        // Letter keys
        m.insert("A", (Keycode::A, evdev_rs::enums::EV_KEY::KEY_A));
        m.insert("B", (Keycode::B, evdev_rs::enums::EV_KEY::KEY_B));
        m.insert("C", (Keycode::C, evdev_rs::enums::EV_KEY::KEY_C));
        m.insert("D", (Keycode::D, evdev_rs::enums::EV_KEY::KEY_D));
        m.insert("E", (Keycode::E, evdev_rs::enums::EV_KEY::KEY_E));
        m.insert("F", (Keycode::F, evdev_rs::enums::EV_KEY::KEY_F));
        m.insert("G", (Keycode::G, evdev_rs::enums::EV_KEY::KEY_G));
        m.insert("H", (Keycode::H, evdev_rs::enums::EV_KEY::KEY_H));
        m.insert("I", (Keycode::I, evdev_rs::enums::EV_KEY::KEY_I));
        m.insert("J", (Keycode::J, evdev_rs::enums::EV_KEY::KEY_J));
        m.insert("K", (Keycode::K, evdev_rs::enums::EV_KEY::KEY_K));
        m.insert("L", (Keycode::L, evdev_rs::enums::EV_KEY::KEY_L));
        m.insert("M", (Keycode::M, evdev_rs::enums::EV_KEY::KEY_M));
        m.insert("N", (Keycode::N, evdev_rs::enums::EV_KEY::KEY_N));
        m.insert("O", (Keycode::O, evdev_rs::enums::EV_KEY::KEY_O));
        m.insert("P", (Keycode::P, evdev_rs::enums::EV_KEY::KEY_P));
        m.insert("Q", (Keycode::Q, evdev_rs::enums::EV_KEY::KEY_Q));
        m.insert("R", (Keycode::R, evdev_rs::enums::EV_KEY::KEY_R));
        m.insert("S", (Keycode::S, evdev_rs::enums::EV_KEY::KEY_S));
        m.insert("T", (Keycode::T, evdev_rs::enums::EV_KEY::KEY_T));
        m.insert("U", (Keycode::U, evdev_rs::enums::EV_KEY::KEY_U));
        m.insert("V", (Keycode::V, evdev_rs::enums::EV_KEY::KEY_V));
        m.insert("W", (Keycode::W, evdev_rs::enums::EV_KEY::KEY_W));
        m.insert("X", (Keycode::X, evdev_rs::enums::EV_KEY::KEY_X));
        m.insert("Z", (Keycode::Z, evdev_rs::enums::EV_KEY::KEY_Z));

        // Numpad
        m.insert("KP0", (Keycode::Numpad0, evdev_rs::enums::EV_KEY::KEY_KP0));
        m.insert("KP1", (Keycode::Numpad1, evdev_rs::enums::EV_KEY::KEY_KP1));
        m.insert("KP2", (Keycode::Numpad2, evdev_rs::enums::EV_KEY::KEY_KP2));
        m.insert("KP3", (Keycode::Numpad3, evdev_rs::enums::EV_KEY::KEY_KP3));
        m.insert("KP4", (Keycode::Numpad4, evdev_rs::enums::EV_KEY::KEY_KP4));
        m.insert("KP5", (Keycode::Numpad5, evdev_rs::enums::EV_KEY::KEY_KP5));
        m.insert("KP6", (Keycode::Numpad6, evdev_rs::enums::EV_KEY::KEY_KP6));
        m.insert("KP7", (Keycode::Numpad7, evdev_rs::enums::EV_KEY::KEY_KP7));
        m.insert("KP8", (Keycode::Numpad8, evdev_rs::enums::EV_KEY::KEY_KP8));
        m.insert("KP9", (Keycode::Numpad9, evdev_rs::enums::EV_KEY::KEY_KP9));

        // Add function keys
        m.insert("F1", (Keycode::F1, evdev_rs::enums::EV_KEY::KEY_F1));
        m.insert("F2", (Keycode::F2, evdev_rs::enums::EV_KEY::KEY_F2));
        m.insert("F3", (Keycode::F3, evdev_rs::enums::EV_KEY::KEY_F3));
        m.insert("F4", (Keycode::F4, evdev_rs::enums::EV_KEY::KEY_F4));
        m.insert("F5", (Keycode::F5, evdev_rs::enums::EV_KEY::KEY_F5));
        m.insert("F6", (Keycode::F6, evdev_rs::enums::EV_KEY::KEY_F6));
        m.insert("F7", (Keycode::F7, evdev_rs::enums::EV_KEY::KEY_F7));
        m.insert("F8", (Keycode::F8, evdev_rs::enums::EV_KEY::KEY_F8));
        m.insert("F9", (Keycode::F9, evdev_rs::enums::EV_KEY::KEY_F9));
        m.insert("F10", (Keycode::F10, evdev_rs::enums::EV_KEY::KEY_F10));
        m.insert("F11", (Keycode::F11, evdev_rs::enums::EV_KEY::KEY_F11));
        m.insert("F12", (Keycode::F12, evdev_rs::enums::EV_KEY::KEY_F12));

        // Numbers
        m.insert("1", (Keycode::Key1, evdev_rs::enums::EV_KEY::KEY_1));
        m.insert("2", (Keycode::Key2, evdev_rs::enums::EV_KEY::KEY_2));
        m.insert("3", (Keycode::Key3, evdev_rs::enums::EV_KEY::KEY_3));
        m.insert("4", (Keycode::Key4, evdev_rs::enums::EV_KEY::KEY_4));
        m.insert("5", (Keycode::Key5, evdev_rs::enums::EV_KEY::KEY_5));
        m.insert("6", (Keycode::Key6, evdev_rs::enums::EV_KEY::KEY_6));
        m.insert("7", (Keycode::Key7, evdev_rs::enums::EV_KEY::KEY_7));
        m.insert("8", (Keycode::Key8, evdev_rs::enums::EV_KEY::KEY_8));
        m.insert("9", (Keycode::Key9, evdev_rs::enums::EV_KEY::KEY_9));
        m.insert("0", (Keycode::Key0, evdev_rs::enums::EV_KEY::KEY_0));

        // Modifiers
        m.insert("Control", (Keycode::LControl, evdev_rs::enums::EV_KEY::KEY_LEFTCTRL));
        m.insert("Shift", (Keycode::LShift, evdev_rs::enums::EV_KEY::KEY_LEFTSHIFT));
        m.insert("Alt", (Keycode::LAlt, evdev_rs::enums::EV_KEY::KEY_LEFTALT));
        m.insert("Meta", (Keycode::LMeta, evdev_rs::enums::EV_KEY::KEY_LEFTMETA));

        // Add special keys
        m.insert("Space", (Keycode::Space, evdev_rs::enums::EV_KEY::KEY_SPACE));
        m.insert("Tab", (Keycode::Tab, evdev_rs::enums::EV_KEY::KEY_TAB));
        m.insert("Return", (Keycode::Enter, evdev_rs::enums::EV_KEY::KEY_ENTER));
        m.insert("Escape", (Keycode::Escape, evdev_rs::enums::EV_KEY::KEY_ESC));

        m
    };
}

pub fn clean_key_string(key: &str) -> String {
    key.trim_start_matches("KEY_KEY_")
       .trim_start_matches("KEY_")
       .trim_start_matches("Named(")
       .trim_end_matches(")")
       .trim_start_matches("Character(\"")
       .trim_end_matches("\")")
       .to_string()
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

pub fn from_cosmic_key(key: Key) -> Option<Keycode> {
    match key {
        Key::Character(s) => {
            let clean_key = clean_key_string(&s);
            
            KEY_MAPPINGS.get(clean_key.as_str())
                .map(|(keycode, _)| keycode.clone())
                .or_else(|| {
                    KEY_MAPPINGS.get(clean_key.to_uppercase().as_str())
                        .map(|(keycode, _)| keycode.clone())
                })
                .or_else(|| {
                    log::warn!("Failed to map cosmic key: {}", clean_key);
                    None
                })
        },
        _ => {
            log::warn!("Unsupported key type: {:?}", key);
            None
        }
    }
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
