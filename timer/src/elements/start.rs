use gpui::{Entity, IntoElement, MouseButton, div, prelude::*, px, rgb};

use crate::models::timer::{TimerModel, TimerStatus};

pub fn start_element(time: &TimerModel, timer_ticket: Entity<TimerModel>) -> impl IntoElement {
    div()
        .flex()
        .bg(rgb(0x4a5c4a))
        .size(px(40.0))
        .rounded(px(5.0))
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(rgb(0xd6e0d6))
        .child(if time.status == TimerStatus::Running {
            "‖"
        } else {
            "▶︎"
        })
        .on_mouse_down(
            MouseButton::Left,
            // impl Fn(&MouseDownEvent, &mut Window, &mut App)
            move |_event, _window, app| {
                // if is timer running, stop it else start it
                timer_ticket.update(app, |time, cx| {
                    if time.status == TimerStatus::Running {
                        time.stop();
                    } else {
                        time.start(cx);
                    }
                    // println!("{}", time.is_running);
                    cx.notify();
                })
            },
        )
}
