use crate::elements::{
    guess::guess_element, input::input_element, message::message_element, reload::reload_element,
    title::title_element,
};
use crate::models::game_state::GameState;
use gpui::{Context, Entity, IntoElement, Render, Window};
use gpui::{div, prelude::*, rgb};

pub struct GameView {
    // Game stateに対する参照
    game_state: Entity<GameState>,
    focus_handle: gpui::FocusHandle,
}

impl GameView {
    pub fn new(view_cx: &mut Context<GameView>) -> Self {
        let game_state = view_cx.new(|_model_cx| GameState::new());
        let focus_handle = view_cx.focus_handle();
        GameView {
            game_state,
            focus_handle,
        }
    }
}

impl Render for GameView {
    fn render(
        &mut self,
        _window: &mut Window,
        game_cx: &mut Context<GameView>,
    ) -> impl IntoElement {
        let state_ticket = self.game_state.clone();
        div()
            .id("root")
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0x1e1e1e))
            .gap_4()
            .on_key_down({
                let state_ticket = state_ticket.clone();
                move |event, _window, cx| {
                    let key = event.keystroke.key.as_str();
                    println!("Key pressed: {}", key);
                    if ("0"..="9").contains(&key) {
                        state_ticket.update(cx, |game, model_cx| {
                            if game.selected == 0 {
                                game.digit_10 = key.to_string().into();
                            } else {
                                game.digit_1 = key.to_string().into();
                            }
                            model_cx.notify();
                        })
                    }
                }
            })
            .child(title_element())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(input_element(state_ticket.clone(), game_cx))
                    .child(guess_element(state_ticket.clone()))
                    .child(reload_element(state_ticket.clone())),
            )
            .child(message_element(state_ticket.clone(), game_cx))
    }
}
