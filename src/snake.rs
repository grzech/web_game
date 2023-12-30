use leptos::*;
use crate::game_board::{GameBoard, SnakeFields};

pub struct Snake {
    snake: Vec<(usize, usize)>,
}

impl Snake {
    pub fn new(rows: usize, cols: usize) -> Self {
        Snake {snake: vec![(rows/2, cols/2)]}
    }

    

    pub fn play(&mut self, board: WriteSignal<GameBoard>, key: String) {
        board.update(|b| {
            let head = self.snake[0];
            let new_head = match key.as_str() {
                "ArrowUp" => (head.0, head.1-1),
                "ArrowDown" => (head.0, head.1+1),
                "ArrowLeft" => (head.0-1, head.1),
                "ArrowRight" => (head.0+1, head.1),
                _ => (head.0, head.1),
            };
            log!("Current key = \"{}\"", key);
            self.snake[0] = new_head;
            b.put_token(self.snake[0], SnakeFields::Head).unwrap();
        });
    }
}
