use gpui::{IntoElement, div, prelude::*, px, rgb};

use crate::models::timer::TimerModel;

pub fn time_element(time: &TimerModel) -> impl IntoElement {
    div()
        .bg(rgb(0x202020))
        .rounded(px(5.0))
        .h(px(60.0))
        .w(px(200.0))
        .text_xl()
        .text_center()
        .text_color(rgb(0xfafafa))
        .child(format!("{:02}:{:02}:{:02}", time.h, time.m, time.s))
}
