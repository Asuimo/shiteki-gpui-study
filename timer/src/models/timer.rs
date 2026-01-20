use gpui::{AsyncApp, Context, WeakEntity};
use std::time::Duration;

pub struct TimerModel {
    pub h: u8,
    pub m: u8,
    pub s: u8,
    // 表示時刻をの10進数として表現したもの。
    pub raw: u32,
    pub is_running: bool,
    _timer_task: Option<gpui::Task<()>>,
}

impl TimerModel {
    pub fn new() -> Self {
        TimerModel {
            h: 0,
            m: 0,
            s: 0,
            raw: 0,
            is_running: false,
            _timer_task: None,
        }
    }

    fn raw_to_time(raw: u32) -> (u8, u8, u8) {
        let h = (raw / 10000) as u8;
        let m = ((raw % 10000) / 100) as u8;
        let s = (raw % 100) as u8;
        (h, m, s)
    }

    fn time_to_raw(h: u8, m: u8, s: u8) -> u32 {
        h as u32 * 10000 + m as u32 * 100 + s as u32
    }

    pub fn reload(&mut self, cx: &mut Context<TimerModel>) {
        if self.is_running {
            self.stop();
        }

        (self.h, self.m, self.s) = Self::raw_to_time(self.raw);
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
                        if this.s > 0 && this.is_running {
                            this.s -= 1;
                            model_cx.notify();
                            true
                        } else if this.s == 0 && this.m > 0 {
                            this.s = 59;
                            this.m -= 1;
                            model_cx.notify();
                            true
                        } else if this.s == 0 && this.m == 0 && this.h > 0 {
                            this.s = 59;
                            this.m = 59;
                            this.h -= 1;
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
        // 処理のため一度10進数にする。
        let mut time_ten = self.h as u32 * 10000 + self.m as u32 * 100 + self.s as u32;

        if key == "backspace" {
            time_ten /= 10;
        } else {
            let digit: u32 = key.parse().unwrap();
            time_ten = time_ten * 10 + digit;
        }
        time_ten %= 1000000;
        self.raw = time_ten;
        // 10進数を元に戻す。
        (self.h, self.m, self.s) = Self::raw_to_time(time_ten);
        cx.notify();
    }

    pub fn start(&mut self, cx: &mut Context<TimerModel>) {
        self.m += self.s / 60;
        self.h += self.m / 60;
        self.m %= 60;
        self.s %= 60;
        if self.h > 99 {
            self.h = 99;
            self.m = 59;
            self.s = 59;
        }
        self.raw = Self::time_to_raw(self.h, self.m, self.s);

        self.down(cx);
        cx.notify();
        self.is_running = true;
    }
    pub fn stop(&mut self) {
        self.is_running = false;
        self._timer_task = None;
    }
}
