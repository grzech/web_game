use leptos::*;
mod game_board;
mod snake;

use game_board::{GameBoard, Board, SnakeFields};
use std::time::Duration;
use snake::Snake;

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

fn handle_keyboard_input(ev: ev::KeyboardEvent, log: WriteSignal<String>)
{
    log.set(format!("Key received = {:?}({:?})", ev.char_code(), ev.code()));
}

#[component]
fn Game(cx: Scope) -> impl IntoView {
    let (brd, set_brd) = create_signal(cx, GameBoard::new((20, 20)));
    let (x, set_x) = create_signal(cx, 0usize);
    let (log, set_log) = create_signal(cx, String::default());
    window_event_listener(ev::keydown, move |ev| {
        handle_keyboard_input(ev, set_log)
    } );
    set_interval(move || {set_x.set(x.get()+1); Snake(set_brd); }, Duration::new(1, 0));
    view! {
        cx,
        {move ||  { view! { cx, <Board board=&brd.get()/> } } }
        <h4>{move || format!("{}", log.get()) }</h4>
    }
}