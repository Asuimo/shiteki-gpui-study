use gpui::Context;
use gpui::SharedString;
use rand::Rng;

pub struct GameState {
    secret_number: u32,
    pub digit_10: SharedString,
    pub digit_1: SharedString,
    pub message: SharedString,
    pub selected: usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            secret_number: rand::rng().random_range(1..=99),
            digit_10: String::new().into(),
            digit_1: String::new().into(),
            message: String::new().into(),
            selected: 0,
        }
    }
    pub fn reload(&mut self) {
        self.secret_number = rand::rng().random_range(1..=99);
        self.digit_10 = String::new().into();
        self.digit_1 = String::new().into();
        self.message = String::new().into();
        self.selected = 0;
    }
    pub fn sub_guess(&mut self, cx: &mut Context<Self>) {
        let input_number = format!("{}{}", self.digit_10, self.digit_1);

        if let Ok(num) = input_number.parse::<u32>() {
            let result = self.guess(num);
            println!("Guess: {}", num);

            if result.is_true {
                self.message = "正解！".into();
            } else {
                self.message = if result.is_bigger {
                    "もっと小さい"
                } else {
                    "もっと大きい"
                }
                .into();
            }
        }
        cx.notify();
    }

    pub fn guess(&mut self, guess: u32) -> Answer {
        let is_true = guess == self.secret_number;
        let is_bigger = guess > self.secret_number;
        Answer { is_true, is_bigger }
    }
}

pub struct Answer {
    is_true: bool,
    is_bigger: bool,
}

impl Answer {
    pub fn new() -> Self {
        Answer {
            is_true: false,
            is_bigger: false,
        }
    }
}
