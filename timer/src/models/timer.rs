use gpui::{AsyncApp, Context, WeakEntity};
use std::time::Duration;

pub struct TimerModel {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    // 表示時刻を10進数として表現したもの。表示には使わない。
    pub numeric_display: u32,
    pub is_running: bool,
    _timer_task: Option<gpui::Task<()>>,
}

impl TimerModel {
    pub fn new() -> Self {
        TimerModel {
            hours: 0,
            minutes: 5,
            seconds: 0,
            numeric_display: 0,
            is_running: false,
            _timer_task: None,
        }
    }

    fn raw_to_time(&mut self) {
        self.hours = (self.numeric_display / 10000) as u8;
        self.minutes = ((self.numeric_display % 10000) / 100) as u8;
        self.seconds = (self.numeric_display % 100) as u8;
    }

    fn time_to_raw(&mut self) {
        self.numeric_display =
            self.hours as u32 * 10000 + self.minutes as u32 * 100 + self.seconds as u32;
    }

    pub fn reload(&mut self, cx: &mut Context<TimerModel>) {
        if self.is_running {
            self.stop();
        }

        self.raw_to_time();
        cx.notify();
    }

    pub fn down(&mut self, cx: &mut Context<TimerModel>) {
        self._timer_task = Some(cx.spawn(|we: WeakEntity<TimerModel>, acxr: &mut AsyncApp| {
            // 所有権を移すためクローン
            let mut acx = acxr.clone();
            // 非同期開始
            async move {
                println!("startloop");
                loop {
                    println!("looping");
                    acx.background_executor()
                        .timer(Duration::from_secs(1))
                        .await;
                    let result = we.update(&mut acx, |this, model_cx| {
                        println!("is_running: {}", this.is_running);
                        if this.seconds > 0 && this.is_running {
                            this.seconds -= 1;
                            model_cx.notify();
                            true
                        } else if this.seconds == 0 && this.minutes > 0 {
                            this.seconds = 59;
                            this.minutes -= 1;
                            model_cx.notify();
                            true
                        } else if this.seconds == 0 && this.minutes == 0 && this.hours > 0 {
                            this.seconds = 59;
                            this.minutes = 59;
                            this.hours -= 1;
                            model_cx.notify();
                            true
                        } else {
                            false
                        }
                    });

                    match result {
                        Ok(true) => continue,
                        _ => break,
                    }
                }
            }
        }));
    }

    pub fn input(&mut self, key: &str, cx: &mut Context<TimerModel>) {
        // 入力
        if self.is_running {
            return;
        }

        if key == "backspace" {
            self.numeric_display /= 10;
        } else {
            let digit: u32 = key.parse().unwrap();
            self.numeric_display = self.numeric_display * 10 + digit;
        }
        self.numeric_display %= 1000000;

        // 10進数を元に戻す。
        self.raw_to_time();

        cx.notify();
    }

    pub fn start(&mut self, cx: &mut Context<TimerModel>) {
        // 12:59:99 とかを 13:00:39に直す。
        self.normalize();

        self.raw_to_time();

        self.down(cx);
        cx.notify();
        self.is_running = true;
    }

    pub fn add(&mut self, add_time: u32) {
        self.is_running = false;
        self.numeric_display += add_time;
        self.raw_to_time();
        self.normalize();
    }

    pub fn stop(&mut self) {
        self.is_running = false;
        self._timer_task = None;
    }

    fn normalize(&mut self) {
        self.minutes += (self.seconds >= 60) as u8;
        self.hours += (self.minutes >= 60) as u8;
        self.minutes %= 60;
        self.seconds %= 60;
        if self.hours > 99 {
            self.hours = 99;
            self.minutes = 59;
            self.seconds = 59;
        }
        self.time_to_raw();
    }
}
