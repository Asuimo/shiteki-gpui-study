use gpui::{div, prelude::*, rgb};

pub fn quick_add_element(cx: &mut Context) {
    div()
        .flex()
        .flex_row()
        .bg(rgb(0xfafafa))
        .text_color(rgb(0x333333))
        .child("Quick Add")
}
