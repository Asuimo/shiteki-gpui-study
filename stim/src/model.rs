use gpui::{AsyncApp, Context, Task, WeakEntity};
use std::time::{Duration, Instant};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TimerStatus {
    Idle,
    Running,
    Paused,
    Finished,
}

pub struct TimerModel {
    pub status: TimerStatus,
    pub display_hours: u8,
    pub display_minutes: u8,
    pub display_seconds: u8,
    pub time_digits: u32,
    pub start_instant: Option<Instant>,
    pub base_secs: f32,
    pub total_secs: f32,
    pub elapsed_secs: u32,
    pub total_elapsed_secs: f32,
    pub _timer_task: Option<Task<()>>,
}

impl TimerModel {
    pub fn new() -> Self {
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

    pub fn start(&mut self, cx: &mut Context<Self>) {
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

    pub fn pause(&mut self, cx: &mut Context<Self>) {
        self.status = TimerStatus::Paused;
        if let Some(start_time) = self.start_instant.take() {
            self.total_elapsed_secs += start_time.elapsed().as_secs_f32();
        }
        self._timer_task = None;
        cx.notify();
    }

    pub fn resume(&mut self, cx: &mut Context<Self>) {
        self.status = TimerStatus::Running;
        self.start_instant = Some(Instant::now());
        self.countdown_timer(cx);
        cx.notify();
    }

    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.status = TimerStatus::Idle;
        self._timer_task = None;
        self.elapsed_secs = 0;
        self.total_elapsed_secs = 0.;
        self.base_secs = 0.;
        self.start_instant = None;
        self.notify_digits_display(cx);
    }

    pub fn push_digit(&mut self, digit: u8, cx: &mut Context<Self>) {
        if self.status != TimerStatus::Idle {
            return;
        }
        self.time_digits = self.time_digits * 10 + digit as u32;
        self.time_digits %= 1000000;
        self.notify_digits_display(cx);
    }

    pub fn pop_digit(&mut self, cx: &mut Context<Self>) {
        if self.status != TimerStatus::Idle {
            return;
        }
        self.time_digits /= 10;
        self.notify_digits_display(cx);
    }

    pub fn normalize_digits(&mut self, cx: &mut Context<Self>) {
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

    pub fn notify_digits_display(&mut self, cx: &mut Context<Self>) {
        self.display_hours = (self.time_digits / 10000) as u8;
        self.display_minutes = ((self.time_digits % 10000) / 100) as u8;
        self.display_seconds = (self.time_digits % 100) as u8;
        cx.notify();
    }

    pub fn culc_hms_secs(&mut self) -> f32 {
        self.display_hours as f32 * 3600.0
            + self.display_minutes as f32 * 60.0
            + self.display_seconds as f32
    }

    pub fn gen_progress_ratio(&self) -> f32 {
        if self.status == TimerStatus::Idle {
            1.0
        } else {
            self.current_remaining() / self.base_secs
        }
    }

    pub fn current_remaining(&self) -> f32 {
        if let Some(start_time) = self.start_instant {
            let elapsed = start_time.elapsed().as_secs_f32() + self.total_elapsed_secs;
            (self.base_secs - elapsed).max(0.0)
        } else {
            (self.base_secs - self.total_elapsed_secs).max(0.0)
        }
    }

    pub fn play_finish_sound() {
        use rodio::{Decoder, OutputStream, Sink, source::*};
        use std::io::Cursor;

        let sound_data = include_bytes!("../assets/finish_sound.mp3");
        std::thread::spawn(move || {
            let Ok((_stream, stream_handle)) = OutputStream::try_default() else {
                return;
            };
            let Ok(sink) = Sink::try_new(&stream_handle) else {
                return;
            };

            let cursor = Cursor::new(sound_data);

            if let Ok(source) = Decoder::new(cursor) {
                sink.append(source);
            } else {
                eprintln!("デコードに失敗");
                let backup_source = SineWave::new(440.0)
                    .take_duration(Duration::from_millis(1500))
                    .amplify(0.15);
                sink.append(backup_source);
            }
            sink.sleep_until_end();
        });
    }

    pub fn consume_one_second(&mut self, cx: &mut Context<Self>) -> bool {
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

    pub fn countdown_timer(&mut self, cx: &mut Context<TimerModel>) {
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
