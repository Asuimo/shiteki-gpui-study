use crate::models::game_state::GameState;
use crate::views::game::GameView;

use gpui::Entity;
use gpui::{div, prelude::*, rgb};

pub fn message_element(
    state_ticket: Entity<GameState>,
    cx: &mut Context<GameView>,
) -> impl IntoElement {
    let state = state_ticket.read(cx);
    let current_message = state.message.clone();
    div()
        .bg(rgb(0x1e1e1e))
        .text_color(rgb(0xffffff))
        .child(current_message)
}
