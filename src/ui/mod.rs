mod components;
mod view;

pub use view::View;

use cosmic::{
    app::Settings,
    iced::Limits,
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

pub fn format_raw_key_for_display(key: &String) -> String {
    key.clone()
}
