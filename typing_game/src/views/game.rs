use gpui::*;
use gpui::{Context, Entity, FocusHandle, IntoElement, Render, Window};
use gpui::{div, rgb};

use crate::elements::target::target_element;
use crate::models::game_state::GameState;
pub struct GameView {
    state_ticket: Entity<GameState>,
    focus_handle: FocusHandle,
    scene: Scene,
}

pub enum Scene {
    Title,
    Game,
}

impl GameView {
    pub fn new(icx: &mut Context<GameView>) -> Self {
        let state_ticket = icx.new(|_cx| GameState::new());
        let focus_handle = icx.focus_handle();
        GameView {
            state_ticket,
            focus_handle,
            scene: Scene::Title,
        }
    }

    fn title_render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<GameView>,
    ) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_row()
            .size_full()
            .items_center()
            .justify_center()
            .text_size(px(24.0))
            .bg(rgb(0xfafafa))
            .on_key_down({
                let view_ticket = cx.entity().clone();
                move |event, _window, app_cx| {
                    let key = event.keystroke.key.as_str();
                    println!("Key : {}", key);
                    if key == "enter" {
                        view_ticket.update(app_cx, |view, cx| {
                            view.switch_game_scene(cx);
                        })
                    }
                }
            })
            .child("Type Game")
    }

    fn play_render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<GameView>,
    ) -> impl IntoElement {
        let state_ticket = self.state_ticket.clone();
        let state = state_ticket.read(cx);
        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_row()
            .size_full()
            .items_center()
            .justify_center()
            .bg(rgb(0xfafafa))
            .on_key_down({
                let state_ticket = state_ticket.clone();
                move |event, _window, app_cx| {
                    let key = event.keystroke.key.as_str();
                    println!("Key Input = {}", key);
                    state_ticket.update(
                        app_cx,
                        |state: &mut GameState, state_cx: &mut Context<GameState>| {
                            state.judge_input(key, state_cx);
                        },
                    );
                }
            })
            .child(target_element(&state.current_word, state.progress))
    }

    fn switch_game_scene(&mut self, cx: &mut Context<GameView>) {
        self.scene = Scene::Game;
        cx.notify();
    }
}

impl Render for GameView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<GameView>) -> impl IntoElement {
        match self.scene {
            Scene::Title => self.title_render(_window, cx).into_any_element(),
            Scene::Game => self.play_render(_window, cx).into_any_element(),
        }
    }
}
