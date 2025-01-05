mod macros;
mod components;
mod view;

pub use view::View;

use cosmic::{
    app::Settings,
    iced::{keyboard::Key, Limits},
};

pub fn default_window_settings() -> Settings {
    Settings::default()
        .size((650.0, 300.0).into())
        .size_limits(
            Limits::NONE
                .min_width(400.0)
                .min_height(200.0),
        )
}

pub fn format_key_for_display(key: &Key) -> String {
    let formatted = match key {
        Key::Character(s) => {
            match s.as_str() {
                "KEY_BTN_LEFT" => "Left Click".to_string(),
                "KEY_BTN_MIDDLE" => "Middle Click".to_string(),
                "KEY_BTN_RIGHT" => "Right Click".to_string(),
                _ => s.trim_start_matches("KEY_")
                     .trim_start_matches("Character(\"")
                     .trim_end_matches("\")")
                     .trim_start_matches("Named(")
                     .trim_end_matches(")")
                     .to_string()
            }
        },
        _ => format!("{:?}", key),
    };
    
    log::trace!("Formatted key '{:?}' as '{}'", key, formatted);
    formatted
}
