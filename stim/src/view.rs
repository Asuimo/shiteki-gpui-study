use crate::model::{TimerModel, TimerStatus};
use gpui::{
    App, Canvas, Context, Entity, FocusHandle, Half, IntoElement, KeyDownEvent, MouseButton,
    PathBuilder, Render, Window, canvas, div, point, prelude::*, px, rgb,
};
use std::f32::consts::{FRAC_PI_2, PI};

pub struct TimerView {
    timer_ticket: Entity<TimerModel>,
    focus_handle: FocusHandle,
}

impl TimerView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let timer_ticket = cx.new(|_| TimerModel::new());
        Self {
            timer_ticket,
            focus_handle: cx.focus_handle(),
        }
    }

    fn toggle_button_element(
        &self,
        status: &TimerStatus,
        timer_ticket: Entity<TimerModel>,
    ) -> Option<impl IntoElement> {
        if matches!(status, TimerStatus::Finished) {
            return None;
        }
        let label = match status {
            TimerStatus::Idle | TimerStatus::Paused => "▶︎",
            TimerStatus::Running => "⏸",
            _ => unreachable!(),
        };
        Some(
            div()
                .flex()
                .justify_center()
                .items_center()
                .rounded(px(20.0))
                .h(px(40.))
                .w_full()
                .bg(rgb(0x4a5c4a))
                .text_color(rgb(0xd6e0d6))
                .child(label)
                .on_mouse_down(MouseButton::Left, move |_event, _window, app_cx| {
                    timer_ticket.update(app_cx, |timer_model, cx| match timer_model.status {
                        TimerStatus::Idle => {
                            timer_model.start(cx);
                        }
                        TimerStatus::Running => {
                            timer_model.pause(cx);
                        }
                        TimerStatus::Paused => {
                            timer_model.resume(cx);
                        }
                        TimerStatus::Finished => unreachable!(),
                    })
                }),
        )
    }

    fn reset_button_element(
        &self,
        status: &TimerStatus,
        timer_ticket: Entity<TimerModel>,
    ) -> Option<impl IntoElement> {
        if matches!(status, TimerStatus::Idle) {
            return None;
        }

        Some(
            div()
                .flex()
                .justify_center()
                .items_center()
                .rounded(px(20.0))
                .h(px(40.))
                .w_full()
                .bg(rgb(0x4a5c4a))
                .text_color(rgb(0xd6e0d6))
                .child("↩︎")
                .on_mouse_down(MouseButton::Left, move |_event, _window, app_cx| {
                    timer_ticket.update(app_cx, |timer_model, cx| timer_model.reset(cx))
                }),
        )
    }

    fn time_display_element(time: &TimerModel) -> impl IntoElement {
        let label = if time.status == TimerStatus::Running || time.status == TimerStatus::Finished {
            if time.display_hours > 0 {
                format!(
                    "{}:{:02}:{:02}",
                    time.display_hours, time.display_minutes, time.display_seconds
                )
            } else if time.display_minutes > 0 {
                format!("{}:{:02}", time.display_minutes, time.display_seconds)
            } else {
                format!("0:{:02}", time.display_seconds)
            }
        } else {
            format!(
                "{:02}:{:02}:{:02}",
                time.display_hours, time.display_minutes, time.display_seconds
            )
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(40.0))
            .text_center()
            .text_color(rgb(0x1b2635))
            .child(label)
    }
    fn progress_circle_element(mut progress: f32) -> Canvas<()> {
        if progress <= 0. {
            progress = 1.
        }
        canvas(
            |_, _, _| {},
            move |bounds, _, window, _app| {
                let center = bounds.center();
                let radius = bounds.size.width.half();
                let line_width = px(5.);
                let angle = -FRAC_PI_2 + 2.0 * PI * progress;
                let progress_point = point(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                );
                let start_point = point(center.x, center.y - radius);

                // 進捗度合いを表す濃い線

                // 残りを表す薄い線
                let mut builder = PathBuilder::stroke(line_width);
                builder.move_to(start_point);
                // 円を直接かけないので半円を2個重ねる。
                if progress >= 1.0 {
                    builder.arc_to(
                        point(radius, radius),
                        px(0.),
                        false,
                        true,
                        point(center.x, center.y + radius),
                    );
                    builder.arc_to(
                        point(radius, radius),
                        px(0.),
                        false,
                        true,
                        point(center.x, center.y - radius),
                    );
                    builder.close();
                } else if progress > 0.0 {
                    builder.arc_to(
                        point(radius, radius),
                        px(0.),
                        // 長い弧をとるかどうか
                        progress > 0.5,
                        true,
                        progress_point,
                    );
                }
                if let Ok(path) = builder.build() {
                    window.paint_path(path, rgb(0xb5c4b5));
                }
            },
        )
        .size_full()
    }

    fn key_handler(
        timer_ticket: Entity<TimerModel>,
    ) -> impl Fn(&KeyDownEvent, &mut Window, &mut App) {
        move |event, _window, app| {
            let key = event.keystroke.key.as_str();
            println!("Key pressed: {}", key);
            if ("0"..="9").contains(&key) {
                timer_ticket.update(app, |model, cx| {
                    if let Ok(digit) = key.parse::<u8>() {
                        model.push_digit(digit, cx);
                    }
                });
            } else if key == "backspace" {
                timer_ticket.update(app, |model, cx| {
                    model.pop_digit(cx);
                });
            } else if key == "enter" {
                timer_ticket.update(app, |timer_model, cx| match timer_model.status {
                    TimerStatus::Idle => timer_model.start(cx),
                    TimerStatus::Running => timer_model.pause(cx),
                    TimerStatus::Paused => timer_model.resume(cx),
                    TimerStatus::Finished => timer_model.reset(cx),
                })
            }
        }
    }
}

impl Render for TimerView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let timer_ticket = self.timer_ticket.clone();
        let timer_model = self.timer_ticket.read(cx);

        let background_color = match timer_model.status {
            TimerStatus::Idle => rgb(0xd6e0d6),
            TimerStatus::Running => rgb(0xd6e0d6),
            TimerStatus::Paused => rgb(0xd6dce0),
            TimerStatus::Finished => rgb(0xd76a1d),
        };

        if timer_model.status == TimerStatus::Running {
            cx.on_next_frame(window, move |_model, _window, cx| {
                cx.notify();
            });
        }
        div()
            .flex()
            .track_focus(&self.focus_handle)
            .flex_col()
            .gap_3()
            .justify_center()
            .items_center()
            .size_full()
            .bg(background_color)
            .p_5()
            .on_key_down(Self::key_handler(timer_ticket.clone()))
            .child(
                div()
                    .relative()
                    .size(px(200.))
                    .child(Self::progress_circle_element(
                        timer_model.gen_progress_ratio(),
                    ))
                    .child(
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .justify_center()
                            .items_center()
                            .child(Self::time_display_element(timer_model)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .w_full()
                    .gap(px(5.))
                    .when_some(
                        self.toggle_button_element(&timer_model.status, timer_ticket.clone()),
                        |this, button| this.child(button),
                    )
                    .when_some(
                        self.reset_button_element(&timer_model.status, timer_ticket.clone()),
                        |this, button| this.child(button),
                    ),
            )
    }
}
