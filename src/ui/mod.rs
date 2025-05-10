mod components;
mod view;

pub use view::View;

use cosmic::{
    app::Settings,
    iced::Limits,
};

// Window size constants
pub const WINDOW_SIZE_WITH_PANEL: (f32, f32) = (650.0, 400.0); 
pub const WINDOW_SIZE_WITHOUT_PANEL: (f32, f32) = (400.0, 300.0);

pub fn default_window_settings() -> Settings {
    Settings::default()
        .size(WINDOW_SIZE_WITH_PANEL.into())
        .size_limits(
            Limits::NONE
                .min_width(WINDOW_SIZE_WITH_PANEL.0)
                .min_height(WINDOW_SIZE_WITH_PANEL.1)
                .max_width(WINDOW_SIZE_WITH_PANEL.0)
                .max_height(WINDOW_SIZE_WITH_PANEL.1),
        )
}

pub fn format_raw_key_for_display(key: &String) -> String {
    key.clone()
}
