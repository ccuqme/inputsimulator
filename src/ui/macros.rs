#[macro_export]
macro_rules! create_button {
    ($label:expr, $message:expr) => {
        Button::new_image(Text::new($label), None)
            .on_press($message)
    };
}

#[macro_export]
macro_rules! create_control_row {
    ($($button:expr),+ $(,)?) => {
        Row::new()
            $(.push($button))+
            .spacing(10)
    };
}
