use cosmic::{
    iced::{Length, Element},
    widget::{self, button, text, Column, Container, Row, Space, text::Text},
    Theme,
};
use crate::{
    app::Message,
    config::{AppData, KeyBehaviorMode},
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

        let footer = {
            let mut row = Row::new().spacing(5);

            if self.is_capturing {
                // Show OK/Cancel buttons for key capture in place of Start/Stop
                row = row
                    .push(button::text("OK").on_press(Message::FinalizeKeys))
                    .push(button::text("Cancel").on_press(Message::CancelCapture))
                    .push(Space::with_width(Length::Fill));
            } else {
                // Show Start/Stop button when not capturing
                row = row
                    .push(components::build_start_button(self.is_running))
                    .push(Space::with_width(Length::Fill));
            }

            if self.is_capturing_hotkey {
                let hotkey_text = components::format_hotkey_text(
                    self.app_data_guard.temp_hotkey.modifiers.ctrl,
                    self.app_data_guard.temp_hotkey.modifiers.alt,
                    self.app_data_guard.temp_hotkey.modifiers.shift,
                    self.app_data_guard.temp_hotkey.modifiers.super_key,
                    self.app_data_guard.temp_hotkey.key.as_deref(),
                );
                row = row
                    .push(Text::new(format!("New Global Hotkey: {}", hotkey_text)))
                    .push(button::text("OK").on_press(Message::FinalizeGlobalHotkey))
                    .push(button::text("Cancel").on_press(Message::CancelGlobalHotkey));
            } else if !self.is_capturing {
                row = row.push(
                    button::text(format!("Global Hotkey: {}", components::format_hotkey_text(
                        self.app_data_guard.global_keybind.modifiers.ctrl,
                        self.app_data_guard.global_keybind.modifiers.alt,
                        self.app_data_guard.global_keybind.modifiers.shift,
                        self.app_data_guard.global_keybind.modifiers.super_key,
                        Some(&self.app_data_guard.global_keybind.key)
                    )))
                        .on_press(Message::CaptureGlobalHotkey)
                        .class(cosmic::theme::Button::Text)
                );
            }

            row
        };

        Container::new(
            Column::new()
                .push(
                    Row::new()
                        .push(
                            Container::new(left_column)
                                .width(Length::FillPortion(1))
                        )
                        .push(widget::vertical_space())
                        .push(
                            Container::new(right_column)
                                .width(Length::FillPortion(1))
                        )
                        .spacing(20)
                )
                .push(footer)
                .padding(10)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn build_left_column(&self) -> widget::Column<'a, Message> {
        let mut column = widget::column().spacing(20);

        column = column.push(
            button::text(format!(
                "Capture Keys{}",
                if self.is_capturing { " (Active)" } else { "" }
            ))
                .on_press(Message::CaptureKeys)
                .class(cosmic::theme::Button::Text)
        );

        // Show either captured keys during capture, or selected keys when not capturing
        if self.is_capturing {
            let captured_keys_text = self.app_data_guard
                .captured_keys
                .iter()
                .map(super::format_raw_key_for_display)
                .collect::<Vec<_>>()
                .join(", ");

            log::debug!("Currently captured keys: {}", captured_keys_text);

            column = column
                .push(text::body(format!("Captured Keys: {}", captured_keys_text)))
                .push(components::build_mouse_buttons());
        } else {
            column = column
                .push(components::build_selected_keys_text(&self.app_data_guard.selected_keys));
        }

        column
    }

    fn build_right_column(&self) -> widget::Column<'a, Message> {
        let mut column = widget::column().spacing(20);

        column = column.push(
            widget::row()
                .push(text::body("Key Behavior: "))
                .push(components::build_key_behavior_dropdown(self.app_data_guard.key_behavior)),
        );
        if self.app_data_guard.key_behavior == KeyBehaviorMode::Click {
            column = column.push(
                widget::row()
                    .push(text::body("Modifier Behavior: "))
                    .push(components::build_modifier_behavior_dropdown(self.app_data_guard.modifier_behavior)),
            );
            column = column.push(
                widget::row()
                    .push(components::interval_controls(self.interval, &self.app_data_guard)),
            );
        }

        column
    }
}
