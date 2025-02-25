use cosmic::iced::mouse::ScrollDelta;

pub fn handle_scroll_value(current: u64, delta: ScrollDelta, min: f32, max: f32) -> u64 {
    let scroll_amount = calculate_scroll_delta(delta, current as f32);
    (current as f32 - scroll_amount)
        .clamp(min, max) as u64
}

pub fn calculate_scroll_delta(delta: ScrollDelta, _current_value: f32) -> f32 {
    match delta {
        ScrollDelta::Lines { y, .. } => y * 10.0,
        ScrollDelta::Pixels { y, .. } => y / 2.0,
    }
}