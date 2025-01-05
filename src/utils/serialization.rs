use cosmic::iced::{keyboard::Key, core::SmolStr};
use serde::{Serialize, Serializer, Deserialize, Deserializer};

pub fn serialize_keys<S>(keys: &Vec<Key>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let key_strings: Vec<String> = keys.iter()
        .map(|key| match key {
            Key::Character(s) => s.to_string(),
            _ => format!("{:?}", key),
        })
        .collect();
    key_strings.serialize(serializer)
}

pub fn deserialize_keys<'de, D>(deserializer: D) -> Result<Vec<Key>, D::Error>
where
    D: Deserializer<'de>,
{
    let key_strings: Vec<String> = Vec::deserialize(deserializer)?;
    Ok(key_strings
        .into_iter()
        .map(|s| Key::Character(SmolStr::from(s)))
        .collect())
}
