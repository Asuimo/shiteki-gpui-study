use gpui::{Entity, FocusHandle, IntoElement, Render, Window, div, prelude::*, rgb};

use crate::models::timer::TimerModel;

use crate::elements::{reload::reload_element, start::start_element, time::time_element};

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
            .bg(rgb(0xf0f0f0))
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
                            if !model.is_running {
                                model.start(cx);
                            }

                            cx.notify();
                        })
                    }
                }
            })
            .child(time_element(time))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().flex().child(""))
                    .child(start_element(time, time_ticket.clone()))
                    .child(reload_element(time_ticket)),
            )
    }
}

impl TimerView {
    pub fn new(icx: &mut Context<TimerView>) -> Self {
        let time_ticket = icx.new(|_| TimerModel::new());
        let focus_handle = icx.focus_handle();
        // 起動時にクリックなしでフォーカスさせる。

        Self {
            time_ticket,
            focus_handle,
        }

        // let mut view = Self { time_ticket };
        // view.down(icx);
        // view
    }

    // pub fn down(&mut self, cx: &mut Context<TimerView>) {
    //     let time_ticket = self.time_ticket.clone();
    //     // spawn<AsyncFn, R>(&self, f: AsyncFn) -> Task<R>
    //     cx.spawn(|_view, ascx_ref: &mut AsyncApp| {
    //         let mut ascx = ascx_ref.clone();
    //         async move {
    //             loop {
    //                 ascx.background_executor()
    //                     .timer(Duration::from_secs(1))
    //                     .await;
    //                 let still_running =
    //                     time_ticket.update(&mut ascx, |model: &mut TimerModel, model_cx| {
    //                         if model.seconds > 0 {
    //                             model.seconds -= 1;
    //                             model_cx.notify();
    //                             true
    //                         } else {
    //                             false
    //                         }
    //                     });
    //                 match still_running {
    //                     Ok(true) => continue,
    //                     _ => break,
    //                 }
    //             }
    //         }
    //     })
    //     .detach();
    // }
}
