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

    pub fn play(&mut self, board: &mut GameBoard, key: String) {
        let new_head = self.get_next_head_position(key);
        if let Ok(old_head) = board.get_token(new_head) {
            self.move_head(new_head, old_head, board).unwrap();
            log!("Current head position: {:?}", self.head());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_should_create_snake_with_head_only() {
        let x = 3;
        let y = 100;
        let snake = Snake::new(x, y);
        assert_eq!(vec![(x/2, y/2)], snake.snake);
    }

    #[test]
    fn play_should_move_head_in_correct_direction() {
        let dimensions = (10, 10);
        let mut brd = GameBoard::new(dimensions);
        let mut snake = Snake::new(dimensions.0, dimensions.1);
        let moves = ["ArrowUp", "ArrowRight", "ArrowDown", "ArrowLeft"];
        let positions = [(5, 4), (6, 4), (6, 5), (5, 5)];
        for (input, expected) in moves.iter().zip(positions.iter()) {
            snake.play(&mut brd, input.to_string());
            assert_eq!(snake.snake, vec![*expected]);
        }
    }

    #[test]
    fn play_should_print_snake_on_board() {
        let dimensions = (10, 10);
        let mut brd = GameBoard::new(dimensions);
        let mut snake = Snake::new(dimensions.0, dimensions.1);
        
        snake.play(&mut brd, String::new());
        assert_eq!(brd.get_token((dimensions.0/2, dimensions.1/2)), Ok(SnakeFields::Head));
    }

    #[test]
    fn head_should_return_last_element_of_snake_body() {
        let dimensions = (10, 10);
        let mut snake = Snake::new(dimensions.0, dimensions.1);
        let heads = [(0, 0), (1, 0), (dimensions.0-1, 0), (6, 4), (1, dimensions.1-1)];
        
        for head in heads {
            snake.snake.push(head);
            assert_eq!(snake.head(), head);
        }
    }

    #[test]
    fn get_next_head_position_should_increment_head_properly() {
        let x = 5;
        let y = 5;
        let mut snake = Snake::new(x*2, y*2);
        let moves = ["ArrowUp", "ArrowRight", "ArrowDown", "ArrowLeft"];
        let positions = [(x, y-1), (x+1, y), (x, y+1), (x-1, y)];
        for (input, expected) in moves.iter().zip(positions.iter()) {
            assert_eq!(snake.get_next_head_position(input.to_string()), *expected);
        }
    }
}