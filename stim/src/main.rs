use gpui::{
    App, Application, AsyncApp, Bounds, Canvas, Context, Entity, FocusHandle, Half, IntoElement,
    KeyDownEvent, MouseButton, PathBuilder, WeakEntity, Window, WindowBounds, WindowOptions,
    canvas, div, point, prelude::*, px, rgb, size,
};

use rodio::source::SineWave;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::f32::consts::{FRAC_PI_2, PI};
use std::io::Cursor;
use std::time::{Duration, Instant};

struct TimerView {
    timer_ticket: Entity<TimerModel>,
    focus_handle: FocusHandle,
}

impl TimerView {
    fn new(cx: &mut Context<Self>) -> Self {
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
            .bg(rgb(0xd6e0d6))
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

struct TimerModel {
    status: TimerStatus,
    display_hours: u8,
    display_minutes: u8,
    display_seconds: u8,
    time_digits: u32,
    start_instant: Option<Instant>,
    base_secs: f32,
    total_secs: f32,
    elapsed_secs: u32,
    total_elapsed_secs: f32,
    _timer_task: Option<gpui::Task<()>>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum TimerStatus {
    Idle,
    Running,
    Paused,
    Finished,
}

impl TimerModel {
    fn new() -> Self {
        TimerModel {
            status: TimerStatus::Idle,
            display_hours: 0,
            display_minutes: 0,
            display_seconds: 0,
            time_digits: 0,
            start_instant: None,
            base_secs: 0.,
            total_secs: 0.,
            elapsed_secs: 0,
            total_elapsed_secs: 0.,
            _timer_task: None,
        }
    }

    fn start(&mut self, cx: &mut Context<Self>) {
        self.normalize_digits(cx);
        self.base_secs = self.culc_hms_secs();
        if self.base_secs <= 0. {
            return;
        }
        self.total_secs = self.base_secs;
        self.status = TimerStatus::Running;
        self.start_instant = Some(Instant::now());
        self.countdown_timer(cx);
        cx.notify();
    }

    fn pause(&mut self, cx: &mut Context<Self>) {
        self.status = TimerStatus::Paused;
        if let Some(start_time) = self.start_instant.take() {
            self.total_elapsed_secs += start_time.elapsed().as_secs_f32();
        }
        self._timer_task = None;
        cx.notify();
    }

    fn resume(&mut self, cx: &mut Context<Self>) {
        self.status = TimerStatus::Running;
        self.start_instant = Some(Instant::now());
        self.countdown_timer(cx);
        cx.notify();
    }

    fn reset(&mut self, cx: &mut Context<Self>) {
        self.status = TimerStatus::Idle;
        self._timer_task = None;
        self.elapsed_secs = 0;
        self.total_elapsed_secs = 0.;
        self.base_secs = 0.;
        self.start_instant = None;
        self.notify_digits_display(cx);
    }

    fn push_digit(&mut self, digit: u8, cx: &mut Context<Self>) {
        if self.status != TimerStatus::Idle {
            return;
        }
        self.time_digits = self.time_digits * 10 + digit as u32;
        self.time_digits %= 1000000;
        self.notify_digits_display(cx);
    }

    fn pop_digit(&mut self, cx: &mut Context<Self>) {
        if self.status != TimerStatus::Idle {
            return;
        }
        self.time_digits /= 10;
        self.notify_digits_display(cx);
    }

    fn normalize_digits(&mut self, cx: &mut Context<Self>) {
        if self.display_seconds > 59 {
            self.display_seconds -= 60;
            self.display_minutes += 1;
        }
        if self.display_minutes > 59 {
            self.display_minutes -= 60;
            self.display_hours += 1;
        }
        self.display_hours = self.display_hours.min(99);
        cx.notify();
    }

    fn notify_digits_display(&mut self, cx: &mut Context<Self>) {
        self.display_hours = (self.time_digits / 10000) as u8;
        self.display_minutes = ((self.time_digits % 10000) / 100) as u8;
        self.display_seconds = (self.time_digits % 100) as u8;
        cx.notify();
    }

    fn culc_hms_secs(&mut self) -> f32 {
        self.display_hours as f32 * 3600.0
            + self.display_minutes as f32 * 60.0
            + self.display_seconds as f32
    }

    fn gen_progress_ratio(&self) -> f32 {
        if self.status == TimerStatus::Idle {
            1.0
        } else {
            self.current_remaining() / self.base_secs
        }
    }

    fn current_remaining(&self) -> f32 {
        if let Some(start_time) = self.start_instant {
            let elapsed = start_time.elapsed().as_secs_f32() + self.total_elapsed_secs;
            (self.base_secs - elapsed).max(0.0)
        } else {
            (self.base_secs - self.total_elapsed_secs).max(0.0)
        }
    }

    fn play_finish_sound() {
        std::thread::spawn(|| {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let source = rodio::source::SineWave::new(440.0)
                .take_duration(std::time::Duration::from_secs_f32(1.5))
                .amplify(0.2);
            sink.append(source);
            sink.sleep_until_end();
        });
    }

    fn consume_one_second(&mut self, cx: &mut Context<Self>) -> bool {
        if self.display_seconds > 0 {
            self.display_seconds -= 1;
        } else if self.display_minutes > 0 {
            self.display_minutes -= 1;
            self.display_seconds = 59;
        } else if self.display_hours > 0 {
            self.display_hours -= 1;
            self.display_minutes = 59;
            self.display_seconds = 59;
        } else {
            self.display_hours = 0;
            self.display_minutes = 0;
            self.display_seconds = 0;
            self.status = TimerStatus::Finished;
            cx.notify();
            return false;
        }
        cx.notify();
        true
    }

    fn countdown_timer(&mut self, cx: &mut Context<TimerModel>) {
        if self._timer_task.is_some() {
            return;
        }
        self._timer_task = Some(
            cx.spawn(|we: WeakEntity<TimerModel>, cx_ref: &mut AsyncApp| {
                let mut cx = cx_ref.clone();
                async move {
                    loop {
                        cx.background_executor()
                            .timer(Duration::from_millis(100))
                            .await;
                        let shoud_stop = we
                            .update(&mut cx, |this, model_cx| {
                                if let Some(start_time) = this.start_instant {
                                    let elapsed = start_time.elapsed().as_secs_f32()
                                        + this.total_elapsed_secs;
                                    if elapsed > (this.elapsed_secs + 1) as f32 {
                                        this.elapsed_secs += 1;
                                        this.consume_one_second(model_cx);
                                        println!("{}", elapsed);
                                        model_cx.notify();
                                    }
                                    let remaining = (this.base_secs - elapsed).max(0.0);

                                    if remaining <= 0.0 {
                                        this.status = TimerStatus::Finished;
                                        TimerModel::play_finish_sound();
                                        this._timer_task = None;
                                        model_cx.notify();
                                        return true;
                                    }
                                }
                                false
                            })
                            .unwrap_or(true);

                        if shoud_stop {
                            break;
                        }
                    }
                }
            }),
        )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(300.), px(300.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|view_cx| TimerView::new(view_cx)),
        )
        .unwrap();
    });
}
