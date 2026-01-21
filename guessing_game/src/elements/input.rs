use crate::models::game_state::GameState;
use crate::views::game::GameView;
use gpui::{Entity, MouseButton};
use gpui::{div, prelude::*, rgb};

pub fn input_element(
    state_ticket: Entity<GameState>,
    cx: &mut Context<GameView>,
) -> impl IntoElement {
    let game = state_ticket.read(cx);
    let current_input = game.current_input.clone();

    div()
        .size_24()
        .flex()
        .items_center()
        .justify_center()
        .bg(rgb(0x222222))
        .border_2()
        .border_color(rgb(0x007aff))
        .text_color(rgb(0xffffff))
        .child(current_input)
        .on_mouse_down(MouseButton::Left, move |_event, _window, cx| {
            state_ticket.update(cx, |game, model_cx| {
                game.current_input = "".into();
                model_cx.notify();
            })
        })
}
