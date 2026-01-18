use gpui::{IntoElement, div, prelude::*, px, rgb};

pub fn reload_element() -> impl IntoElement {
    div()
        .flex()
        .bg(rgb(0x202020))
        .rounded(px(5.0))
        .size(px(30.0))
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(rgb(0xefefef))
        .child("↩︎")
}
