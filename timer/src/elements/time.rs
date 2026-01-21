use gpui::{IntoElement, div, prelude::*, px, rgb};

use crate::models::timer::TimerModel;

pub fn time_element(time: &TimerModel) -> impl IntoElement {
    let label = if time.is_running {
        if time.hours > 0 {
            format!("{}:{:02}:{:02}", time.hours, time.minutes, time.seconds)
        } else if time.minutes > 0 {
            format!("{}:{:02}", time.minutes, time.seconds)
        } else {
            format!("0:{:02}", time.seconds)
        }
    } else {
        format!("{:02}:{:02}:{:02}", time.hours, time.minutes, time.seconds)
    };

    div()
        .bg(rgb(0x202020))
        .rounded(px(5.0))
        .h(px(60.0))
        .w(px(200.0))
        .text_size(px(40.0))
        .text_center()
        .text_color(rgb(0xfafafa))
        .child(label)
}
