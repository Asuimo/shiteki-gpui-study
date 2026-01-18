use gpui::{Entity, IntoElement, MouseButton, div, prelude::*, px, rgb};

use crate::models::timer::TimerModel;

pub fn reload_element(time_ticket: Entity<TimerModel>) -> impl IntoElement {
    let time_ticket = time_ticket.clone();
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
        .on_mouse_down(
            MouseButton::Left,
            //
            move |_event, _window, app| {
                time_ticket.update(app, |model, cx| {
                    println!("reload!");
                    model.reload(cx);
                })
            },
        )
}
