use std::fs;
use crate::config::AppData;
use crate::error::Result;
use crate::utils::key_utils::normalize_key;

pub fn save_app_data(app_data: &mut AppData) -> Result<()> {
    // Normalize selected_keys so that JSON and UI show cleaned keys.
    app_data.selected_keys = app_data.selected_keys
        .iter()
        .map(|s| normalize_key(s))
        .collect();
    if app_data.global_keybind.key.is_empty() {
        app_data.global_keybind.key = "Named(F8)".to_string();
    }
    let json = serde_json::to_string(app_data)?;
    fs::write("app_data.json", json)?;
    log::info!("Config saved successfully");
    Ok(())
}