use gpui::prelude::*;
use gpui::{Entity, FocusHandle, IntoElement, Render, Window, div, px, rgb};

use crate::models::timer::TimerModel;
use crate::models::timer::TimerStatus;

use crate::elements::{
    progress_circle::progress_circle_element, quick_add::quick_add_element, reload::reload_element,
    start::start_element, time::time_element,
};

pub struct TimerView {
    time_ticket: Entity<TimerModel>,
    focus_handle: FocusHandle,
}

impl Render for TimerView {
    fn render(&mut self, _: &mut Window, icx: &mut Context<TimerView>) -> impl IntoElement {
        let time_ticket = self.time_ticket.clone();
        let time = time_ticket.read(icx);
        div()
            .flex()
            .track_focus(&self.focus_handle)
            .flex_col()
            .gap_3()
            .justify_center()
            .items_center()
            .size_full()
            .bg(rgb(0xd6e0d6))
            .on_key_down({
                let timer_ticket = self.time_ticket.clone();
                move |event, _window, app| {
                    let key = event.keystroke.key.as_str();
                    println!("Key pressed: {}", key);
                    if ("0"..="9").contains(&key) || key == "backspace" {
                        timer_ticket.update(app, |model, cx| {
                            model.input(key, cx);
                        });
                    } else if key == "enter" {
                        timer_ticket.update(app, |model, cx| {
                            if model.status == TimerStatus::Paused
                                || model.status == TimerStatus::Idle
                            {
                                model.start(cx);
                            }
                            cx.notify();
                        })
                    }
                }
            })
            .child(
                div()
                    .relative()
                    .size(px(200.))
                    .child(progress_circle_element(0.3))
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(time_element(time)),
                    ),
            )
            // .child(progress_circle_element())
            // .child(time_element(time))
            .child(quick_add_element(30, time_ticket.clone()))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().child(start_element(time, time_ticket.clone())))
                    .child(
                        div().when_some(
                            reload_element(time, time_ticket.clone()),
                            |this, button| this.child(button),
                        ),
                    ),
            )
    }
}

impl TimerView {
    pub fn new(icx: &mut Context<TimerView>) -> Self {
        let time_ticket = icx.new(|_| TimerModel::new());
        let focus_handle = icx.focus_handle();
        Self {
            time_ticket,
            focus_handle,
        }
    }
}
