use gpui::{Entity, IntoElement, MouseButton, div, prelude::*, px, rgb};

use crate::models::timer::TimerModel;

pub fn start_element(time: &TimerModel, timer_ticket: Entity<TimerModel>) -> impl IntoElement {
    div()
        .flex()
        .bg(rgb(0x202020))
        .size(px(30.0))
        .rounded(px(5.0))
        .justify_center()
        .items_center()
        .text_xl()
        .text_color(rgb(0xefefef))
        .child(if time.is_running { "‖" } else { "▶︎" })
        .on_mouse_down(
            MouseButton::Left,
            // impl Fn(&MouseDownEvent, &mut Window, &mut App)
            move |_event, _window, app| {
                // if is timer running, stop it else start it
                timer_ticket.update(app, |time, cx| {
                    if time.is_running {
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
