use gpui::{AsyncApp, Context, WeakEntity};

use std::time::Duration;

pub struct TimerModel {
    pub seconds: u32,
    pub is_running: bool,
}

impl TimerModel {
    pub fn new() -> Self {
        TimerModel {
            seconds: 10,
            is_running: false,
        }
    }
    pub fn down(&mut self, cx: &mut Context<TimerModel>) {
        cx.spawn(|we: WeakEntity<TimerModel>, acxr: &mut AsyncApp| {
            // 所有権を移すためクローン
            let mut acx = acxr.clone();
            // 非同期開始
            async move {
                loop {
                    acx.background_executor()
                        .timer(Duration::from_secs(1))
                        .await;
                    let result = we.update(&mut acx, |this, model_cx| {
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

    pub fn start(&mut self) {
        self.is_running = true;
    }
    pub fn stop(&mut self) {
        self.is_running = false;
    }
}
