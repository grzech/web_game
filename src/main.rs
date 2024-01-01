use leptos::*;
mod game_board;
mod snake;

use game_board::{GameBoard, Board};
use std::time::Duration;
use snake::Snake;
use std::sync::{Arc, Mutex};

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
    let (brd, set_brd) = create_signal(cx, GameBoard::new((20, 20)));
    let (key, set_key) = create_signal(cx, String::default());
    let game = Arc::new(Mutex::new(Snake::new(20, 20)));

    window_event_listener(ev::keydown, move |ev| {
        set_key.set(ev.key());
    } );
    
    set_interval(move || {
            set_brd.update( |b| {
                    game.lock().unwrap().play(b, key.get());
                }    
            );
        }, Duration::new(1, 0));

    view! {
        cx,
        {move ||  { view! { cx, <Board board=&brd.get()/> } } }
    }
}