use leptos::*;
use crate::game_board::{GameBoard, SnakeFields};
use rand::Rng;

pub fn Snake(board: WriteSignal<GameBoard>) {
    board.update(|b| {
        let x = rand::thread_rng().gen_range(0..20);
        let y = rand::thread_rng().gen_range(0..20);
        b.put_token((x, y), SnakeFields::Body).unwrap();
    });
}
