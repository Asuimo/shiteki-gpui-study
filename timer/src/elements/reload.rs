use gpui::{Entity, IntoElement, MouseButton, div, prelude::*, px, rgb};

use crate::models::timer::TimerModel;

pub fn reload_element(
    time: &TimerModel,
    time_ticket: Entity<TimerModel>,
) -> Option<impl IntoElement> {
    if !time.is_running || time.numeric_display == 0 {
        return None;
    }

    let ticket = time_ticket.clone();

    Some(
        div()
            .flex()
            .bg(rgb(0x303030))
            .rounded(px(5.0))
            .size(px(40.0))
            .justify_center()
            .items_center()
            .text_color(rgb(0xefefef))
            .cursor_pointer() // マウスを乗せた時に指アイコンにする
            .hover(|s| s.bg(rgb(0x404040))) // ホバー時の反応
            .child("↩︎")
            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                ticket.update(cx, |model, cx| {
                    model.reload(cx);
                });
            }),
    )
}
