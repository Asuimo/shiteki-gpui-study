use gpui::{Entity, IntoElement, MouseButton, div, prelude::*, px, rgb};

use crate::models::timer::TimerModel;

pub fn quick_add_element(mut amount: u32, time_ticket: Entity<TimerModel>) -> impl IntoElement {
    if amount > 995959 {
        amount = 995959;
    }

    let mut hours = (amount / 10000) as u8;
    let mut minutes = ((amount % 10000) / 100) as u8;
    let mut seconds = (amount % 100) as u8;

    minutes += (seconds > 59) as u8;
    seconds %= 60;
    hours += (minutes > 59) as u8;
    minutes %= 60;

    let label = if hours > 0 {
        format!("+{}:{:02}:{:02}", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("+{}:{:02}", minutes, seconds)
    } else {
        format!("+0:{:02}", seconds)
    };

    div()
        .flex()
        .flex_row()
        .bg(rgb(0x4a5c4a))
        .text_color(rgb(0xd6e0d6))
        .rounded(px(4.0))
        .child(label)
        .on_mouse_down(MouseButton::Left, move |_event, _window, app| {
            time_ticket.update(app, |timer, cx| {
                timer.add(amount);
                cx.notify();
            });
        })
}
