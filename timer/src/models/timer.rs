use gpui::{AsyncApp, Context, WeakEntity};
use std::time::Duration;

pub struct TimerModel {
    // 表示、入力のためのhms、最終的にはここを動かす。
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    // 表示時刻のけたを10進数として表現したもの。表示には使わない。入力処理用。初期値保存用。
    pub raw_time_digits: u32,
    // アニメーション用の秒数
    // pausedの時のみstartを押すとき更新する。
    pub progress: f32,
    pub progress_as_secs: f32,
    // idleの時のみstartを押すとき更新する。
    pub total_seconds: u32,
    // 現在の状態
    pub status: TimerStatus,
    _timer_task: Option<gpui::Task<()>>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TimerStatus {
    Idle,
    Running,
    Paused,
    Finished,
}

impl TimerModel {
    pub fn new() -> Self {
        TimerModel {
            hours: 0,
            minutes: 0,
            seconds: 0,
            raw_time_digits: 0,
            progress: 1.,
            progress_as_secs: 0.,
            total_seconds: 0,
            status: TimerStatus::Idle,
            _timer_task: None,
        }
    }

    pub fn inter_run_timer(&mut self, cx: &mut Context<TimerModel>) {
        match self.status {
            TimerStatus::Idle => {
                self.start(cx);
            }
            TimerStatus::Paused => {
                self.resume(cx);
            }
            _ => {
                println!("どこかで間違った仕様をしている。(into_run_timer())");
                return;
            }
        }
    }

    pub fn inter_pause_timer(&mut self, cx: &mut Context<Self>) {
        match self.status {
            TimerStatus::Running => {
                self.pause(cx);
            }
            _ => {
                println!("どこかで間違った仕様をしている。(into_pause_timer())");
                return;
            }
        }
    }

    pub fn inter_reload(&mut self, cx: &mut Context<Self>) {
        match self.status {
            TimerStatus::Running => {
                self.pause(cx);
                self.reset(cx);
            }
            TimerStatus::Paused => {
                self.reset(cx);
            }
            TimerStatus::Finished => {
                self.reset(cx);
            }
            _ => {
                println!("どこかで間違った仕様をしている。(into_reload_timer())");
                return;
            }
        }
    }

    /// 10進数表記の時間numericからhmsに適応させる。60を越える値でも正規化を行わない。
    fn apply_digits(&mut self) {
        self.hours = (self.raw_time_digits / 10000) as u8;
        self.minutes = ((self.raw_time_digits % 10000) / 100) as u8;
        self.seconds = (self.raw_time_digits % 100) as u8;
    }

    /// hmsをそのまま10進数表記に変換する。
    fn sync_digits(&mut self) {
        self.raw_time_digits =
            self.hours as u32 * 10000 + self.minutes as u32 * 100 + self.seconds as u32;
    }

    /// hmsからcurrent_secondsを更新する。
    fn update_progress_from_hms(&mut self) {
        self.progress =
            (self.hours as f32 * 3600.0 + self.minutes as f32 * 60.0 + self.seconds as f32)
                / self.total_seconds as f32;
    }

    fn update_progress_as_secs(&mut self) {
        self.progress_as_secs = self.progress * self.total_seconds as f32;
    }

    fn init_total_seconds(&mut self) {
        self.total_seconds =
            self.hours as u32 * 3600 + self.minutes as u32 * 60 + self.seconds as u32;
    }

    pub fn run_count_down(&mut self, cx: &mut Context<TimerModel>) {
        self._timer_task = Some(
            cx.spawn(|we: WeakEntity<TimerModel>, cx_ref: &mut AsyncApp| {
                let mut cx = cx_ref.clone();
                async move {
                    loop {
                        cx.background_executor().timer(Duration::from_secs(1)).await;
                        let result = we.update(&mut cx, |this, model_cx| {
                            if this.seconds > 0 && this.status == TimerStatus::Running {
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
            }),
        );
    }

    // 停止中のみ入力可能。
    pub fn input(&mut self, key: &str, cx: &mut Context<TimerModel>) {
        // 入力
        if self.status == TimerStatus::Running {
            return;
        }

        if key == "backspace" {
            self.raw_time_digits /= 10;
        } else {
            let digit: u32 = key.parse().unwrap();
            self.raw_time_digits = self.raw_time_digits * 10 + digit;
        }
        // 桁あふれを防ぐ。
        self.raw_time_digits %= 1000000;
        // 入力を反映させる。
        self.apply_digits();
        cx.notify();
    }

    pub fn start(&mut self, cx: &mut Context<TimerModel>) {
        self.normalize();
        self.init_total_seconds();
        if self.total_seconds == 0 {
            return;
        }
        self.status = TimerStatus::Running;
        self.run_count_down(cx);
        cx.notify();
    }

    pub fn resume(&mut self, cx: &mut Context<TimerModel>) {
        self.status = TimerStatus::Running;
        self.run_count_down(cx);
        cx.notify();
    }

    // 固定値をrawに追加する。running中は呼ばない。
    pub fn add(&mut self, add_time: u32) {
        self.raw_time_digits += add_time;
        self.apply_digits();
        self.normalize();
    }

    pub fn pause(&mut self) {
        self.status = TimerStatus::Paused;
        self._timer_task = None;
        self.update_progress_as_secs();
    }

    pub fn stop(&mut self) {
        self.status = TimerStatus::Paused;
        self._timer_task = None;
        self.update_progress_from_hms();
    }

    // Idleに戻して時間を前のIdleの時の値に戻す。
    pub fn reset(&mut self, cx: &mut Context<TimerModel>) {
        if self.status == TimerStatus::Running {
            self.stop();
        }
        self.status = TimerStatus::Idle;
        self.apply_digits();
        self.normalize();
        cx.notify();
    }

    // 12:59:99 とかを 13:00:39に直す。
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
    }
}
