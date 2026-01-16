use gpui::Context;

pub struct GameState {
    pub word_list: Vec<String>,
    pub current_word: String,
    pub progress: usize,
}

impl GameState {
    pub fn new() -> Self {
        let mut list = Self::new_targets();
        let current_word = list.pop().unwrap_or_default();
        GameState {
            word_list: list,
            current_word,
            progress: 0,
        }
    }

    pub fn new_targets() -> Vec<String> {
        vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ]
    }

    pub fn judge_input(&mut self, key: &str, state_cx: &mut Context<GameState>) {
        let Some(target_char) = self.current_word.chars().nth(self.progress) else {
            return;
        };
        if key == target_char.to_string() {
            println!("progress: {}", self.progress);
            self.progress += 1;
            if self.progress == self.current_word.len() {
                self.progress = 0;
                self.current_word = self.word_list.pop().unwrap_or_default();
            }
        }

        state_cx.notify();
    }
}
