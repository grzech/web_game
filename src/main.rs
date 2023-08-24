use leptos::*;
mod game_board;

use game_board::{GameBoard, Board, SnakeFields};

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
    let mut brd = GameBoard::new((20, 16));
    let test_data = [
        ((1,1), SnakeFields::Body),
        ((2,2), SnakeFields::Body),
        ((2,1), SnakeFields::Head),
        ((2,3), SnakeFields::Food)];
    for (coordinates, token) in test_data {
        assert_eq!(brd.get_token(coordinates), SnakeFields::Empty);
        brd.put_token(coordinates, token);
        assert_eq!(brd.get_token(coordinates), token);
    }
    brd.put_token((1,1), SnakeFields::Body);
    brd.put_token((10,10), SnakeFields::Head);
    brd.put_token((10,11), SnakeFields::Food);
    view! {
        cx,
        <Board board=&brd/>
    }
}