use serde::{Serialize, Deserialize};
use std::str::FromStr;

const KEY_BEHAVIOR_MODES: [(&str, KeyBehaviorMode); 2] = [
    ("Click", KeyBehaviorMode::Click),
    ("Hold", KeyBehaviorMode::Hold),
];

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyBehaviorMode {
    Hold,
    Click,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HoldBehaviorMode {
    Continuous,
    Cycle,
}

impl Default for HoldBehaviorMode {
    fn default() -> Self {
        HoldBehaviorMode::Continuous
    }
}

impl std::fmt::Display for HoldBehaviorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoldBehaviorMode::Continuous => write!(f, "Continuous"),
            HoldBehaviorMode::Cycle => write!(f, "Cycle"),
        }
    }
}

impl FromStr for HoldBehaviorMode {
    type Err = ();

    fn from_str(input: &str) -> Result<HoldBehaviorMode, Self::Err> {
        match input {
            "Continuous" => Ok(HoldBehaviorMode::Continuous),
            "Cycle" => Ok(HoldBehaviorMode::Cycle),
            _ => Err(()),
        }
    }
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModifierBehaviorMode {
    Hold,
    Click,
}

impl std::fmt::Display for ModifierBehaviorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModifierBehaviorMode::Hold => write!(f, "Hold"),
            ModifierBehaviorMode::Click => write!(f, "Click"),
        }
    }
}

impl FromStr for ModifierBehaviorMode {
    type Err = ();

    fn from_str(input: &str) -> Result<ModifierBehaviorMode, Self::Err> {
        match input {
            "Hold" => Ok(ModifierBehaviorMode::Hold),
            "Click" => Ok(ModifierBehaviorMode::Click),
            _ => Err(()),
        }
    }
}

impl Default for ModifierBehaviorMode {
    fn default() -> Self {
        ModifierBehaviorMode::Click
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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppData {
    #[serde(skip)]
    pub captured_keys: Vec<String>,
    #[serde(default)]
    pub selected_keys: Vec<String>,
    #[serde(default)]
    pub global_keybind: GlobalHotkey,
    pub interval_ms: u64,
    pub key_behavior: KeyBehaviorMode,
    pub modifier_behavior: ModifierBehaviorMode,
    #[serde(default)]
    pub hold_behavior: HoldBehaviorMode,
    #[serde(skip)]
    pub capturing_global_hotkey: bool,
    #[serde(skip)]
    pub temp_hotkey: TempHotkeyState,
}