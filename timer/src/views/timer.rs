use gpui::{AsyncApp, Entity, IntoElement, Render, Window};
use gpui::{div, prelude::*, rgb};

use std::time::Duration;

use crate::models::timer::TimerModel;

pub struct TimerView {
    time_ticket: Entity<TimerModel>,
}

impl Render for TimerView {
    fn render(&mut self, _: &mut Window, icx: &mut Context<TimerView>) -> impl IntoElement {
        let time_ticket = self.time_ticket.clone();
        let time = time_ticket.read(icx);
        div()
            .flex()
            .justify_center()
            .items_center()
            .size_full()
            .bg(rgb(0xf0f0f0))
            .child(format!("Time: {}", time.seconds))
    }
}

impl TimerView {
    pub fn new(icx: &mut Context<TimerView>) -> Self {
        let time_ticket = icx.new(|_| TimerModel::new());

        let mut view = Self { time_ticket };
        view.down(icx);
        view
    }

    pub fn down(&mut self, cx: &mut Context<TimerView>) {
        let time_ticket = self.time_ticket.clone();
        // spawn<AsyncFn, R>(&self, f: AsyncFn) -> Task<R>
        cx.spawn(|_view, ascx_ref: &mut AsyncApp| {
            let mut ascx = ascx_ref.clone();
            async move {
                loop {
                    println!("1");
                    ascx.background_executor()
                        .timer(Duration::from_secs(1))
                        .await;
                    println!("2");
                    let still_running =
                        time_ticket.update(&mut ascx, |model: &mut TimerModel, model_cx| {
                            if model.seconds > 0 {
                                println!("3");
                                model.seconds -= 1;
                                model_cx.notify();
                                true
                            } else {
                                false
                            }
                        });
                    println!("4");
                    match still_running {
                        Ok(true) => continue,
                        _ => break,
                    }
                }
            }
        })
        .detach();
    }
}
