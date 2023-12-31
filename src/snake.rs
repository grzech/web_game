use leptos::*;
use crate::game_board::{GameBoard, SnakeFields};

pub struct Snake {
    snake: Vec<(usize, usize)>,
    max_rows: usize,
    max_cols: usize,
}

impl Snake {
    pub fn new(rows: usize, cols: usize) -> Self {
        Snake {
            snake: vec![(rows/2, cols/2)],
            max_cols: cols,
            max_rows: rows
        }
    }

    fn head(&self) -> (usize, usize) {
        *self.snake.last().unwrap()
    }

    fn get_next_head_position(&mut self, key: String) -> (usize, usize) {
        let (x, y) = self.head();
        match key.as_str() {
            "ArrowUp" => (x, y-1),
            "ArrowDown" => (x, y+1),
            "ArrowLeft" => (x-1, y),
            "ArrowRight" => (x+1, y),
            _ => (x, y),
        }
    }

    fn check_boundaries(&self, (x, y): (usize, usize)) -> Result<(), ()> {
        if self.max_cols < x || self.max_rows < y {
            return Err(());
        }
        Ok(())
    }

    fn check_new_field(field: &SnakeFields) -> Result<(), ()> {
        match field {
            SnakeFields::Body => Err(()),
            _ => Ok(()),
        }
    }

    fn put_head(&mut self, new_head: (usize, usize), brd: &mut GameBoard, field: SnakeFields) {
        if new_head != self.head() {
            if self.snake.len() == 1 {
                brd.put_token(self.head(), SnakeFields::Empty).unwrap();
                self.snake.pop();
            } else {
                    brd.put_token(self.head(), SnakeFields::Body).unwrap();
                if field != SnakeFields::Food {
                    brd.put_token(self.snake.pop().unwrap(), SnakeFields::Empty).unwrap();
                }
            }
            brd.put_token(new_head, SnakeFields::Head).unwrap();
            self.snake.push(new_head);
        } else {
            brd.put_token(self.head(), SnakeFields::Head).unwrap();
        }
    }

    fn move_head(&mut self, new_head: (usize, usize), field: SnakeFields, brd: &mut GameBoard) -> Result<(), ()> {
        self.check_boundaries(new_head)?;
        Self::check_new_field(&field)?;
        self.put_head(new_head, brd, field);
        Ok(())
    }

    pub fn play(&mut self, board: WriteSignal<GameBoard>, key: String) {
        board.update(|b| {
            let new_head = self.get_next_head_position(key);
            if let Ok(old_head) = b.get_token(new_head) {
                self.move_head(new_head, old_head, b).unwrap();
                log!("Current head position: {:?}", self.head());
            }
        });
    }
}
