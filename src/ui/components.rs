use cosmic::{
    iced::{keyboard::Key, Length},
    widget::{Button, Column, Dropdown, MouseArea, Row, Slider, TextInput, Text},
    Element,
};
use crate::{
    app::{Message, KeyEvent},
    config::{AppData, KeyBehaviorMode},
    utils::handle_scroll_value,
    create_button, create_control_row,
    ui::format_key_for_display,
    constants::{MIN_INTERVAL_MS, MAX_INTERVAL_MS},
};

pub fn interval_controls(interval: f64, app_data: &AppData) -> Column<'static, Message> {
    let interval_value = format!("{}", app_data.interval_ms);
    let current_interval = app_data.interval_ms;
    
    log::debug!("Building interval controls with value: {}", interval_value);

    let interval_input = MouseArea::new(
        TextInput::new("", interval_value.clone())
            .on_input(Message::UpdateInterval)
            .on_submit(Message::UpdateInterval(interval_value))
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

pub fn build_mouse_buttons() -> Row<'static, Message> {
    create_control_row!(
        create_button!("Left Click", Message::AddKey(KeyEvent::mouse_left())),
        create_button!("Middle Click", Message::AddKey(KeyEvent::mouse_middle())),
        create_button!("Right Click", Message::AddKey(KeyEvent::mouse_right()))
    )
}

pub fn build_capture_controls() -> Row<'static, Message> {
    create_control_row!(
        create_button!("OK", Message::FinalizeKeys),
        create_button!("Cancel", Message::CancelCapture)
    )
}

pub fn build_hotkey_controls() -> Row<'static, Message> {
    create_control_row!(
        create_button!("OK", Message::FinalizeGlobalHotkey),
        create_button!("Cancel", Message::CancelGlobalHotkey)
    )
}

pub fn build_modifier_dropdown(current_mode: KeyBehaviorMode) -> Dropdown<'static, &'static str, Message> {
    const MODIFIER_MODES: [&str; 2] = ["Click", "Hold"];
    let selected_index = MODIFIER_MODES
        .iter()
        .position(|&mode| mode == current_mode.to_string());

    Dropdown::new(
        &MODIFIER_MODES,
        selected_index,
        |index| match index {
            0 => Message::UpdateKeyBehaviorMode(KeyBehaviorMode::Click),
            1 => Message::UpdateKeyBehaviorMode(KeyBehaviorMode::Hold),
            _ => Message::Noop,
        },
    )
}

pub fn build_hotkey_text(
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

pub fn build_start_button(is_running: bool) -> Button<'static, Message> {
    Button::new_image(
        Text::new(if is_running { "Stop" } else { "Start" }),
        None
    ).on_press(Message::ToggleRunning)
}

pub fn build_selected_keys_text(keys: &[Key]) -> Element<'static, Message> {
    let formatted_keys = keys.iter()
        .map(format_key_for_display)
        .collect::<Vec<String>>()
        .join(", ");
    
    log::debug!("Displaying selected keys: {}", formatted_keys);
    
    Text::new(format!("Selected Keys: [{}]", formatted_keys)).into()
}
