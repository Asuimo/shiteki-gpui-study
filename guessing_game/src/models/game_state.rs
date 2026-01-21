use gpui::Context;
use gpui::SharedString;
use rand::Rng;

pub struct GameState {
    secret_number: u32,
    pub current_input: SharedString,
    pub message: SharedString,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            secret_number: rand::rng().random_range(1..=99),
            current_input: String::new().into(),
            message: String::new().into(),
        }
    }
    pub fn reload(&mut self) {
        self.secret_number = rand::rng().random_range(1..=99);
        self.current_input = String::new().into();
        self.message = String::new().into();
    }
    pub fn sub_guess(&mut self, cx: &mut Context<Self>) {
        if let Ok(num) = self.current_input.parse::<u32>() {
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
