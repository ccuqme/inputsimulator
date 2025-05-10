use cosmic::{
    iced::{Length, Element},
    widget::{button, text, Column, Container, Row, Space},
    Theme,
};
use crate::{
    app::Message,
    config::{AppData, KeyBehaviorMode, HoldBehaviorMode},
    ui::components,
};

pub struct View<'a> {
    is_running: bool,
    interval: f64,
    app_data_guard: std::sync::MutexGuard<'a, AppData>,
    is_capturing: bool,
    is_capturing_hotkey: bool,
    settings_panel_open: bool,
}

impl<'a> View<'a> {
    pub fn new(
        is_running: bool,
        interval: f64,
        app_data_guard: std::sync::MutexGuard<'a, AppData>,
        is_capturing: bool,
        is_capturing_hotkey: bool,
        settings_panel_open: bool,
    ) -> Self {
        log::debug!("Creating new view with running: {}, capturing: {}, capturing_hotkey: {}", 
            is_running, is_capturing, is_capturing_hotkey);
        Self {
            is_running,
            interval,
            app_data_guard,
            is_capturing,
            is_capturing_hotkey,
            settings_panel_open,
        }
    }

    pub fn build(&self) -> Element<'a, Message, Theme> {
        let main_content = self.build_main_content();
        let settings_panel = self.build_settings_panel();
        
        let mut layout = Row::new().spacing(0);
        
        layout = layout.push(
            Container::new(main_content)
                .width(Length::Fill)
                .height(Length::Fill)
        );
        
        if self.settings_panel_open {
            layout = layout.push(
                Container::new(settings_panel)
                    .width(Length::Fixed(250.0))
                    .height(Length::Fill)
            );
        }
        
        Container::new(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
    
    fn build_main_content(&self) -> Column<'a, Message> {
        let mut column = Column::new().spacing(20);
        
        let toggle_icon = if self.settings_panel_open {
            "go-next-symbolic"
        } else {
            "go-previous-symbolic"
        };
        
        let title_row = Row::new()
            .push(text::heading("Input Simulator").size(24))
            .push(Space::with_width(Length::Fill))
            .push(
                button::icon(cosmic::widget::icon::from_name(toggle_icon))
                    .on_press(Message::ToggleSettingsPanel)
                    .tooltip(if self.settings_panel_open { "Hide settings" } else { "Show settings" })
                    .class(cosmic::theme::Button::Text)
            );
        
        column = column.push(title_row);
        
        column = column.push(
            button::text(format!(
                "Capture Keys{}",
                if self.is_capturing { " (Active)" } else { "" }
            ))
                .on_press(Message::CaptureKeys)
                .class(cosmic::theme::Button::Suggested)
        );

        if self.is_capturing {
            let captured_keys_text = self.app_data_guard
                .captured_keys
                .iter()
                .map(super::format_raw_key_for_display)
                .collect::<Vec<_>>()
                .join(", ");

            log::debug!("Currently captured keys: {}", captured_keys_text);

            column = column
                .push(text::body(format!("Captured Keys: {}", captured_keys_text)).size(16))
                .push(components::build_mouse_buttons());
                
            column = column.push(Space::with_height(Length::Fill));
            column = column.push(
                Row::new()
                    .push(
                        button::text("OK")
                            .on_press(Message::FinalizeKeys)
                    )
                    .push(Space::with_width(Length::Fill))
                    .push(
                        button::text("Cancel")
                            .on_press(Message::CancelCapture)
                    )
                    .spacing(10)
            );
        } else {
            column = column
                .push(components::build_selected_keys_text(&self.app_data_guard.selected_keys))
                .push(Space::with_height(Length::Fill));
        }
        if self.is_capturing_hotkey {
            let hotkey_text = components::format_hotkey_text(
                self.app_data_guard.temp_hotkey.modifiers.ctrl,
                self.app_data_guard.temp_hotkey.modifiers.alt,
                self.app_data_guard.temp_hotkey.modifiers.shift,
                self.app_data_guard.temp_hotkey.modifiers.super_key,
                self.app_data_guard.temp_hotkey.key.as_deref(),
            );
            
            column = column
                .push(text::body(format!("New Global Hotkey: {}", hotkey_text)).size(16))
                .push(
                    Row::new()
                        .push(
                            button::text("OK")
                                .on_press(Message::FinalizeGlobalHotkey)
                                .width(Length::Fill)
                        )
                        .push(
                            button::text("Cancel")
                                .on_press(Message::CancelGlobalHotkey)
                                .width(Length::Fill)
                        )
                        .spacing(10)
                );
        }

        if !self.is_capturing && !self.is_capturing_hotkey {
            column = column.push(
                Row::new()
                    .push(Container::new(components::build_start_button(self.is_running)))
                    .push(Space::with_width(Length::Fill))
            );
        }

        column
    }
    
    fn build_settings_panel(&self) -> Column<'a, Message> {
        let mut column = Column::new().spacing(20);
        
        column = column.push(text::heading("Settings").size(20));
        
        column = column.push(
            Column::new()
                .push(text::body("Key Behavior:"))
                .push(components::build_key_behavior_dropdown(self.app_data_guard.key_behavior).width(Length::Fill))
                .spacing(5)
        );
        
        if self.app_data_guard.key_behavior == KeyBehaviorMode::Hold {
            column = column.push(
                Column::new()
                    .push(text::body("Hold Behavior:"))
                    .push(components::build_hold_behavior_dropdown(self.app_data_guard.hold_behavior).width(Length::Fill))
                    .spacing(5)
            );
            if self.app_data_guard.hold_behavior == HoldBehaviorMode::Cycle {
                column = column.push(components::interval_controls(self.interval, &self.app_data_guard));
            }
        } else if self.app_data_guard.key_behavior == KeyBehaviorMode::Click {
            column = column.push(
                Column::new()
                    .push(text::body("Modifier Behavior:"))
                    .push(components::build_modifier_behavior_dropdown(self.app_data_guard.modifier_behavior).width(Length::Fill))
                    .spacing(5)
            );
            column = column.push(components::interval_controls(self.interval, &self.app_data_guard));
        }
        
        column = column.push(Space::with_height(Length::Fill));
        if !self.is_capturing_hotkey {
            column = column.push(
                Row::new()
                    .push(Space::with_width(Length::Fill))
                    .push(
                        button::text(format!("Global Hotkey: {}", components::format_hotkey_text(
                            self.app_data_guard.global_keybind.modifiers.ctrl,
                            self.app_data_guard.global_keybind.modifiers.alt,
                            self.app_data_guard.global_keybind.modifiers.shift,
                            self.app_data_guard.global_keybind.modifiers.super_key,
                            Some(&self.app_data_guard.global_keybind.key)
                        )))
                            .on_press(Message::CaptureGlobalHotkey)
                            .class(cosmic::theme::Button::Text)
                    )
            );
        }
        
        column
    }
}
