use cosmic::{
    cosmic_config::{Config as CosmicConfig, CosmicConfigEntry, Error as CosmicError},
    iced::keyboard::Key,
};
use serde::{Serialize, Deserialize};
use std::str::FromStr;

use crate::utils::{KeyWrapper, serialize_keys, deserialize_keys};

const KEY_BEHAVIOR_MODES: [(&str, KeyBehaviorMode); 2] = [
    ("Click", KeyBehaviorMode::Click),
    ("Hold", KeyBehaviorMode::Hold),
];

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyBehaviorMode {
    Hold,
    Click,
}

impl std::fmt::Display for KeyBehaviorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", KEY_BEHAVIOR_MODES.iter()
            .find(|(_, mode)| mode == self)
            .map(|(name, _)| *name)
            .unwrap_or("Unknown"))
    }
}

impl FromStr for KeyBehaviorMode {
    type Err = ();

    fn from_str(input: &str) -> Result<KeyBehaviorMode, Self::Err> {
        KEY_BEHAVIOR_MODES.iter()
            .find(|(name, _)| *name == input)
            .map(|(_, mode)| *mode)
            .ok_or(())
    }
}

impl Default for KeyBehaviorMode {
    fn default() -> Self {
        KeyBehaviorMode::Click
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub captured_keys: Vec<KeyWrapper>,
    pub current_key_bind: Vec<KeyWrapper>,
    pub interval_ms: u64,
    pub modifier_mode: KeyBehaviorMode,
}

impl CosmicConfigEntry for Config {
    const VERSION: u64 = 1;

    fn write_entry(&self, _config: &CosmicConfig) -> std::result::Result<(), CosmicError> {
        serde_json::to_string(self)
            .map_err(|e| CosmicError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("JSON error: {}", e))))?;
        Ok(())
    }

    fn get_entry(_config: &CosmicConfig) -> std::result::Result<Self, (Vec<CosmicError>, Self)> {
        // Try to read from config, fallback to default if fails
        let default = Self::default();
        Ok(default)
    }

    fn update_keys<T: AsRef<str>>(&mut self, _: &CosmicConfig, _: &[T]) -> (Vec<CosmicError>, Vec<&'static str>) {
        (vec![], vec![])
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct HotkeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub super_key: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalHotkey {
    pub key: String,
    #[serde(flatten)]
    pub modifiers: HotkeyModifiers,
}

impl Default for GlobalHotkey {
    fn default() -> Self {
        Self {
            key: "F8".to_string(),
            modifiers: HotkeyModifiers::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TempHotkeyState {
    pub key: Option<String>,
    pub modifiers: HotkeyModifiers,
}

impl Default for TempHotkeyState {
    fn default() -> Self {
        Self {
            key: None,
            modifiers: HotkeyModifiers::default(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct AppData {
    #[serde(skip)]
    pub captured_keys: Vec<Key>,
    #[serde(serialize_with = "serialize_keys", deserialize_with = "deserialize_keys")]
    pub selected_keys: Vec<Key>,
    #[serde(default)]
    pub global_keybind: GlobalHotkey,
    pub interval_ms: u64,
    pub modifier_mode: KeyBehaviorMode,
    #[serde(skip)]
    pub capturing_global_hotkey: bool,
    #[serde(skip)]
    pub captured_global_hotkey: Option<Key>,
    #[serde(skip)]
    pub temp_hotkey: TempHotkeyState,
}