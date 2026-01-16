use crate::models::game_state::GameState;
use crate::views::game::GameView;
use gpui::{Entity, MouseButton};
use gpui::{div, prelude::*, rgb};

pub fn input_element(
    state_ticket: Entity<GameState>,
    cx: &mut Context<GameView>,
) -> impl IntoElement {
    let game = state_ticket.read(cx);
    let d10 = game.digit_10.clone();
    let d1 = game.digit_1.clone();
    let selected = game.selected.clone();

    div()
        .flex()
        .flex_row()
        .gap_2()
        .child({
            let state_ticket = state_ticket.clone();
            div()
                .size_12()
                .flex()
                .items_center()
                .justify_center()
                .bg(if selected == 0 {
                    rgb(0x444444)
                } else {
                    rgb(0x222222)
                })
                .border_2()
                .border_color(if selected == 0 {
                    rgb(0x007aff)
                } else {
                    rgb(0x555555)
                })
                .text_color(rgb(0xffffff))
                .child(d10)
                .on_mouse_down(MouseButton::Left, move |_event, _window, cx| {
                    state_ticket.update(cx, |game, model_cx| {
                        game.selected = 0;
                        model_cx.notify();
                    })
                })
        })
        .child({
            let state_ticket = state_ticket.clone();
            div()
                .size_12()
                .flex()
                .items_center()
                .justify_center()
                .bg(if selected == 1 {
                    rgb(0x444444)
                } else {
                    rgb(0x222222)
                })
                .border_2()
                .border_color(if selected == 1 {
                    rgb(0x007aff)
                } else {
                    rgb(0x555555)
                })
                .text_color(rgb(0xffffff))
                .child(d1)
                .on_mouse_down(MouseButton::Left, move |_event, _window, cx| {
                    state_ticket.update(cx, |game, model_cx| {
                        game.selected = 1;
                        model_cx.notify();
                    })
                })
        })
}
