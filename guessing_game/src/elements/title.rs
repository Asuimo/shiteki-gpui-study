use gpui::{div, prelude::*, px, rgb};

pub fn title_element() -> impl IntoElement {
    div()
        .text_size(px(30.0))
        .bg(rgb(0x1e1e1e))
        .text_color(rgb(0xffffff))
        .child(format!("Guessing Game"))
}
