use crate::models::game_state::GameState;
use gpui::{Entity, MouseButton};
use gpui::{div, prelude::*, rgb};

pub fn guess_element(state_ticket: Entity<GameState>) -> impl IntoElement {
    div()
        .flex()
        // .flex_row()
        .items_center()
        .bg(rgb(0xf2f0ee))
        .rounded_md()
        .text_color(rgb(0x5a3e44))
        .child("GUESS!")
        .on_mouse_down(
            MouseButton::Left,
            // Fn(&MouseDownEvent, &mut Window, &mut App)
            move |_event, _window, app_cx| {
                state_ticket.update(app_cx, |state, model_cx| {
                    println!("{}{}", state.digit_10, state.digit_1);
                    state.sub_guess(model_cx);
                })
            },
        )
}
