use cosmic::iced::mouse::ScrollDelta;
use crate::constants::{MIN_INTERVAL_MS, MAX_INTERVAL_MS};

pub fn calculate_scroll_delta(delta: ScrollDelta, _current_value: f32) -> f32 {
    match delta {
        ScrollDelta::Lines { y, .. } => y * 10.0,
        ScrollDelta::Pixels { y, .. } => y / 2.0,
    }
}

pub fn handle_scroll_value(current: u64, delta: ScrollDelta, _min: f32, _max: f32) -> u64 {
    let scroll_amount = calculate_scroll_delta(delta, current as f32);
    (current as f32 - scroll_amount).clamp(MIN_INTERVAL_MS as f32, MAX_INTERVAL_MS as f32) as u64
}
