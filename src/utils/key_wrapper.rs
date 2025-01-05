use std::hash::{Hash, Hasher};
use cosmic::iced::{keyboard::{Modifiers}};
use device_query::Keycode;
use serde::{Serialize, Deserialize};
use super::key_utils::KEY_MAPPINGS;

#[derive(Debug, Clone)]
pub struct KeyWrapper {
    pub keycode: Keycode,
    pub modifiers: Modifiers,
}

impl PartialEq for KeyWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.keycode == other.keycode && self.modifiers == other.modifiers
    }
}

impl Eq for KeyWrapper {}

impl Hash for KeyWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(&self.keycode).hash(state);
        self.modifiers.hash(state);
    }
}

impl From<Keycode> for KeyWrapper {
    fn from(keycode: Keycode) -> Self {
        KeyWrapper {
            keycode,
            modifiers: Modifiers::default(),
        }
    }
}

impl From<&Keycode> for KeyWrapper {
    fn from(keycode: &Keycode) -> Self {
        KeyWrapper {
            keycode: keycode.clone(),
            modifiers: Modifiers::default(),
        }
    }
}

impl From<KeyWrapper> for Keycode {
    fn from(wrapper: KeyWrapper) -> Self {
        wrapper.keycode
    }
}

impl Serialize for KeyWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        format!("{:?}", self.keycode).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for KeyWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let key_str = String::deserialize(deserializer)?;
        let key_str = key_str.trim_start_matches("KEY_");
        
        let keycode = KEY_MAPPINGS.get(key_str)
            .map(|(k, _)| k.clone())
            .ok_or_else(|| serde::de::Error::custom(format!("Invalid Keycode: {}", key_str)))?;
        
        Ok(KeyWrapper {
            keycode,
            modifiers: Modifiers::default(),
        })
    }
}
