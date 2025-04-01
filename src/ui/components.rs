use cosmic::{
    iced::Length,
    widget::{button, Column, Dropdown, MouseArea, Row, Slider, TextInput, Text},
    Element,
};
use crate::{
    app::{Message, KeyEvent},
    config::{AppData, KeyBehaviorMode, ModifierBehaviorMode, HoldBehaviorMode},
    utils::handle_scroll_value,
    constants::{MIN_INTERVAL_MS, MAX_INTERVAL_MS},
};

pub fn interval_controls(interval: f64, app_data: &AppData) -> Column<'static, Message> {
    let interval_value = format!("{}", app_data.interval_ms);
    let current_interval = app_data.interval_ms;
    
    log::debug!("Building interval controls with value: {}", interval_value);

    let interval_input = MouseArea::new(
        TextInput::new("", interval_value.clone())
            .on_input(Message::UpdateInterval)
            .on_submit(|value| Message::UpdateInterval(value))
            .padding(5)
            .width(Length::Fixed(60.0))
            .size(16)
    )
    .on_scroll(move |delta| {
        Message::SetIntervalAndSave(handle_scroll_value(
            current_interval,
            delta,
            MIN_INTERVAL_MS as f32,
            MAX_INTERVAL_MS as f32
        ))
    });

    let input_row = Row::new()
        .push(Text::new("Interval (ms):").width(Length::Shrink))
        .push(interval_input)
        .spacing(5);

    let interval_slider = MouseArea::new(
        Slider::new(
            MIN_INTERVAL_MS as f64..=MAX_INTERVAL_MS as f64, 
            interval, 
            |value| Message::SetInterval(value as u64)
        )
        .on_release(Message::SetIntervalAndSave(interval as u64))
    )
    .on_scroll(move |delta| {
        Message::SetIntervalAndSave(handle_scroll_value(
            current_interval,
            delta,
            MIN_INTERVAL_MS as f32,
            MAX_INTERVAL_MS as f32
        ))
    });

    Column::new()
        .push(input_row)
        .push(interval_slider)
        .spacing(5)
}

pub fn build_mouse_buttons() -> impl Into<Element<'static, Message>> {
    Row::new()
        .spacing(5)
        .push(
            button::text("Left Click")
                .on_press(Message::AddKey(KeyEvent::mouse_left()))
                .class(cosmic::theme::Button::Text)
        )
        .push(
            button::text("Middle Click")
                .on_press(Message::AddKey(KeyEvent::mouse_middle()))
                .class(cosmic::theme::Button::Text)
        )
        .push(
            button::text("Right Click")
                .on_press(Message::AddKey(KeyEvent::mouse_right()))
                .class(cosmic::theme::Button::Text)
        )
}

fn build_generic_dropdown<T, F>(
    choices: &'static [&'static str],
    current_mode: T,
    map_fn: F,
) -> Dropdown<'static, &'static str, Message, Message>
where
    T: ToString,
    F: Fn(usize) -> Message + 'static + Send + Sync,
{
    let selected_index = choices.iter().position(|&mode| mode == current_mode.to_string());
    Dropdown::new(choices, selected_index, map_fn)
}

pub fn build_key_behavior_dropdown(current_mode: KeyBehaviorMode) -> Dropdown<'static, &'static str, Message, Message> {
    const KEY_BEHAVIORS: [&str; 2] = ["Click", "Hold"];
    build_generic_dropdown(
        &KEY_BEHAVIORS,
        current_mode,
        |index| match index {
            0 => Message::UpdateKeyBehaviorMode(KeyBehaviorMode::Click),
            1 => Message::UpdateKeyBehaviorMode(KeyBehaviorMode::Hold),
            _ => Message::Noop,
        },
    )
}

pub fn build_hold_behavior_dropdown(current_mode: HoldBehaviorMode) -> Dropdown<'static, &'static str, Message, Message> {
    const HOLD_BEHAVIOR_MODES: [&str; 2] = ["Continuous", "Cycle"];
    build_generic_dropdown(
        &HOLD_BEHAVIOR_MODES,
        current_mode,
        |index| match index {
            0 => Message::UpdateHoldBehaviorMode(HoldBehaviorMode::Continuous),
            1 => Message::UpdateHoldBehaviorMode(HoldBehaviorMode::Cycle),
            _ => Message::Noop,
        },
    )
}

pub fn build_modifier_behavior_dropdown(current_mode: ModifierBehaviorMode) -> Dropdown<'static, &'static str, Message, Message> {
    const MODIFIER_BEHAVIOR_MODES: [&str; 2] = ["Click", "Hold"];
    build_generic_dropdown(
        &MODIFIER_BEHAVIOR_MODES,
        current_mode,
        |index| match index {
            0 => Message::UpdateModifierBehaviorMode(ModifierBehaviorMode::Click),
            1 => Message::UpdateModifierBehaviorMode(ModifierBehaviorMode::Hold),
            _ => Message::Noop,
        },
    )
}

pub fn format_hotkey_text(
    ctrl: bool, 
    alt: bool, 
    shift: bool, 
    super_key: bool, 
    key: Option<&str>,
) -> String {
    let mut parts = Vec::new();
    if ctrl { parts.push("Ctrl"); }
    if alt { parts.push("Alt"); }
    if shift { parts.push("Shift"); }
    if super_key { parts.push("Super"); }
    if let Some(k) = key { parts.push(k); }
    parts.join("+")
}

pub fn build_start_button(is_running: bool) -> impl Into<Element<'static, Message>> {
    button::text(if is_running { "Stop" } else { "Start" })
        .on_press(Message::ToggleRunning)
        .class(cosmic::theme::Button::Text)
}

pub fn build_selected_keys_text(keys: &[String]) -> Element<'static, Message> {
    let selected_count = keys.len();
    Column::new()
        .push(Text::new(format!("Selected Keys ({}):", selected_count)))
        .push(
            Text::new(keys.join(", "))
                .width(Length::Fill)
                .wrapping(cosmic::iced_core::text::Wrapping::WordOrGlyph)
        )
        .spacing(5)
        .into()
}
