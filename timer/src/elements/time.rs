use gpui::{IntoElement, div, prelude::*, px, rgb};

use crate::models::timer::TimerModel;
use crate::models::timer::TimerStatus;

pub fn time_element(time: &TimerModel) -> impl IntoElement {
    let label = if time.status == TimerStatus::Running {
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
        .text_size(px(40.0))
        .text_center()
        .text_color(rgb(0x1b2635))
        .child(label)
}
