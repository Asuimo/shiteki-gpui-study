use gpui::{AsyncApp, Context, WeakEntity};

use std::time::Duration;

pub struct TimerModel {
    pub seconds: u16,
    pub input_time: u16,
    pub is_running: bool,
}

impl TimerModel {
    pub fn new() -> Self {
        TimerModel {
            seconds: 0,
            input_time: 0,
            is_running: false,
        }
    }

    pub fn reload(&mut self, cx: &mut Context<TimerModel>) {
        if self.is_running {
            return;
        }

        self.seconds = self.input_time;
        cx.notify();
    }

    pub fn down(&mut self, cx: &mut Context<TimerModel>) {
        cx.spawn(|we: WeakEntity<TimerModel>, acxr: &mut AsyncApp| {
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
                        } else {
                            true
                        }
                    });

                    match result {
                        Ok(true) => continue,
                        _ => break,
                    }
                }
            }
        })
        .detach();
    }

    pub fn input(&mut self, key: &str, cx: &mut Context<TimerModel>) {
        // 入力
        if self.is_running {
            return;
        }

        if key == "backspace" {
            self.input_time /= 10;
        } else {
            let digit: u16 = key.parse().unwrap();
            self.input_time = self.input_time * 10 + digit;
        }
        self.seconds = self.input_time;
        cx.notify();
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }
    pub fn stop(&mut self) {
        self.is_running = false;
    }
}
