use cosmic::{
    iced::{Length, Element},
    widget::{self, button, text},
    Theme,
};
use crate::{
    app::Message,
    config::AppData,
    ui::components,
};

pub struct View<'a> {
    is_running: bool,
    interval: f64,
    app_data_guard: std::sync::MutexGuard<'a, AppData>,
    is_capturing: bool,
    is_capturing_hotkey: bool,
}

impl<'a> View<'a> {
    pub fn new(
        is_running: bool,
        interval: f64,
        app_data_guard: std::sync::MutexGuard<'a, AppData>,
        is_capturing: bool,
        is_capturing_hotkey: bool,
    ) -> Self {
        log::debug!("Creating new view with running: {}, capturing: {}, capturing_hotkey: {}", 
            is_running, is_capturing, is_capturing_hotkey);
        Self {
            is_running,
            interval,
            app_data_guard,
            is_capturing,
            is_capturing_hotkey,
        }
    }

    pub fn build(&self) -> Element<'a, Message, Theme> {
        let left_column = self.build_left_column();
        let right_column = self.build_right_column();

        widget::row()
            .push(
                widget::container(left_column)
                    .padding(10)
                    .width(Length::FillPortion(1))
            )
            .push(widget::vertical_space())
            .push(right_column.width(Length::FillPortion(1)))
            .spacing(20)
            .into()
    }

    fn build_left_column(&self) -> widget::Column<'a, Message> {
        let mut column = widget::column().spacing(20);

        column = column.push(
            button::text(format!(
                "Capture Keys{}",
                if self.is_capturing { " (Active)" } else { "" }
            ))
            .on_press(Message::CaptureKeys),
        );

        if self.is_capturing {
            let captured_keys_text = self.app_data_guard
                .captured_keys
                .iter()
                .map(super::format_key_for_display)
                .collect::<Vec<_>>()
                .join(", ");

            log::debug!("Currently captured keys: {}", captured_keys_text);

            column = column
                .push(text::body(format!("Captured Keys: {}", captured_keys_text)))
                .push(components::build_mouse_buttons())
                .push(components::build_capture_controls());
        }

        column = column
            .push(components::build_selected_keys_text(&self.app_data_guard.selected_keys))
            .push(components::build_start_button(self.is_running));

        column
    }

    fn build_right_column(&self) -> widget::Column<'a, Message> {
        let mut column = widget::column().spacing(20);

        column = column.push(
            widget::row()
                .push(text::body("Key Behavior: "))
                .push(components::build_modifier_dropdown(self.app_data_guard.modifier_mode)),
        );

        if self.app_data_guard.modifier_mode != crate::config::KeyBehaviorMode::Hold {
            column = column.push(
                widget::row()
                    .push(components::interval_controls(self.interval, &self.app_data_guard)),
            );
        }

        let hotkey_text = components::build_hotkey_text(
            self.app_data_guard.global_keybind.modifiers.ctrl,
            self.app_data_guard.global_keybind.modifiers.alt,
            self.app_data_guard.global_keybind.modifiers.shift,
            self.app_data_guard.global_keybind.modifiers.super_key,
            Some(&self.app_data_guard.global_keybind.key)
        );
        column = column.push(
            widget::button::text(format!("Bind Global Hotkey ({})", hotkey_text))
                .on_press(Message::CaptureGlobalHotkey),
        );

        if self.is_capturing_hotkey {
            let pending_text = components::build_hotkey_text(
                self.app_data_guard.temp_hotkey.modifiers.ctrl,
                self.app_data_guard.temp_hotkey.modifiers.alt,
                self.app_data_guard.temp_hotkey.modifiers.shift,
                self.app_data_guard.temp_hotkey.modifiers.super_key,
                self.app_data_guard.temp_hotkey.key.as_deref()
            );
            log::debug!("Pending hotkey configuration: {}", pending_text);
            column = column
                .push(widget::text::body(format!("New Global Hotkey: {}", pending_text)))
                .push(components::build_hotkey_controls());
        }

        column
    }
}
