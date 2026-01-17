pub struct TimerModel {
    pub seconds: u32,
}

impl TimerModel {
    pub fn new() -> Self {
        TimerModel { seconds: 10 }
    }
}
