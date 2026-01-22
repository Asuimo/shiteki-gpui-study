use gpui::{Entity, IntoElement, MouseButton, div, prelude::*, px, rgb};

use crate::models::timer::{TimerModel, TimerStatus};

pub fn reload_element(
    time_model: &TimerModel,
    time_ticket: Entity<TimerModel>,
) -> Option<impl IntoElement> {
    if time_model.status == TimerStatus::Idle || time_model.raw_time_digits == 0 {
        return None;
    }

    let ticket = time_ticket.clone();

    Some(
        div()
            .flex()
            .bg(rgb(0x4a5c4a))
            .rounded(px(5.0))
            .size(px(40.0))
            .justify_center()
            .items_center()
            .text_color(rgb(0xd6e0d6))
            .cursor_pointer()
            .hover(|s| s.bg(rgb(0x404040)))
            .child("↩︎")
            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                ticket.update(cx, |model, cx| {
                    model.reset(cx);
                });
            }),
    )
}
