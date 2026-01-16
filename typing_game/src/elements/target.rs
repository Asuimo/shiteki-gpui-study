use gpui::{IntoElement, div, prelude::*, rgb};

pub fn target_element(current_word: &str, progress: usize) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .size_full()
        .items_center()
        .justify_center()
        .bg(rgb(0xfafafa))
        .children(
            // ループを使う
            current_word.chars().enumerate().map(|(i, c)| {
                let color = if i < progress {
                    rgb(0xbbbbbb)
                } else {
                    rgb(0x333333)
                };
                div().text_color(color).child(c.to_string())
            }),
        )
}
