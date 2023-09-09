use leptos::*;
mod game_board;

use game_board::{GameBoard, Board, SnakeFields};
use std::time::Duration;

fn main() {
    mount_to_body(|cx| view!{ cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (game_started, set_game_started) = create_signal(cx, false);
    let start_game = move |_ev| { set_game_started.set(true) };
    view! {
        cx,
        <div id="Web Game">
            <h4>"Player name"</h4>
            <h3><input type="text"></input></h3>
            <h3><button on:click=start_game>"Start"</button></h3>
            <Show
                when= move || { game_started.get() }
                fallback=|_cx| view! { cx, <> }
            >
                <Game/>
            </Show>
        </div>
    }
    
}

#[component]
fn Game(cx: Scope) -> impl IntoView {
    let mut brd = GameBoard::new((20, 20));
    let (x, set_x) = create_signal(cx, 0usize);
    set_interval(move || {set_x.set(x.get()+1);}, Duration::new(1, 0));
    view! {
        cx,
        <Board board=&mut brd/>
        <h4>{move || format!("Current value is {}", x.get())}</h4>
    }
}